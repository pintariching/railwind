use std::path::Path;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "railwind.css")]
    output: String,
}

fn main() {
    let args = Args::parse();

    let input = Path::new(&args.input);
    let output = Path::new(&args.output);

    railwind::parse_html(input, output)
}
