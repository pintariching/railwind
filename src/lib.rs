use class::{
    layout::{aspect_ratio::AspectRatio, container::Container},
    BaseClass,
};
use lazy_static::lazy_static;
use modifiers::{Modifier, PseudoClass};
use regex::Regex;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use crate::class::Class;

pub mod class;
pub mod modifiers;

lazy_static! {
    pub static ref STYLE_REGEX: Regex =
        Regex::new(r#"(?:class|className)=(?:["']\W+\s*(?:\w+)\()?["']([^'"]+)['"]"#).unwrap();
}

pub fn parse_html(input: &Path, output: &Path) -> Result<(), String> {
    let html = match fs::read_to_string(input) {
        Ok(h) => h,
        Err(e) => return Err(e.to_string()),
    };

    let mut classes: Vec<Class> = Vec::new();

    // get all the classes from "class" and "className"
    // attributes and puts them in a list
    for capture in STYLE_REGEX.captures_iter(&html) {
        if let Some(group) = capture.get(1) {
            for cap in group.as_str().split(" ") {
                if let Some(parsed_class) = Class::parse_from_str(cap) {
                    classes.push(parsed_class);
                }
            }
        }
    }

    println!("{:#?}", classes);

    let compiled_css = String::new();

    let mut file = match File::create(output) {
        Ok(f) => f,
        Err(e) => return Err(e.to_string()),
    };

    match file.write_all(compiled_css.as_bytes()) {
        Ok(_) => return Ok(()),
        Err(e) => return Err(e.to_string()),
    }
}
