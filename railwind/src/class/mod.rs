use crate::{
    modifiers::{MediaQuery, Modifier, PseudoClass, PseudoElement},
    utils::indent_string,
};

use self::layout::{AspectRatio, Container};

// pub mod background;
// pub mod border;
// pub mod flex;
// pub mod grid;
// pub mod spacing;
pub mod layout;

#[derive(Debug, PartialEq)]
pub struct SeperatedClass<'a> {
    pub class: &'a str,
    pub raw_class: &'a str,
    pub args: Option<Vec<&'a str>>,
    pub pseudo_classes: Option<Vec<PseudoClass>>,
    pub pseudo_elements: Option<Vec<PseudoElement>>,
    pub media_queries: Option<Vec<MediaQuery>>,
}

impl<'a> SeperatedClass<'a> {
    pub fn from_str(raw_class: &'a str) -> Self {
        let (class, args) = Self::extract_class_and_args(raw_class);
        let states = Self::extract_states(raw_class);

        let mut seperated_class = Self {
            class,
            raw_class,
            args,
            pseudo_classes: None,
            pseudo_elements: None,
            media_queries: None,
        };

        if let Some(states) = states {
            let mut pseudo_classes = Vec::new();
            let mut pseudo_elements = Vec::new();
            let mut media_queries = Vec::new();

            for state in states {
                if let Some(pseudo_class) = PseudoClass::parse_from_str(state) {
                    pseudo_classes.push(pseudo_class);
                } else if let Some(pseudo_element) = PseudoElement::parse_from_str(state) {
                    pseudo_elements.push(pseudo_element);
                } else if let Some(media_query) = MediaQuery::parse_from_str(state) {
                    media_queries.push(media_query);
                }
            }

            if !pseudo_classes.is_empty() {
                seperated_class.pseudo_classes = Some(pseudo_classes);
            }

            if !pseudo_elements.is_empty() {
                seperated_class.pseudo_elements = Some(pseudo_elements);
            }

            if !media_queries.is_empty() {
                seperated_class.media_queries = Some(media_queries);
            }
        }

        seperated_class
    }

    fn extract_states(raw_class: &'a str) -> Option<Vec<&'a str>> {
        // if raw_class contains ':' split at the last one
        if let Some((pseudo_elements, _)) = raw_class.rsplit_once(':') {
            // split at every ':' so that pseudo classes with '-' work too
            return Some(pseudo_elements.split(':').collect::<Vec<&'a str>>());
        }

        None
    }

    fn extract_class_and_args(raw_class: &'a str) -> (&'a str, Option<Vec<&'a str>>) {
        if let Some((_, class_with_values)) = raw_class.rsplit_once(':') {
            // if class contains '-' like py-5 or rounded-sm seperate by it
            // otherwise set class to str (rounded, container...)
            if class_with_values.contains('-') {
                let mut split = class_with_values.split("-");
                let class = split.next();
                let args = split.collect::<Vec<&str>>();

                if let Some(c) = class {
                    return (c, Some(args));
                }
            } else {
                return (class_with_values, None);
            }
        }

        if raw_class.contains('-') {
            let mut split = raw_class.split("-");

            // raw_class should contain a '-' so this should be safe to unwrap I think?
            let class = split.next().unwrap_or("");
            let args = split.collect::<Vec<&str>>();

            return (class, Some(args));
        } else {
            return (raw_class, None);
        }
    }
}

// fn generate_class(selector: &str, template: &str) -> String {
//     let modifiers = Modifier::parse_many_from_vec(selector);
//     let selector = generate_class_selector(selector, &modifiers);
//     let class_body = wrap_with_media_query(template, &modifiers);
//     class_body.replace("[class-selector]", &selector)
// }

pub fn parse_class_from_str(str: &str) -> Result<String, String> {
    let class = SeperatedClass::from_str(str);

    let declarations = match class.class {
        "aspect" => Some(AspectRatio::generate(&class)),
        _ => None,
    };

    let mut generated_class = if let Some(decls) = declarations {
        match decls {
            Ok(d) => {
                let selector = generate_class_selector(&class);
                let class = format!(".{} {{\n    {};\n}}\n", selector, d.join(";\n    "));
                class
            }
            Err(err) => return Err(err),
        }
    } else {
        match class.class {
            "container" => Container::generate_definitions(),
            _ => return Err(format!("unsupported class: {}", str)),
        }
    };

    println!("{}", generated_class);

    if let Some(media_queries) = class.media_queries {
        for query in media_queries {
            match query {
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
                        "@media ({}) {{\n{}}}\n",
                        query.as_str(),
                        indent_string(&generated_class)
                    );
                }
                _ => (),
            }
        }
    }

