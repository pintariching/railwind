use crate::class::generate_class;

pub struct BorderWidth;

impl BorderWidth {
    pub fn parse_from_str(class: &str, border_width: &str) -> Option<String> {
        if border_width.starts_with("border") {
            match border_width.matches('-').count() {
                0 =>
                // border
                {
                    return Some(generate_class(
                        class,
                        ".[class-selector] {\n  border-width: 1px;\n}\n",
                    ))
                }
                1 =>
                // border-t border-4
                {
                    let dir_or_size = border_width.split('-').nth(1)?;
                    return Some(generate_class(
                        class,
                        &format!(
                            ".[class-selector] {{\n  {}\n}}\n",
                            convert(dir_or_size, "")?
                        ),
                    ));
                }
                2 =>
                // border-t-4, border-l-2
                {
                    let mut split = border_width.split('-');
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
        "0" => "0px",
        "" => "1px",
        "2" => "2px",
        "4" => "4px",
        "8" => "8px",
        _ => return None,
    })
}

fn convert_direction(dir: &str) -> Option<&'static str> {
    match dir {
        "t" => Some("top"),
        "b" => Some("bottom"),
        "l" => Some("left"),
        "r" => Some("right"),
        _ => None,
    }
}

fn convert(dir_or_size: &str, raw_size: &str) -> Option<String> {
    if let Some(size) = convert_size(dir_or_size) {
        return Some(format!("border-width: {};", size));
    }

    let size = convert_size(raw_size)?;

    match dir_or_size {
        "x" => Some(format!(
            "border-left-width: {};\n  border-right-width: {};",
            size, size
        )),
        "y" => Some(format!(
            "border-top-width: {};\n  border-bottom-width: {};",
            size, size
        )),
        _ => {
            let dir = convert_direction(dir_or_size)?;
            Some(format!("border-{}-width: {};", dir, size))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert() {
        assert!(convert("x", "").is_some());
        assert_eq!(
            convert("x", "").unwrap(),
            "border-left-width: 1px;\n  border-right-width: 1px;"
        );

        assert!(convert("t", "").is_some());
        assert_eq!(convert("t", "").unwrap(), "border-top-width: 1px;");

        assert!(convert("l", "4").is_some());
        assert_eq!(convert("l", "4").unwrap(), "border-left-width: 4px;");
    }
}
