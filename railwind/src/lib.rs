use class::parse_class;
use lazy_static::lazy_static;
use line_col::LineColLookup;
use modifiers::{generate_state_selector, MediaQuery, State};
use regex::Regex;
use std::fs::{self, File};
use std::{io::Write, path::Path};
use traits::ToStaticStr;
use utils::indent_string;
use warning::{Position, Warning};

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
    let mut generated_warnings = Vec::new();

    for captures in STYLE_REGEX.captures_iter(&html) {
        if let Some(group) = captures.get(1) {
            let mut index = group.start();

            for cap in group.as_str().split([' ', '\n']) {
                if cap.is_empty() {
                    index += cap.len() + 1;
                    continue;
                }

                let mut warning_types = Vec::new();

                // use linecol crate to get the current line and column
                let position: Position = lookup.get(index).into();

                // if the capture contains a colon ':' then split it and parse everything
                // before the colon as states (hover, after...) and everything after as a regular class
                // TODO: this could probably be improved as it contains a lot duplicated code from
                // the else block
                if let Some((states, raw_class)) = cap.rsplit_once(":") {
                    // if the raw_class starts with a '-', used by negative margins, then skip over the first character
                    let mut split_args = if raw_class.starts_with('-') {
                        raw_class[1..].split("-")
                    } else {
                        raw_class.split("-")
                    };

                    // get the first argument as the class name
                    let class_name = split_args.next().unwrap();

                    // collect the rest into an array
                    let args = [split_args.next(), split_args.next(), split_args.next()]
                        .map(|arg| if let Some(a) = arg { a } else { "" });

                    // parse all the states into a buffer
                    let mut states_buf = Vec::new();
                    for s in states.split(':') {
                        if let Some(state) = State::new(s, &mut warning_types) {
                            states_buf.push(state)
                        }
                    }

                    // parse class from class_name and args
                    if let Some(class) = parse_class(class_name, &args, &mut warning_types) {
                        // generate a CSS selector for the class
                        // ':' have to be replaced to be valid CSS
                        // and State::PseudoElement and State::PseudoClass go after the class name
                        // .hover:aspect-auto:hover becomes .hover\:aspect-auto:hover
                        let class_selector = if states_buf.iter().any(|s| match s {
                            State::PseudoClass(_) | State::PseudoElement(_) => true,
                            _ => false,
                        }) {
                            format!(
                                "{}:{}",
                                cap.replace(':', "\\:"),
                                generate_state_selector(&states_buf)
                            )
                        } else {
                            cap.replace(':', "\\:")
                        };

                        // generate the entire class with curly braces and new lines
                        let mut generated_class =
                            format!(".{} {{\n    {};\n}}", class_selector, class.join(";\n    "));

                        // if it's a media query, wrap the entire class with a
                        // pair of curly braces and indent the CSS one level
                        // TODO: setup matches for MediaQuery::Print, MediaQuery::Ltr, MediaQuery::Rtl
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
                                            "@media ({}) {{\n{}}}",
                                            mq.to_static_str(),
                                            indent_string(&generated_class)
                                        );
                                    }
                                    _ => (),
                                },
                                _ => (),
                            }
                        }

                        // push the generated class to a buffer
                        generated_classes.push_str(&generated_class);
                    }
                } else {
                    let mut split_args = if cap.starts_with('-') {
                        cap[1..].split("-")
                    } else {
                        cap.split("-")
                    };

                    // this feels wonky, if the class starts with a -, then it should
                    // be ignored in the split, but included in the class_name somehow
                    // TODO: improve this
                    let class_name = &if cap.starts_with('-') {
                        format!("-{}", split_args.next().unwrap())
                    } else {
                        split_args.next().unwrap().into()
                    };

                    let args = [split_args.next(), split_args.next(), split_args.next()]
                        .map(|arg| if let Some(a) = arg { a } else { "" });

                    if let Some(class) = parse_class(class_name, &args, &mut warning_types) {
                        let generated_class = format!(
                            ".{} {{\n    {};\n}}",
                            cap.replace(':', "\\:"),
                            class.join(";\n    ")
                        );

                        generated_classes.push_str(&generated_class);
                    }
                }

                for warning_type in warning_types {
                    generated_warnings.push(Warning::new(cap, &position, warning_type));
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

    generated_warnings
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
