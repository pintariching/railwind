use class::{Borders, Class, Decl, Spacing};
use indexmap::IndexMap;
use modifiers::{generate_state_selector, MediaQuery, State};
use utils::{indent_string, replace_invalid_chars};
use warning::{Position, Warning, WarningType};

use lazy_static::lazy_static;
use line_col::LineColLookup;
use regex::Regex;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

mod class;
mod modifiers;
mod utils;
pub mod warning;

lazy_static! {
    static ref CLASS_REGEX: Regex =
        Regex::new(r#"(?:class|className)=(?:["]\W+\s*(?:\w+)\()?["]([^"]+)["]"#).unwrap();
    static ref PREFLIGHT: &'static str = include_str!("../preflight.css");
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
        let selector_to_append = match &self.class {
            Class::Spacing(spacing) => match spacing {
                Spacing::SpaceBetween(_) => Some("> :not([hidden]) ~ :not([hidden])"),
                _ => None,
            },
            Class::Borders(borders) => match borders {
                Borders::DivideWidth(_) => Some("> :not([hidden]) ~ :not([hidden])"),
                _ => None,
            },
            _ => None,
        };

        let decl = self.class.to_decl()?;

        let mut class_selector = if self.states.iter().any(|s| match s {
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

        for state in self.states.iter() {
            match state {
                State::Group(g) => {
                    class_selector = format!("{}{}", g.to_static_str(), class_selector)
                }
                State::Peer(p) => {
                    class_selector = format!("{}{}", p.to_static_str(), class_selector)
                }
                _ => (),
            }
        }

        if let Some(to_append) = selector_to_append {
            class_selector = format!("{} {}", class_selector, to_append);
        }

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
        css_file.write_all(PREFLIGHT.as_bytes()).unwrap();
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
        css.push_str(&PREFLIGHT);
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
        css.push_str(&PREFLIGHT);
        css.push_str("\n\n");
    }

    css.push_str(&generated_classes.join("\n\n"));
    css.push_str("\n");

    css
}

fn collect_classes_from_html(html: &str) -> IndexMap<&str, Position> {
    let lookup = LineColLookup::new(html);

    let mut classes = IndexMap::new();

    for captures in CLASS_REGEX.captures_iter(html) {
        if let Some(group) = captures.get(1) {
            let mut index = group.start();

            for cap in group.as_str().split([' ', '\n']) {
                if cap.is_empty() || (cap == "group") || (cap == "peer") {
                    index += cap.len() + 1;
                    continue;
                }

                let position = lookup.get(index).into();
                classes.insert(cap, position);

                index += cap.len() + 1;
            }
        }
    }

    classes
}

fn collect_classes_from_str(text: &str) -> IndexMap<&str, Position> {
    let lookup = LineColLookup::new(text);
    let mut classes = IndexMap::new();
    let mut index = 0;

    for cap in text.split([' ', '\n']) {
        if cap.is_empty() {
            index += cap.len() + 1;
            continue;
        }

        let position = lookup.get(index).into();
        classes.insert(cap, position);

        index += cap.len() + 1;
    }

    classes
}

fn parse_raw_classes<'a>(
    raw_classes: IndexMap<&'a str, Position>,
    warnings: &mut Vec<Warning>,
) -> Vec<ParsedClass<'a>> {
    let mut parsed_classes = Vec::new();

    for (raw_class, position) in raw_classes {
        if let Some(colon_index) = raw_class.rfind(':') {
            // if an arbitrary value inside [...] contains a colon
            if let Some(left_bracket_index) = raw_class.rfind('[') {
                if left_bracket_index < colon_index {
                    if let Some(class) = Class::new(raw_class) {
                        parsed_classes.push(ParsedClass::new(raw_class, class, Vec::new()))
                    }
                }
            }

            let states = &raw_class[..colon_index];
            let entire_class = &raw_class[colon_index + 1..];

            let mut parsed_states = Vec::new();
            for state in states.split(':') {
                if let Some(s) = State::new(state) {
                    parsed_states.push(s)
                }
            }

            if let Some(class) = Class::new(entire_class) {
                parsed_classes.push(ParsedClass::new(raw_class, class, parsed_states))
            } else {
                let warning_type = WarningType::ClassNotFound;
                warnings.push(Warning::new(raw_class, &position, warning_type));
            }
        } else if let Some(class) = Class::new(raw_class) {
            parsed_classes.push(ParsedClass::new(raw_class, class, Vec::new()))
        } else {
            let warning_type = WarningType::ClassNotFound;
            warnings.push(Warning::new(raw_class, &position, warning_type));
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
            IndexMap::from([
                ("px-5", Position::new(1, 1)),
                ("justify-start", Position::new(1, 6)),
                ("container", Position::new(1, 20))
            ])
        );
    }

    #[test]
    fn test_collect_classes_from_html() {
        let text = r#"class="px-5 justify-start container""#;
        let classes = collect_classes_from_html(text);

        assert!(!classes.is_empty());
        assert_eq!(
            classes,
            IndexMap::from([
                ("px-5", Position::new(1, 8)),
                ("justify-start", Position::new(1, 13)),
                ("container", Position::new(1, 27))
            ])
        );
    }
}
