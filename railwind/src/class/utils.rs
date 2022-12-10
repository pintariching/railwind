use std::collections::HashMap;

pub fn get_value<'a>(
    arg: &'a str,
    hashmap: &HashMap<&'static str, &'static str>,
) -> Option<String> {
    if let Some(arbitrary) = get_arbitrary_value(arg) {
        return Some(arbitrary);
    }

    if let Some(value) = hashmap.get(arg) {
        return Some(value.to_string());
    }

    None
}

pub fn get_value_neg(
    negative: bool,
    arg: &str,
    hashmap: &HashMap<&'static str, &'static str>,
) -> Option<String> {
    if let Some(arbitrary) = get_arbitrary_value(arg) {
        if negative {
            if arbitrary.starts_with('-') {
                return Some(arbitrary[1..].to_string());
            } else {
                return Some(format!("-{}", arbitrary));
            }
        } else {
            return Some(arbitrary.to_string());
        }
    }

    if let Some(value) = hashmap.get(arg) {
        if negative {
            if value.starts_with('-') {
                return Some(value[1..].to_string());
            } else {
                return Some(format!("-{}", value));
            }
        }

        return Some(value.to_string());
    }

    None
}

pub fn get_tuple_value<'a>(
    arg: &'a str,
    hashmap: &HashMap<&'static str, (&'static str, &'static str)>,
) -> Option<(String, String)> {
    if let Some(arbitrary) = get_arbitrary_value(arg) {
        return Some((arbitrary.clone(), arbitrary));
    }

    if let Some(value) = hashmap.get(arg) {
        return Some((value.0.into(), value.1.into()));
    }

    None
}

pub fn get_arbitrary_value(arg: &str) -> Option<String> {
    if arg.starts_with('[') && arg.ends_with(']') {
        let mut value = arg[1..arg.len() - 1].to_string();
        if value.starts_with('.') {
            value = format!("0{}", value);
        }

        value = value.replace("_", " ").replace("'", "\"");

        Some(value)
    } else {
        None
    }
}

static UNITS: [&'static str; 26] = [
    "cm", "mm", "Q", "in", "pc", "pt", "px", "em", "ex", "ch", "rem", "lh", "rlh", "vw", "vh",
    "vmin", "vmax", "vb", "vi", "svw", "svh", "lvw", "lvh", "dvw", "dvh", "%",
];

pub fn value_is_size(arg: &str) -> bool {
    let value = if arg.starts_with('[') && arg.ends_with(']') {
        arg.get(1..arg.len() - 1).unwrap()
    } else {
        arg
    };

    for unit in UNITS {
        if value.ends_with(unit) {
            return true;
        } else {
            continue;
        }
    }

    false
}

pub fn value_is_hex(arg: &str) -> bool {
    (arg.starts_with("[#") && arg.ends_with("]")) || arg.starts_with("#")
}

pub fn hex_to_rgb_color(value: &str) -> Option<[u8; 3]> {
    let value = if value.starts_with('#') {
        &value[1..]
    } else {
        value
    };

    let mut colors = [0; 3];
    for i in 0..3 {
        let char = if value.len() == 3 {
            let c = &value[i..i + 1];

            if c == "f" {
                "ff"
            } else {
                c
            }
        } else {
            &value[i * 2..i * 2 + 2]
        };

        let color = u8::from_str_radix(char, 16).ok()?;

        colors[i] = color;
    }

    Some(colors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_is_size() {
        assert!(value_is_size("5rem"));
        assert!(value_is_size("50%"));
        assert!(value_is_size("[25px]"));
        assert!(!value_is_size("red-500"));
    }

    #[test]
    fn test_hex_to_rgb_color() {
        assert!(hex_to_rgb_color("#000").is_some());
        assert_eq!(hex_to_rgb_color("#000").unwrap(), [0, 0, 0]);
        assert_eq!(hex_to_rgb_color("#64748b").unwrap(), [100, 116, 139]);
        assert!(hex_to_rgb_color("#color").is_none());
    }
}
