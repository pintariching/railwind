use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use class::{Class, Decl};
use lazy_static::lazy_static;
use line_col::LineColLookup;
use modifiers::{generate_state_selector, MediaQuery, State};
use regex::Regex;
use utils::{indent_string, replace_invalid_chars};
use warning::{Position, Warning, WarningType};

mod class;
mod modifiers;
mod utils;
mod warning;

lazy_static! {
    static ref CLASS_REGEX: Regex =
        Regex::new(r#"(?:class|className)=(?:["']\W+\s*(?:\w+)\()?["']([^'"]+)['"]"#).unwrap();
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawClass<'a> {
    pub class: &'a str,
    pub position: Position,
}

#[derive(Debug)]
pub struct ParsedClass<'a> {
    pub raw_class_name: &'a str,
    pub class: Class<'a>,
    pub states: Vec<State>,
}

impl<'a> ParsedClass<'a> {
    pub fn new(raw_class_name: &'a str, class: Class<'a>, states: Vec<State>) -> Self {
        Self {
            raw_class_name,
            class,
            states,
        }
    }

    pub fn try_to_string(self) -> Option<String> {
        let decl = self.class.to_decl()?;

        let class_selector = if self.states.iter().any(|s| match s {
            State::PseudoClass(_) | State::PseudoElement(_) => true,
            _ => false,
        }) {
            format!(
                "{}:{}",
                replace_invalid_chars(self.raw_class_name),
                generate_state_selector(self.states.clone())
            )
        } else {
            replace_invalid_chars(self.raw_class_name)
        };

        let mut generated_class = match decl {
            Decl::FullClass(_) => decl.to_string().replace("container", &class_selector),
            _ => format!(".{} {{\n    {};\n}}", class_selector, decl.to_string()),
        };

        for state in self.states {
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

        Some(generated_class)
    }
}

pub fn parse_html_to_file(input: &Path, output: &Path, include_preflight: bool) -> Vec<Warning> {
    let html = fs::read_to_string(input).unwrap();
    let collected_classes = collect_classes_from_html(&html);

    let mut warnings = Vec::new();
    let parsed_classes = parse_raw_classes(collected_classes, &mut warnings);
    let generated_classes: Vec<String> = parsed_classes
        .into_iter()
        .filter_map(|c| c.try_to_string())
        .collect();

    let css = generated_classes.join("\n\n");

    let mut css_file = File::create(output).unwrap();

    if include_preflight {
        let preflight = fs::read_to_string("preflight.css").unwrap();
        css_file.write_all(preflight.as_bytes()).unwrap();
        css_file.write_all("\n\n".as_bytes()).unwrap();
    }

    css_file.write_all(css.as_bytes()).unwrap();
    css_file.write_all("\n".as_bytes()).unwrap();
    warnings
}

pub fn parse_html_to_string(
    input: &Path,
    include_preflight: bool,
    warnings: &mut Vec<Warning>,
) -> String {
    let html = fs::read_to_string(input).unwrap();
    let collected_classes = collect_classes_from_html(&html);

    let parsed_classes = parse_raw_classes(collected_classes, warnings);
    let generated_classes: Vec<String> = parsed_classes
        .into_iter()
        .filter_map(|c| c.try_to_string())
        .collect();

    let mut css = String::new();

    if include_preflight {
        let preflight = fs::read_to_string("preflight.css").unwrap();
        css.push_str(&preflight);
        css.push_str("\n\n");
    }

    css.push_str(&generated_classes.join("\n\n"));
    css.push_str("\n");

    css
}

pub fn parse_string(input: &str, include_preflight: bool, warnings: &mut Vec<Warning>) -> String {
    let collected_classes = collect_classes_from_str(input);
    let parsed_classes = parse_raw_classes(collected_classes, warnings);
    let generated_classes: Vec<String> = parsed_classes
        .into_iter()
        .filter_map(|c| c.try_to_string())
        .collect();

    let mut css = String::new();

    if include_preflight {
        let preflight = fs::read_to_string("preflight.css").unwrap();
        css.push_str(&preflight);
        css.push_str("\n\n");
    }

    css.push_str(&generated_classes.join("\n\n"));
    css.push_str("\n");

    css
}

fn collect_classes_from_html(html: &str) -> Vec<RawClass> {
    let lookup = LineColLookup::new(html);

    let mut classes = Vec::new();

    for captures in CLASS_REGEX.captures_iter(html) {
        if let Some(group) = captures.get(1) {
            let mut index = group.start();

            for cap in group.as_str().split([' ', '\n']) {
                if cap.is_empty() {
                    index += cap.len() + 1;
                    continue;
                }

                let position = lookup.get(index).into();
                classes.push(RawClass {
                    class: cap,
                    position,
                });

                index += cap.len() + 1;
            }
        }
    }

    classes
}

fn collect_classes_from_str(text: &str) -> Vec<RawClass> {
    let lookup = LineColLookup::new(text);
    let mut classes = Vec::new();
    let mut index = 0;

    for cap in text.split([' ', '\n']) {
        if cap.is_empty() {
            index += cap.len() + 1;
            continue;
        }

        let position = lookup.get(index).into();
        classes.push(RawClass {
            class: cap,
            position,
        });

        index += cap.len() + 1;
    }

    classes
}

fn parse_raw_classes<'a>(
    raw_classes: Vec<RawClass<'a>>,
    warnings: &mut Vec<Warning>,
) -> Vec<ParsedClass<'a>> {
    let mut parsed_classes = Vec::new();

    for raw_class in raw_classes {
        if let Some(colon_index) = raw_class.class.rfind(':') {
            let states = &raw_class.class[..colon_index];
            let entire_class = &raw_class.class[colon_index + 1..];

            let mut parsed_states = Vec::new();
            for state in states.split(':') {
                if let Some(s) = State::new(state) {
                    parsed_states.push(s)
                }
            }

            if let Some(class) = Class::new(entire_class) {
                parsed_classes.push(ParsedClass::new(raw_class.class, class, parsed_states))
            } else {
                let warning_type = WarningType::ClassNotFound;
                warnings.push(Warning::new(
                    raw_class.class,
                    &raw_class.position,
                    warning_type,
                ));
            }
        } else if let Some(class) = Class::new(raw_class.class) {
            parsed_classes.push(ParsedClass::new(raw_class.class, class, Vec::new()))
        } else {
            let warning_type = WarningType::ClassNotFound;
            warnings.push(Warning::new(
                raw_class.class,
                &raw_class.position,
                warning_type,
            ));
        }
    }

    parsed_classes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html() {
        let input = Path::new("../index.html");
        let output = Path::new("../railwind.css");

        let warnings = parse_html_to_file(input, output, false);

        for warning in warnings {
            println!("{}", warning);
        }
    }

    #[test]
    fn test_parse_raw_classes() {
        let text = "px-5 hover:lg:justify-start sm:container";
        let raw_classes = collect_classes_from_str(text);

        let mut warnings = Vec::new();
        let classes = parse_raw_classes(raw_classes, &mut warnings);

        for w in warnings {
            println!("{}", w);
        }

        for c in classes {
            println!("{}", c.try_to_string().unwrap());
        }
    }

    #[test]
    fn test_collect_classes_from_str() {
        let text = "px-5 justify-start container";
        let classes = collect_classes_from_str(text);

        assert!(!classes.is_empty());
        assert_eq!(
            classes,
            vec![
                RawClass {
                    class: "px-5",
                    position: Position::new(1, 1)
                },
                RawClass {
                    class: "justify-start",
                    position: Position::new(1, 6)
                },
                RawClass {
                    class: "container",
                    position: Position::new(1, 20)
                }
            ]
        );
    }

    #[test]
    fn test_collect_classes_from_html() {
        let text = r#"class="px-5 justify-start container""#;
        let classes = collect_classes_from_html(text);

        assert!(!classes.is_empty());
        assert_eq!(
            classes,
            vec![
                RawClass {
                    class: "px-5",
                    position: Position::new(1, 8)
                },
                RawClass {
                    class: "justify-start",
                    position: Position::new(1, 13)
                },
                RawClass {
                    class: "container",
                    position: Position::new(1, 27)
                }
            ]
        );
    }
}
