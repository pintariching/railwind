use std::{fs, io::Write, path::Path};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    input: String,

    #[arg(short, long, default_value = "railwind.css")]
    output: String,

    #[arg(short = 'p', long, default_value = "false")]
    include_preflight: bool,
}

fn main() {
    let args = Args::parse();

    let input = Path::new(&args.input);
    let output = Path::new(&args.output);

    let mut warnings = Vec::new();
    if let Some(extension) = input.extension() {
        match extension.to_str().unwrap() {
            "html" => {
                warnings = railwind::parse_html_to_file(input, output, args.include_preflight)
            }
            _ => {
                let file_string = fs::read_to_string(input).unwrap();
                let css =
                    railwind::parse_string(&file_string, args.include_preflight, &mut warnings);
                let mut file = fs::File::create(args.output).unwrap();
                file.write_all(css.as_bytes()).unwrap();
            }
        }
    }
    for warning in warnings {
        println!("{}", warning)
    }
}
