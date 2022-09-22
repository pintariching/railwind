use lazy_static::lazy_static;
use regex::Regex;
use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};
use swc_common::Span;

use swc_css::ast::{Rule, Stylesheet};
use swc_css::codegen::writer::basic::{BasicCssWriter, BasicCssWriterConfig};
use swc_css::codegen::{CodeGenerator, CodegenConfig, Emit};

use crate::class::Class;

pub mod class;
pub mod modifiers;

lazy_static! {
    pub static ref STYLE_REGEX: Regex =
        Regex::new(r#"(?:class|className)=(?:["']\W+\s*(?:\w+)\()?["']([^'"]+)['"]"#).unwrap();
}

pub fn parse_html(input: &Path, output: &Path) {
    let html = fs::read_to_string(input).unwrap();

    let mut rules: Vec<Rule> = Vec::new();

    for capture in STYLE_REGEX.captures_iter(&html) {
        if let Some(group) = capture.get(1) {
            for cap in group.as_str().split(" ") {
                if let Some(parsed_class) = Class::parse_from_str(cap) {
                    rules.push(parsed_class.to_rule());
                }
            }
        }
    }

    let ast = Stylesheet {
        span: Span::default(),
        rules: rules,
    };

    let mut css_str = String::new();
    let writer = BasicCssWriter::new(&mut css_str, None, BasicCssWriterConfig::default());
    let mut generator = CodeGenerator::new(writer, CodegenConfig { minify: false });

    generator.emit(&ast).unwrap();

    let mut css_file = File::create(output).unwrap();
    css_file.write_all(css_str.as_bytes()).unwrap();
}