    Ok(generated_class)

    // let declarations: Vec<String> = match *class.first()? {
    //     "container" => todo!(), //
    //     _ => return None,
    // };
    // css needs to escape ":" with "\:"
    // let class_selector = str.replace(':', "\\:");

    // if class_selector.ends_with("container") {
    //     return Some(Container::parse_from_str(&class_selector));
    // }

    // if let Some(last_selector) = class_selector.split("\\:").last() {
    //     if last_selector.starts_with("aspect") {
    //         return AspectRatio::parse_from_str(&class_selector, last_selector);
    //     }

    //     if last_selector.starts_with("flex") {
    //         return Flex::parse_from_str(&class_selector, last_selector);
    //     }

    //     if last_selector.starts_with("border") || last_selector.starts_with("rounded") {
    //         return Border::parse_from_str(&class_selector, last_selector);
    //     }

    //     if last_selector.contains('-') {
    //         if last_selector.starts_with('p') {
    //             return Padding::parse_from_str(&class_selector, last_selector);
    //         }

    //         if last_selector.starts_with('m') {
    //             return Margin::parse_from_str(&class_selector, last_selector);
    //         }

    //         if last_selector.starts_with("bg") {
    //             return Background::parse_from_str(&class_selector, last_selector);
    //         }
    //     }
    // }
}

pub trait OneArgSingleDeclaration {
    fn generate_declaration(arg: &str) -> Result<Vec<String>, String>;

    fn generate(seperated_class: &SeperatedClass) -> Result<Vec<String>, String> {
        if let Some(args) = &seperated_class.args {
            if args.len() != 1 {
                return Err(format!(
                    "invalid argument count, should be 1 but is {}",
                    args.len()
                ));
            }

            let arg = args.first().unwrap();

            return Self::generate_declaration(arg);
        }

        return Err("class isn't formatted correctly, requires arguments after dash '-'".into());
    }
}

pub trait NoArgsMultiDefinition {
    fn generate_definitions() -> String;
}

fn generate_class_selector(seperated_class: &SeperatedClass) -> String {
    let mut result = seperated_class.raw_class.replace(':', "\\:");

    if let Some(pseudo_classes) = &seperated_class.pseudo_classes {
        let p = pseudo_classes
            .iter()
            .map(|p| p.as_str())
            .collect::<Vec<&str>>()
            .join(":");

        result.push(':');
        result.push_str(&p);
    };

    if let Some(pseudo_elements) = &seperated_class.pseudo_elements {
        let p = pseudo_elements
            .iter()
            .map(|p| p.as_str())
            .collect::<Vec<&str>>()
            .join("::");

        result.push_str("::");
        result.push_str(&p);
    };

    result
}

/// Splits a class selector by dash and returns the string
/// before the dash and converts the string after the dash into a CSS unit
///
/// For example `split_by_dash("py-5")` returns a tuple ("py", "1.25rem")
pub fn split_by_dash(str: &str) -> Option<(String, String)> {
    let mut split = str.split('-');
    let before_dash = split.next();
    let after_dash = split.next();

    if let (Some(before), Some(after)) = (before_dash, after_dash) {
        if before.is_empty() || after.is_empty() {
            return None;
        }

        if let Some((size, unit)) = convert_size(after) {
            let value = format!("{}{}", size, unit);
            return Some((before.into(), value));
        }

        return None;
    }

    None
}

