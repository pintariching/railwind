use std::{
    fs::{self, File},
    io::Write,
};

use lazy_static::lazy_static;
use regex::Regex;

use railwindcss::class::{
    spacing::{Margin, Padding},
    Class,
};

lazy_static! {
    pub static ref STYLE_REGEX: Regex =
        Regex::new(r#"(?:class|className)=(?:["']\W+\s*(?:\w+)\()?["']([^'"]+)['"]"#).unwrap();
}

fn main() {
    let html = fs::read_to_string("index.html").unwrap();

    let style = STYLE_REGEX
        .captures_iter(&html)
        .nth(0)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();

    let classes = style.split(" ").collect::<Vec<&str>>();
    let mut output = String::new();

    for class in classes {
        let mut split = class.split("-");
        let before_dash = split.nth(0).unwrap();
        let after_dash = split.nth(0).unwrap();

        let class: Class = match before_dash.len() {
            1 => match before_dash {
                "m" => Class::Margin(Margin::new(before_dash, after_dash)),
                "p" => Class::Padding(Padding::new(before_dash, after_dash)),
                _ => todo!(),
            },
            2 => match before_dash.chars().nth(0).unwrap() {
                'm' => Class::Margin(Margin::new(before_dash, after_dash)),
                'p' => Class::Padding(Padding::new(before_dash, after_dash)),
                _ => todo!(),
            },
            _ => todo!(),
        };

        output.push_str(&class.to_css());
    }

    let mut file = File::create("railwind.css").unwrap();
    let _ = file.write_all(output.as_bytes());
}
