use class::Class;
use lazy_static::lazy_static;
use line_col::LineColLookup;
use modifiers::{generate_state_selector, MediaQuery, State};
use regex::Regex;
use std::fs::{self, File};
use std::str::FromStr;
use std::{io::Write, path::Path};
use traits::{IntoDeclaration, ToStaticStr};
use utils::indent_string;
use warning::Warning;

mod class;
mod config;
mod modifiers;
mod traits;
mod utils;
mod warning;

lazy_static! {
    static ref STYLE_REGEX: Regex =
        Regex::new(r#"(?:class|className)=(?:["']\W+\s*(?:\w+)\()?["']([^'"]+)['"]"#).unwrap();
}

pub fn parse_html(input: &Path, output: &Path, include_preflight: bool) -> Vec<Warning> {
    let html = fs::read_to_string(input).unwrap();
    let lookup = LineColLookup::new(&html);

    let mut generated_classes = String::new();
    let mut warnings = Vec::new();

    for captures in STYLE_REGEX.captures_iter(&html) {
        if let Some(group) = captures.get(1) {
            let mut index = group.start();

            for cap in group.as_str().split([' ', '\n']) {
                if cap.is_empty() {
                    index += cap.len() + 1;
                    continue;
                }

                let position = lookup.get(index).into();

                if let Some((states, raw_class)) = cap.rsplit_once(":") {
                    let mut split_args = if raw_class.starts_with('-') {
                        raw_class[1..].split("-")
                    } else {
                        raw_class.split("-")
                    };

                    let class_name = split_args.next().unwrap();
                    let args = [split_args.next(), split_args.next(), split_args.next()]
                        .map(|arg| if let Some(a) = arg { a } else { "" });

                    let class = match Class::new(class_name, &args) {
                        Ok(c) => c,
                        Err(e) => {
                            index += raw_class.len() + 1;
                            warnings.push(Warning::new(raw_class, &position, e));
                            continue;
                        }
                    };

                    let mut states_buf = Vec::new();
                    for s in states.split(':') {
                        match State::from_str(s) {
                            Ok(s) => states_buf.push(s),
                            Err(e) => {
                                index += cap.len() + 1;
                                Warning::new(cap, &position, e);
                            }
                        };
                    }

                    let class_selector = format!(
                        "{}:{}",
                        raw_class.replace(':', "\\:"),
                        generate_state_selector(&states_buf)
                    );

                    let mut generated_class = format!(
                        ".{} {{\n    {};\n}}",
                        class_selector,
                        class.into_decl().join(";\n    ")
                    );

                    for state in states_buf {
                        match state {
                            State::MediaQuery(mq) => match mq {
                                MediaQuery::Sm
                                | MediaQuery::Md
                                | MediaQuery::Lg
                                | MediaQuery::Xl
                                | MediaQuery::Xxl
                                | MediaQuery::Dark
                                | MediaQuery::MotionReduce
                                | MediaQuery::MotionSafe
                                | MediaQuery::ContrastMore
                                | MediaQuery::ContrastLess
                                | MediaQuery::Portrait
                                | MediaQuery::Landscape => {
                                    generated_class = format!(
                                        "@media ({}) {{\n{}}}\n",
                                        mq.to_static_str(),
                                        indent_string(&generated_class)
                                    );
                                }
                                _ => (),
                            },
                            _ => (),
                        }
                    }

                    generated_classes.push_str(&generated_class);
                } else {
                    let mut split_args = if cap.starts_with('-') {
                        cap[1..].split("-")
                    } else {
                        cap.split("-")
                    };

                    let class_name = split_args.next().unwrap();
                    let args = [split_args.next(), split_args.next(), split_args.next()]
                        .map(|arg| if let Some(a) = arg { a } else { "" });

                    let class = match Class::new(class_name, &args) {
                        Ok(c) => c,
                        Err(e) => {
                            index += cap.len() + 1;
                            warnings.push(Warning::new(cap, &position, e));
                            continue;
                        }
                    };

                    let generated_class = format!(
                        ".{} {{\n    {};\n}}",
                        cap.replace(':', "\\:"),
                        class.into_decl().join(";\n    ")
                    );

                    generated_classes.push_str(&generated_class);
                }

                index += cap.len() + 1;

                generated_classes.push('\n');
                generated_classes.push('\n');
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
