use std::collections::HashMap;

pub fn get_value(arg: &str, hashmap: &HashMap<&'static str, &'static str>) -> Option<String> {
    if arg.starts_with('[') && arg.ends_with(']') {
        return Some(arg[1..arg.len() - 1].to_string());
    }

    if let Some(value) = hashmap.get(arg) {
        return Some(value.to_string());
    }

    None
}

pub fn get_value_neg(
    class_name: &str,
    arg: &str,
    hashmap: &HashMap<&'static str, &'static str>,
) -> Option<String> {
    if arg.starts_with('[') && arg.ends_with(']') {
        return Some(arg[1..arg.len() - 1].to_string());
    }

    if let Some(value) = hashmap.get(arg) {
        if class_name.starts_with('-') {
            return Some(format!("-{}", value));
        }

        return Some(value.to_string());
    }

    None
}
