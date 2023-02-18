use clap::Parser;
use config::Config;
use notify::event::ModifyKind;
use notify::{Error, Event, EventKind, RecursiveMode, Watcher};
use railwind::{parse_to_string, CollectionOptions, Source, SourceOptions};
use ron::ser::PrettyConfig;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the output file
    #[arg(short, long, default_value = "railwind.css")]
    output: String,

    /// Include the Tailwind preflight in the output file
    #[arg(short = 'p', long, default_value = "false")]
    include_preflight: bool,

    /// Path to the config file
    #[arg(short = 'c', long, default_value = "railwind.config.ron")]
    config: String,

    /// Watch files for changes and automaticaly run the parser
    #[arg(short = 'w', long, default_value = "false")]
    watch: bool,

    /// Generate a default config file at the current directory
    #[arg(short = 'g', long, default_value = "false")]
    generate: bool,
}

fn main() {
    let args = Args::parse();

    if args.generate {
        let pretty = PrettyConfig::new().depth_limit(4);
        let config = ron::ser::to_string_pretty(&Config::default(), pretty).unwrap();
        let mut file = File::create("railwind.config.ron").unwrap();
        file.write_all(config.as_bytes()).unwrap();
        return;
    }

    let config = parse_config(&args.config);
    let input: Vec<PathBuf> = get_paths_from_config(&config);
    let output = Path::new(&args.output);

    if args.watch {
        let mut watcher = notify::recommended_watcher(|res: Result<Event, Error>| match res {
            Ok(event) => match event.kind {
                EventKind::Modify(m) => match m {
                    ModifyKind::Data(_) => {
                        println!("Running parser");
                        let start = Instant::now();

                        let args = Args::parse();
                        let config = parse_config(&args.config);
                        let input: Vec<PathBuf> = get_paths_from_config(&config);
                        let output = Path::new(&args.output);

                        run_parsing(&args, input, output, &config);

                        let duration = start.elapsed();
                        println!("Parsing took: {:?}", duration);
                    }
                    _ => (),
                },
                _ => (),
            },
            Err(e) => panic!("{}", e),
        })
        .unwrap();

        for watch_path in &input {
            watcher
                .watch(&watch_path, RecursiveMode::NonRecursive)
                .unwrap();
        }

        run_parsing(&args, input, output, &config);

        loop {}
    } else {
        run_parsing(&args, input, output, &config);
    }
}

fn parse_config(config_path: &str) -> Config {
    let config_file = fs::read_to_string(config_path).unwrap();

    match ron::from_str::<Config>(&config_file) {
        Ok(c) => c,
        Err(e) => {
            println!("Failed to parse config: {e}. Running with default config");
            Config {
                content: vec!["index.html".to_string()],
                extend_collection_options: None,
            }
        }
    }
}

fn get_paths_from_config(config: &Config) -> Vec<PathBuf> {
    let mut out_paths: Vec<PathBuf> = Vec::new();

    for c in config.content.clone() {
        if Path::new(&c).is_dir() {
            let dir = Path::new(&c);
            let paths = fs::read_dir(dir).unwrap();
            for file in paths {
                let f = file.unwrap();
                out_paths.push(f.path());
            }
        }
        for entry in globwalk::glob(&c).unwrap() {
            match entry {
                Ok(path) => out_paths.push(path.into_path()),
                Err(err) => panic!("{err}"),
            }
        }
    }

    out_paths
}

fn run_parsing(args: &Args, input: Vec<PathBuf>, output: &Path, config: &Config) {
    let mut warnings = Vec::new();

    let source_options: Vec<SourceOptions> = input
        .iter()
        .map(|i| SourceOptions {
            input: &i,
            option: if let Some(extension) = i.extension() {
                if let Some(str) = extension.to_str() {
                    CollectionOptions::new(str, config.extend_collection_options.clone())
                } else {
                    CollectionOptions::String
                }
            } else {
                CollectionOptions::String
            },
        })
        .collect();

    let css = parse_to_string(
        Source::Files(source_options),
        args.include_preflight,
        &mut warnings,
    );

    let mut css_file = File::create(output).unwrap();
    css_file.write_all(css.as_bytes()).unwrap();

    for warning in warnings {
        println!("{}", warning)
    }
}
