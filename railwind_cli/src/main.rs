use std::path::Path;

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

    let warnings = railwind::parse_html(input, output, args.include_preflight);

    for warning in warnings {
        println!("{}", warning)
    }
}