pub fn convert_size(size: &str) -> Option<(f32, &'static str)> {
    let result = match size {
        "0" => (0., "px"),
        "px" => (1., "px"),
        "0.5" => (0.125, "rem"),
        "1" => (0.25, "rem"),
        "1.5" => (0.375, "rem"),
        "2" => (0.5, "rem"),
        "2.5" => (0.625, "rem"),
        "3" => (0.75, "rem"),
        "3.5" => (0.875, "rem"),
        "4" => (1., "rem"),
        "5" => (1.25, "rem"),
        "6" => (1.5, "rem"),
        "7" => (1.75, "rem"),
        "8" => (2., "rem"),
        "9" => (2.25, "rem"),
        "10" => (2.5, "rem"),
        "11" => (2.75, "rem"),
        "12" => (3., "rem"),
        "14" => (3.5, "rem"),
        "16" => (4., "rem"),
        "20" => (5., "rem"),
        "24" => (6., "rem"),
        "28" => (7., "rem"),
        "32" => (8., "rem"),
        "36" => (9., "rem"),
        "40" => (10., "rem"),
        "44" => (11., "rem"),
        "48" => (12., "rem"),
        "52" => (13., "rem"),
        "56" => (14., "rem"),
        "60" => (15., "rem"),
        "64" => (16., "rem"),
        "72" => (18., "rem"),
        "80" => (20., "rem"),
        "96" => (24., "rem"),
        _ => return None,
    };

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seperated_class_extract_class_and_args() {
        assert_eq!(
            SeperatedClass::extract_class_and_args("container"),
            ("container", None)
        );

        assert_eq!(
            SeperatedClass::extract_class_and_args("py-5"),
            ("py", Some(vec!["5"]))
        );

        assert_eq!(
            SeperatedClass::extract_class_and_args("box-decoration-clone"),
            ("box", Some(vec!["decoration", "clone"]))
        );

        assert_eq!(
            SeperatedClass::extract_class_and_args("grid-flow-row-dense"),
            ("grid", Some(vec!["flow", "row", "dense"]))
        );

        assert_eq!(
            SeperatedClass::extract_class_and_args("hover:bg-slate-500"),
            ("bg", Some(vec!["slate", "500"]))
        );

        assert_eq!(
            SeperatedClass::extract_class_and_args("-slate-500"),
            ("", Some(vec!["slate", "500"]))
        );
    }

    #[test]
    fn test_seperated_class_from_str() {
        assert_eq!(
            SeperatedClass::from_str("container"),
            SeperatedClass {
                class: "container",
                raw_class: "container",
                args: None,
                pseudo_classes: None,
                pseudo_elements: None,
                media_queries: None,
            }
        );

        assert_eq!(
            SeperatedClass::from_str("p-5"),
            SeperatedClass {
                class: "p",
                raw_class: "p-5",
                args: Some(vec!["5"]),
                pseudo_classes: None,
                pseudo_elements: None,
                media_queries: None,
            }
        );

        assert_eq!(
            SeperatedClass::from_str("overflow-x-hidden"),
            SeperatedClass {
                class: "overflow",
                raw_class: "overflow-x-hidden",
                args: Some(vec!["x", "hidden"]),
                pseudo_classes: None,
                pseudo_elements: None,
                media_queries: None,
            }
        );

        assert_eq!(
            SeperatedClass::from_str("sm:hover:bg-green-200"),
            SeperatedClass {
                class: "bg",
                raw_class: "sm:hover:bg-green-200",
                args: Some(vec!["green", "200"]),
                pseudo_classes: Some(vec![PseudoClass::Hover]),
                pseudo_elements: None,
                media_queries: Some(vec![MediaQuery::Sm]),
            }
        );

        assert_eq!(
            SeperatedClass::from_str("sm::bg--200"),
            SeperatedClass {
                class: "bg",
                raw_class: "sm::bg--200",
                args: Some(vec!["", "200"]),
                pseudo_classes: None,
                pseudo_elements: None,
                media_queries: Some(vec![MediaQuery::Sm]),
            }
        );
    }

    #[test]
    fn test_generate_class_selector() {
        let class = SeperatedClass {
            class: "container",
            raw_class: "container",
            args: None,
            pseudo_classes: None,
            pseudo_elements: None,
            media_queries: None,
        };

        assert_eq!(generate_class_selector(&class), "container".to_string());

        let class = SeperatedClass {
            class: "container",
            raw_class: "hover:container",
            args: None,
            pseudo_classes: Some(vec![PseudoClass::Hover]),
            pseudo_elements: None,
            media_queries: None,
        };

        assert_eq!(
            generate_class_selector(&class),
            "hover\\:container:hover".to_string()
        );

        let class = SeperatedClass {
            class: "container",
            raw_class: "hover:after:container",
            args: None,
            pseudo_classes: Some(vec![PseudoClass::Hover]),
            pseudo_elements: Some(vec![PseudoElement::After]),
            media_queries: None,
        };

        assert_eq!(
            generate_class_selector(&class),
            "hover\\:after\\:container:hover::after".to_string()
        );
    }
}
