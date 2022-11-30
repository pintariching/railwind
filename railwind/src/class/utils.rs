use std::collections::HashMap;

pub fn get_value<'a>(
    arg: &'a str,
    hashmap: &HashMap<&'static str, &'static str>,
) -> Option<&'a str> {
    if let Some(arbitrary) = get_arbitrary_value(arg) {
        return Some(arbitrary);
    }

    if let Some(value) = hashmap.get(arg) {
        return Some(value);
    }

    None
}

pub fn get_value_neg(
    negative: bool,
    arg: &str,
    hashmap: &HashMap<&'static str, &'static str>,
) -> Option<String> {
    if let Some(arbitrary) = get_arbitrary_value(arg) {
        return Some(arbitrary.to_string());
    }

    if let Some(value) = hashmap.get(arg) {
        if negative {
            return Some(format!("-{}", value));
        }

        return Some(value.to_string());
    }

    None
}

pub fn get_tuple_value<'a>(
    arg: &'a str,
    hashmap: &HashMap<&'static str, (&'static str, &'static str)>,
) -> Option<(&'a str, &'a str)> {
    if let Some(arbitrary) = get_arbitrary_value(arg) {
        return Some((arbitrary, arbitrary));
    }

    if let Some(value) = hashmap.get(arg) {
        return Some(*value);
    }

    None
}

pub fn get_arbitrary_value(arg: &str) -> Option<&str> {
    if arg.starts_with('[') && arg.ends_with(']') {
        Some(&arg[1..arg.len() - 1])
    } else {
        None
    }
}
