use class::{Borders, Class, Decl, Spacing};
use indexmap::IndexMap;
use modifiers::{generate_state_selector, MediaQuery, State};
use serde::{Deserialize, Serialize};
use utils::{indent_string, replace_invalid_chars};
use warning::{Position, Warning, WarningType};

use lazy_static::lazy_static;
use line_col::LineColLookup;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::PathBuf;

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
    pub position: Position,
}

impl<'a> ParsedClass<'a> {
    pub fn new(
        raw_class_name: &'a str,
        class: Class<'a>,
        states: Vec<State>,
        position: Position,
    ) -> Self {
        Self {
            raw_class_name,
            class,
            states,
            position,
        }
    }

    pub fn new_from_raw_class(raw_class: &'a str, position: Position) -> Result<Self, Warning> {
        if let Some(colon_index) = raw_class.rfind(':') {
            // if an arbitrary value inside [...] contains a colon
            if let Some(left_bracket_index) = raw_class.rfind('[') {
                if left_bracket_index < colon_index {
                    return Ok(Self::new(
                        raw_class,
                        Class::new(raw_class, raw_class, &position)?,
                        Vec::new(),
                        position.clone(),
                    ));
                }
            }

            let states = &raw_class[..colon_index];
            let entire_class = &raw_class[colon_index + 1..];

            let mut parsed_states = Vec::new();
            for state in states.split(':') {
                parsed_states.push(State::new(raw_class, state, &position)?)
            }

            Ok(Self::new(
                raw_class,
                Class::new(raw_class, entire_class, &position)?,
                parsed_states,
                position.clone(),
            ))
        } else {
            Ok(Self::new(
                raw_class,
                Class::new(raw_class, raw_class, &position)?,
                Vec::new(),
                position.clone(),
            ))
        }
    }

    pub fn try_to_string(self) -> Result<String, WarningType> {
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

        Ok(generated_class)
    }
}

pub struct SourceOptions<'a> {
    pub input: &'a PathBuf,
    pub option: CollectionOptions,
}

pub enum Source<'a> {
    File(SourceOptions<'a>),
    Files(Vec<SourceOptions<'a>>),
    String(String, CollectionOptions),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CollectionOptions {
    Html,
    String,
    #[serde(with = "serde_regex")]
    Regex(Regex),
}

impl CollectionOptions {
    pub fn new(value: &str, expand: Option<HashMap<String, CollectionOptions>>) -> Self {
        if let Some(exp) = expand {
            if let Some(opt) = exp.get(value) {
                return opt.clone();
            }
        }

        match value {
            "html" => CollectionOptions::Html,
            _ => CollectionOptions::String,
        }
    }
}

/// A convenience function to write a `Source` to a `railwind` CSS file
pub fn parse_to_file(
    source: Source,
    output: &str,
    include_preflight: bool,
    warnings: &mut Vec<Warning>,
) {
    let css = parse_to_string(source, include_preflight, warnings);

    let mut file = File::create(output).unwrap();
    file.write_all(css.as_bytes()).unwrap();
}

/// Parses a source to a `railwind` CSS string
pub fn parse_to_string(
    source: Source,
    include_preflight: bool,
    warnings: &mut Vec<Warning>,
) -> String {
    let mut css = if include_preflight {
        PREFLIGHT.to_string()
    } else {
        String::new()
    };

    match source {
        Source::File(opt) => {
            let file_string = read_to_string(opt.input).unwrap();
            let raw_classes: IndexMap<&str, Position> = match opt.option {
                CollectionOptions::Html => collect_with_regex(&file_string, &HTML_CLASS_REGEX),
                CollectionOptions::String => collect(&file_string),
                CollectionOptions::Regex(r) => collect_with_regex(&file_string, &r),
            };
            let parsed_classes = parse_classes(raw_classes, warnings);
            let generated_classes = generate_strings(parsed_classes, warnings);

            css.push_str(&generated_classes.join("\n\n"));
        }
        Source::Files(opts) => {
            let mut raw_string_classes: IndexMap<String, Position> = IndexMap::new();

            for opt in opts {
                let file_string = read_to_string(opt.input).unwrap();

                for (raw_str, position) in match opt.option {
                    CollectionOptions::Html => collect_with_regex(&file_string, &HTML_CLASS_REGEX),
                    CollectionOptions::String => collect(&file_string),
                    CollectionOptions::Regex(r) => collect_with_regex(&file_string, &r),
                } {
                    raw_string_classes.insert(raw_str.to_string(), position);
                }
            }

            let mut raw_classes: IndexMap<&str, Position> = IndexMap::new();
            for (c, p) in &raw_string_classes {
                raw_classes.insert(&c, p.clone());
            }

            let parsed_classes = parse_classes(raw_classes, warnings);
            let generated_classes = generate_strings(parsed_classes, warnings);

            css.push_str(&generated_classes.join("\n\n"));
        }
        Source::String(str, opt) => {
            let raw_classes: IndexMap<&str, Position> = match opt {
                CollectionOptions::Html => collect_with_regex(&str, &HTML_CLASS_REGEX),
                CollectionOptions::String => collect(&str),
                CollectionOptions::Regex(r) => collect_with_regex(&str, &r),
            };

            let parsed_classes = parse_classes(raw_classes, warnings);

            let generated_classes = generate_strings(parsed_classes, warnings);

            css.push_str(&generated_classes.join("\n\n"));
        }
    }

    css.push_str("\n");
    css
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
            match ParsedClass::new_from_raw_class(raw_str, position.clone()) {
                Ok(c) => Some(c),
                Err(w) => {
                    warnings.push(w);
                    None
                }
            }
        })
        .collect()
}

fn generate_strings<'a>(
    parsed_classes: Vec<ParsedClass<'a>>,
    warnings: &mut Vec<Warning>,
) -> Vec<String> {
    let mut out = Vec::new();

    for class in parsed_classes {
        let pos = class.position.clone();
        let raw = class.raw_class_name;

        match class.try_to_string() {
            Ok(c) => out.push(c),
            Err(w) => warnings.push(Warning::new(raw, &pos, w)),
        }
    }

    out
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
    fn test_parse_classes_fail() {
        let text = "space-c-4";
        let mut warnings = Vec::new();
        let _ = parse_to_string(
            Source::String(text.into(), CollectionOptions::String),
            false,
            &mut warnings,
        );

        assert!(!warnings.is_empty())
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

    #[test]
    fn test_collection_options() {
        let opts = CollectionOptions::new("html", None);

        match opts {
            CollectionOptions::Html => assert!(true),
            CollectionOptions::String => assert!(false),
            CollectionOptions::Regex(_) => assert!(false),
        }

        let opts =
            CollectionOptions::new("rs", Some(HashMap::from([("rs", CollectionOptions::Html)])));

        match opts {
            CollectionOptions::Html => assert!(true),
            CollectionOptions::String => assert!(false),
            CollectionOptions::Regex(_) => assert!(false),
        }
    }
}
