use crate::class::generate_class;

pub struct BorderRadius;

impl BorderRadius {
    pub fn parse_from_str(class: &str, border_radius: &str) -> Option<String> {
        if border_radius.starts_with("rounded") {
            match border_radius.matches('-').count() {
                0 =>
                // rounded
                {
                    return Some(generate_class(
                        class,
                        ".[class-selector] {\n  border-radius: 0.25rem;\n}\n",
                    ))
                }
                1 =>
                // rounded-t, rounded-b.. rounded-none, rounded-sm, rounded-lg...
                {
                    let dir_or_size = border_radius.split('-').nth(1)?;
                    return Some(generate_class(
                        class,
                        &format!(
                            ".[class-selector] {{\n  {}\n}}\n",
                            convert(dir_or_size, "")?
                        ),
                    ));
                }
                2 =>
                // rounder-t-lg, rounded-bl-sm..
                {
                    let mut split = border_radius.split('-');
                    split.next();
                    let dir = split.next()?;
                    let size = split.next()?;
                    return Some(generate_class(
                        class,
                        &format!(".[class-selector] {{\n  {}\n}}\n", convert(dir, size)?),
                    ));
                }
                _ => return None,
            }
        }

        None
    }
}

fn convert_size(size: &str) -> Option<&'static str> {
    Some(match size {
        "none" => "0px",
        "sm" => "0.125rem",
        "" => "0.25rem",
        "md" => "0.375rem",
        "lg" => "0.5rem",
        "xl" => "0.75rem",
        "2xl" => "1rem",
        "3xl" => "1.5rem",
        "full" => "9999px",
        _ => return None,
    })
}

fn convert(dir_or_size: &str, raw_size: &str) -> Option<String> {
    if let Some(size) = convert_size(dir_or_size) {
        return Some(format!("border-radius: {};", size));
    }

    let size = convert_size(raw_size)?;

    if dir_or_size.len() == 1 {
        return match dir_or_size {
            "t" => Some(format!(
                "border-top-left-radius: {};\n  border-top-right-radius: {};",
                size, size
            )),
            "b" => Some(format!(
                "border-bottom-left-radius: {};\n  border-bottom-right-radius: {};",
                size, size
            )),
            "l" => Some(format!(
                "border-top-left-radius: {};\n  border-bottom-left-radius: {};",
                size, size
            )),
            "r" => Some(format!(
                "border-top-right-radius: {};\n  border-bottom-right-radius: {};",
                size, size
            )),
            _ => None,
        };
    }

    if dir_or_size.len() == 2 {
        let vertical = match dir_or_size.chars().next()? {
            't' => "top",
            'b' => "bottom",
            _ => return None,
        };

        let horizontal = match dir_or_size.chars().next()? {
            'l' => "left",
            'r' => "right",
            _ => return None,
        };

        return Some(format!(
            "border-{}-{}-radius: {};",
            vertical, horizontal, size
        ));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert!(convert("t", "").is_some());
        assert_eq!(
            convert("t", "").unwrap(),
            "border-top-left-radius: 0.25rem;\n  border-top-right-radius: 0.25rem;"
        );

        assert!(convert("sm", "").is_some());
        assert_eq!(convert("sm", "").unwrap(), "border-radius: 0.125rem;");

        assert!(convert("tl", "sm").is_some());
        assert_eq!(
            convert("tl", "sm").unwrap(),
            "border-top-left-radius: 0.125rem;"
        );
    }
}
