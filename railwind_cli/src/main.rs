use clap::Parser;
use config::Config;
use railwind::{parse_to_file, CollectionOptions, Source, SourceOptions};
use std::path::{Path, PathBuf};

mod config;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: String,

    #[arg(short, long, default_value = "railwind.css")]
    output: String,

    #[arg(short = 'p', long, default_value = "false")]
    include_preflight: bool,

    #[arg(short = 'c', long, default_value = "railwind.config.ron")]
    config: PathBuf,
}

fn main() {
    let args = Args::parse();

    let input = Path::new(&args.input);
    let output = Path::new(&args.output);

    let mut warnings = Vec::new();

    if let Ok(config_file) = std::fs::read_to_string(args.config) {
        // if let Ok(config) = ron::from_str::<Config>(&config_file) {}
    }

    if input.is_dir() {
        todo!()
    }

    if input.is_file() {
        if let Some(extension) = input.extension() {
            match extension.to_str().unwrap() {
                "html" => {
                    _ = parse_to_file(
                        Source::File(SourceOptions {
                            input,
                            option: CollectionOptions::Html,
                        }),
                        Some(output),
                        args.include_preflight,
                        &mut warnings,
                    )
                }
                _ => {
                    _ = parse_to_file(
                        Source::File(SourceOptions {
                            input,
                            option: CollectionOptions::String,
                        }),
                        Some(output),
                        args.include_preflight,
                        &mut warnings,
                    )
                }
            }
        }
    }
    for warning in warnings {
        println!("{}", warning)
    }
}
