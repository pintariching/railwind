use class::parse_class_from_str;
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

pub mod class;
pub mod modifiers;
pub mod utils;

lazy_static! {
    pub static ref STYLE_REGEX: Regex =
        Regex::new(r#"(?:class|className)=(?:["']\W+\s*(?:\w+)\()?["']([^'"]+)['"]"#).unwrap();
}

pub fn parse_html(input: &Path, output: &Path) {
    let html = fs::read_to_string(input).unwrap();

    let mut classes = String::new();

    for capture in STYLE_REGEX.captures_iter(&html) {
        if let Some(group) = capture.get(1) {
            for cap in group.as_str().split(" ") {
                if let Some(parsed_class) = parse_class_from_str(cap) {
                    classes.push_str(&parsed_class);
                    classes.push('\n');
                }
            }
        }
    }

    let mut css_file = File::create(output).unwrap();
    css_file.write_all(classes.as_bytes()).unwrap();
}
