use class::{Borders, Class, Decl, Spacing};
use indexmap::IndexMap;
use modifiers::{generate_state_selector, MediaQuery, State};
use utils::{indent_string, replace_invalid_chars};
use warning::{Position, Warning, WarningType};

use lazy_static::lazy_static;
use line_col::LineColLookup;
use regex::Regex;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;

mod class;
mod modifiers;
mod utils;
pub mod warning;

lazy_static! {
    static ref HTML_CLASS_REGEX: Regex =
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

    pub fn new_from_raw_class(raw_class: &'a str, position: &Position) -> Result<Self, Warning> {
        if let Some(colon_index) = raw_class.rfind(':') {
            // if an arbitrary value inside [...] contains a colon
            if let Some(left_bracket_index) = raw_class.rfind('[') {
                if left_bracket_index < colon_index {
                    if let Some(class) = Class::new(raw_class) {
                        return Ok(Self::new(raw_class, class, Vec::new()));
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
                Ok(Self::new(raw_class, class, parsed_states))
            } else {
                Err(Warning::new(
                    raw_class,
                    &position,
                    WarningType::ClassNotFound,
                ))
            }
        } else if let Some(class) = Class::new(raw_class) {
            Ok(Self::new(raw_class, class, Vec::new()))
        } else {
            Err(Warning::new(
                raw_class,
                &position,
                WarningType::ClassNotFound,
            ))
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

pub struct SourceOptions<'a> {
    pub input: &'a Path,
    pub option: CollectionOptions<'a>,
}

pub enum Source<'a> {
    File(SourceOptions<'a>),
    Files(Vec<SourceOptions<'a>>),
    String(String, CollectionOptions<'a>),
}

pub enum CollectionOptions<'a> {
    Html,
    String,
    Regex(&'a Regex),
}

/// Parses a source to a `railwind` CSS file
pub fn parse_to_file(
    source: Source,
    output: Option<&Path>,
    include_preflight: bool,
    warnings: &mut Vec<Warning>,
) -> String {
    match source {
        Source::File(opt) => {
            let file_string = read_to_string(opt.input).unwrap();
            let raw_classes: IndexMap<&str, Position> = match opt.option {
                CollectionOptions::Html => collect_with_regex(&file_string, &HTML_CLASS_REGEX),
                CollectionOptions::String => collect(&file_string),
                CollectionOptions::Regex(r) => collect_with_regex(&file_string, r),
            };
            let parsed_classes = parse_classes(raw_classes, warnings);
            let generated_classes = generate_strings(parsed_classes);

            let mut css = generated_classes.join("\n\n");

            if let Some(output) = output {
                let mut css_file = File::create(output).unwrap();

                if include_preflight {
                    css_file.write_all(PREFLIGHT.as_bytes()).unwrap();
                    css_file.write_all("\n\n".as_bytes()).unwrap();
                }

                css_file.write_all(css.as_bytes()).unwrap();
                css_file.write_all("\n".as_bytes()).unwrap();
            }

            css.push('\n');
            css
        }
        Source::Files(opts) => {
            let mut raw_string_classes: IndexMap<String, Position> = IndexMap::new();

            for opt in opts {
                let file_string = read_to_string(opt.input).unwrap();

                for (raw_str, position) in match opt.option {
                    CollectionOptions::Html => collect_with_regex(&file_string, &HTML_CLASS_REGEX),
                    CollectionOptions::String => collect(&file_string),
                    CollectionOptions::Regex(r) => collect_with_regex(&file_string, r),
                } {
                    raw_string_classes.insert(raw_str.to_string(), position);
                }
            }

            let mut raw_classes: IndexMap<&str, Position> = IndexMap::new();
            for (c, p) in &raw_string_classes {
                raw_classes.insert(&c, p.clone());
            }

            let parsed_classes = parse_classes(raw_classes, warnings);
            let generated_classes = generate_strings(parsed_classes);

            let mut css = generated_classes.join("\n\n");

            if let Some(output) = output {
                let mut css_file = File::create(output).unwrap();

                if include_preflight {
                    css_file.write_all(PREFLIGHT.as_bytes()).unwrap();
                    css_file.write_all("\n\n".as_bytes()).unwrap();
                }

                css_file.write_all(css.as_bytes()).unwrap();
                css_file.write_all("\n".as_bytes()).unwrap();
            }

            css.push('\n');
            css
        }
        Source::String(str, opt) => {
            let raw_classes: IndexMap<&str, Position> = match opt {
                CollectionOptions::Html => collect_with_regex(&str, &HTML_CLASS_REGEX),
                CollectionOptions::String => collect(&str),
                CollectionOptions::Regex(r) => collect_with_regex(&str, r),
            };

            let parsed_classes = parse_classes(raw_classes, warnings);
            let generated_classes = generate_strings(parsed_classes);

            let mut css = generated_classes.join("\n\n");

            if let Some(output) = output {
                let mut css_file = File::create(output).unwrap();

                if include_preflight {
                    css_file.write_all(PREFLIGHT.as_bytes()).unwrap();
                    css_file.write_all("\n\n".as_bytes()).unwrap();
                }

                css_file.write_all(css.as_bytes()).unwrap();
                css_file.write_all("\n".as_bytes()).unwrap();
            }

            css.push('\n');
            css
        }
    }
}

fn collect_with_regex<'a>(str: &'a str, regex: &Regex) -> IndexMap<&'a str, Position> {
    let lookup = LineColLookup::new(str);

    let mut raw_classes = IndexMap::new();

    for captures in regex.captures_iter(str) {
        if let Some(group) = captures.get(1) {
            let mut index = group.start();

            for cap in group.as_str().split([' ', '\n']) {
                if cap.is_empty() || (cap == "group") || (cap == "peer") {
                    index += cap.len() + 1;
                    continue;
                }

                let position: Position = lookup.get(index).into();
                raw_classes.insert(cap, position);

                index += cap.len() + 1;
            }
        }
    }

    raw_classes
}

fn collect(str: &str) -> IndexMap<&str, Position> {
    let lookup = LineColLookup::new(str);
    let mut classes = IndexMap::new();
    let mut index = 0;

    for cap in str.split([' ', '\n']) {
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

fn parse_classes<'a>(
    raw_classes: IndexMap<&'a str, Position>,
    warnings: &mut Vec<Warning>,
) -> Vec<ParsedClass<'a>> {
    raw_classes
        .iter()
        .filter_map(|(raw_str, position)| {
            match ParsedClass::new_from_raw_class(raw_str, &position) {
                Ok(c) => Some(c),
                Err(w) => {
                    warnings.push(w);
                    None
                }
            }
        })
        .collect()
}

fn generate_strings<'a>(parsed_classes: Vec<ParsedClass<'a>>) -> Vec<String> {
    parsed_classes
        .into_iter()
        .filter_map(|c| c.try_to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_classes_from_str() {
        let text = "px-5 justify-start container";
        let classes = collect(text);

        assert!(!classes.is_empty());
        assert_eq!(
            classes,
            IndexMap::from([
                ("px-5", Position::new("", 1, 1)),
                ("justify-start", Position::new("", 1, 6)),
                ("container", Position::new("", 1, 20))
            ])
        );
    }

    #[test]
    fn test_collect_classes_from_html() {
        let text = r#"class="px-5 justify-start container""#;
        let classes = collect_with_regex(text, &HTML_CLASS_REGEX);

        assert!(!classes.is_empty());
        assert_eq!(
            classes,
            IndexMap::from([
                ("px-5", Position::new("", 1, 8)),
                ("justify-start", Position::new("", 1, 13)),
                ("container", Position::new("", 1, 27))
            ])
        );
    }
}
