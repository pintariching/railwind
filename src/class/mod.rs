use self::layout::Container;

pub mod layout;
pub mod spacing;

pub fn parse_class_from_str(str: &str) -> Option<String> {
    // escape ":" with "\:"
    let class_selector = str.replace(':', "\\:");

    if class_selector.ends_with("container") {
        return Some(Container::parse_from_str(&class_selector));
    }

    // if class.starts_with("container") {
    //     return Some(Class::Container(Container(Modifier::parse_many_from_str(
    //         &modifiers,
    //     ))));
    // }

    // if class.starts_with("aspect-") {
    //     if let Some(aspect_ratio) = AspectRatio::parse_from_str(str) {
    //         return Some(Class::AspectRatio(aspect_ratio));
    //     }
    // }

    // if class.starts_with("p") {
    //     if let Some(padding) = Padding::parse_from_str(class, &modifiers) {
    //         return Some(Class::Padding(padding));
    //     }
    // }

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
