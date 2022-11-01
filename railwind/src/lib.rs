use class::parse_class_from_str;
use std::fs::{self, File};
use std::{io::Write, path::Path};
use warning::Warning;

mod class;
mod config;
mod modifiers;
mod utils;
mod warning;

pub fn parse_html(input: &Path, output: &Path, include_preflight: bool) -> Vec<Warning> {
    let html = fs::read_to_string(input).unwrap();

    let mut generated_classes = String::new();
    let mut warnings = Vec::new();

    // line_index starts at 0, but editors usualy start counting lines with 1
    for (line_index, line) in html.lines().enumerate() {
        let line_index = line_index + 1;

        if let Some((before_split, after_split)) = line.split_once("class=") {
            let mut split = after_split.split(['"', '\'']);
            split.next(); // advance over the first " or '
            let raw_classes = split.next();

            if let Some(raw_class) = raw_classes {
                let mut column_index = before_split.len() + 8; // add 8 to account for class=" and an extra character
                for class in raw_class.split(' ') {
                    match parse_class_from_str(class) {
                        Ok(class) => {
                            generated_classes.push_str(&class);
                            generated_classes.push('\n');
                            generated_classes.push('\n');
                        }
                        Err(err) => warnings.push(Warning::new(&err, line_index, column_index)),
                    }

                    column_index += class.len() + 1
                }
            }
        }
    }

    // replaces a double newline with a single newline
    if generated_classes.ends_with("\n\n") {
        generated_classes = generated_classes.trim_end().to_string();
        generated_classes.push('\n');
    }

    let mut css_file = File::create(output).unwrap();

    if include_preflight {
        let preflight = fs::read_to_string("preflight.css").unwrap();
        css_file.write_all(preflight.as_bytes()).unwrap();
        css_file.write_all("\n\n".as_bytes()).unwrap();
    }

    css_file.write_all(generated_classes.as_bytes()).unwrap();

    warnings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html() {
        let input = Path::new("../index.html");
        let output = Path::new("../railwind.css");

        let warnings = parse_html(input, output, false);

        for warning in warnings {
            println!("{}", warning);
        }

        assert!(true)
    }
}
