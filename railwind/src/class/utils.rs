use std::collections::HashMap;

pub fn get_value<'a>(
    arg: &'a str,
    hashmap: &HashMap<&'static str, &'static str>,
) -> Option<&'a str> {
    if arg.starts_with('[') && arg.ends_with(']') {
        return Some(&arg[1..arg.len() - 1]);
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
    if arg.starts_with('[') && arg.ends_with(']') {
        return Some(arg[1..arg.len() - 1].to_string());
    }

    if let Some(value) = hashmap.get(arg) {
        if negative {
            return Some(format!("-{}", value));
        }

        return Some(value.to_string());
    }

    None
}
