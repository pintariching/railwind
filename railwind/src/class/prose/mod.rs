use lazy_static::lazy_static;
use std::collections::HashMap;

use super::{utils::get_value, Decl};
use crate::{
    utils::{get_class_name, get_opt_args},
    warning::WarningType,
};

lazy_static! {
    static ref DEFAULT: &'static str = include_str!("default.css");
    static ref BASE: &'static str = include_str!("base.css");
    static ref GRAY: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("gray.ron")).unwrap();
    static ref COLORS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("../colors.ron")).unwrap();
}

/**
 * Implements the `prose`, `prose-<size>` and `prose-<color>` classes of
 * the Tailwind typography plugin (https://tailwindcss.com/docs/typography-plugin).
 *
 * CSS and RON files in this module were originally based on
 * https://github.com/tailwindlabs/tailwindcss-typography/blob/4c3b76ccc8120f307514d5721598763fd09761c0/src/styles.js
 */
#[derive(Debug)]
pub struct Prose<'a>(&'a str);

impl<'a> Prose<'a> {
    pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
        match get_class_name(value) {
            "prose" => Ok(Some(Self(get_opt_args(value)))),
            _ => Ok(None),
        }
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let mut css = DEFAULT.to_string();

        // Add vars for the gray scale
        let scale = match self.0 {
            //"neutral" => NEUTRAL,
            //"slate" => SLATE,
            //"stone" => STONE,
            //"zinc" => ZINC,
            _ => &GRAY,
        };
        css += ":root {\n";
        for (var, color) in scale.iter() {
            let color = get_value(&color, &COLORS)?;
            css += &format!("  {var}: {color};\n");
        }
        css += "}\n";

        // Add classes for the size
        css += match self.0 {
            //"sm" => &SM,
            //"lg" => &LG,
            //"xl" => &XL,
            //"2xl" => &XXL,
            _ => &BASE,
        };

        Ok(Decl::FullClass(css))
    }
}
