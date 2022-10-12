use crate::modifiers::{generate_class_selector, wrap_with_media_query, Modifier};

use self::{
    layout::{AspectRatio, Container},
    spacing::{Margin, Padding},
};

pub mod layout;
pub mod spacing;

pub fn parse_class_from_str(str: &str) -> Option<String> {
    // css needs to escape ":" with "\:"
    let class_selector = str.replace(':', "\\:");

    if class_selector.ends_with("container") {
        return Some(Container::parse_from_str(&class_selector));
    }

    if let Some(last_selector) = class_selector.split("\\:").last() {
        if last_selector.starts_with("aspect") {
            return Some(AspectRatio::parse_from_str(&class_selector, last_selector));
        }

        if last_selector.contains("-") {
            if last_selector.starts_with("p") {
                return Some(Padding::parse_from_str(&class_selector, last_selector));
            }

            if last_selector.starts_with("m") {
                return Some(Margin::parse_from_str(&class_selector, last_selector));
            }
        }
    }

    None
}

pub fn convert_size(size: &str) -> (f32, &'static str) {
    match size {
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
        _ => (0., "px"),
    }
}

pub fn convert_breakpoint(breakpoint: &str) -> String {
    match breakpoint {
        "sm" => "640px",
        "md" => "786px",
        "lg" => "1024px",
        "xl" => "1280px",
        "2xl" => "1536px",
        _ => "1024px",
    }
    .to_string()
}

pub fn wrap_with_everything(
    class_body: &str,
    selector: &str,
    modifiers: &Option<Vec<Modifier>>,
) -> String {
    let selector = generate_class_selector(selector, modifiers);

    let class_body = wrap_with_media_query(class_body, modifiers);

    class_body.replace("[class-selector]", &selector)
}
