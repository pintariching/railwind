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
        Some(arg[1..arg.len() - 1].replace("_", " "))
    } else {
        None
    }
}
