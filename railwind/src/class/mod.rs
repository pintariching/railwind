use crate::{
    class::{background::Background, spacing::MarginAndPadding},
    modifiers::{MediaQuery, PseudoClass, PseudoElement},
    utils::indent_string,
};

use self::layout::{AspectRatio, Container};

mod background;
// pub mod border;
// pub mod flex;
// pub mod grid;
mod layout;
mod spacing;

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
    pub fn from_str(raw_class: &'a str) -> Result<Self, String> {
        let (class, args) = Self::extract_class_and_args(raw_class);

        if class.is_empty() {
            return Err(format!("class shouldn't be empty: {}", raw_class));
        }

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

        Ok(seperated_class)
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

pub fn parse_class_from_str(str: &str) -> Result<String, String> {
    let class = SeperatedClass::from_str(str)?;

    let declarations = match class.class {
        "aspect" => AspectRatio::generate(&class)?,
        "bg" => Background::generate(&class)?,
        c => {
            let mut class_chars = c.chars();

            if let Some(first_char) = class_chars.next() {
                match first_char {
                    'p' | 'm' => MarginAndPadding::generate(&class)?,
                    first_char => {
                        if let Some(second_char) = class_chars.next() {
                            match second_char {
                                _ => return Err(format!("failed to parse class name: {}", str)),
                            }
                        } else {
                            return Err(format!("failed to parse class name: {}", str));
                        }
                    }
                }
            } else {
                return Err(format!("failed to parse class name: {}", str));
            }
        }
    };

    let mut generated_class = if !declarations.is_empty() {
        let selector = generate_class_selector(&class);
        let class = format!(
            ".{} {{\n    {};\n}}\n",
            selector,
            declarations.join(";\n    ")
        );
        class
    } else {
        match class.class {
            "container" => Container::generate_definitions(),
            _ => return Err(format!("unsupported class: {}", str)),
        }
    };

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

    println!("{}", generated_class);
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

pub trait MultiArgsDeclaration {
    fn generate_declaration(class: &str, args: &Vec<&str>) -> Result<Vec<String>, String>;
    fn generate(seperated_class: &SeperatedClass) -> Result<Vec<String>, String> {
        if let Some(args) = &seperated_class.args {
            return Self::generate_declaration(seperated_class.class, args);
        }

        return Err("class isn't formatted correctly, requires arguments after dash '-'".into());
    }
}

pub trait OneArgDeclarationWithDirection {
    fn generate_declaration(class: &str, arg: &str) -> Result<Vec<String>, String>;
    fn generate(seperated_class: &SeperatedClass) -> Result<Vec<String>, String> {
        if let Some(args) = &seperated_class.args {
            if args.len() != 1 {
                return Err(format!(
                    "invalid argument count, should be 1 but is {}",
                    args.len()
                ));
            }

            let arg = args.first().unwrap();

            return Self::generate_declaration(seperated_class.class, arg);
        }

        return Err("class isn't formatted correctly, requires arguments after dash '-'".into());
    }
}

pub trait OneArgDeclaration {
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
            SeperatedClass::from_str("container").unwrap(),
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
            SeperatedClass::from_str("p-5").unwrap(),
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
            SeperatedClass::from_str("overflow-x-hidden").unwrap(),
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
            SeperatedClass::from_str("sm:hover:bg-green-200").unwrap(),
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
            SeperatedClass::from_str("sm::bg--200").unwrap(),
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
