// Indents every line in a string with two spaces
// and removes empty lines
pub fn indent_string(str: &str) -> String {
    str.lines().map(|line| format!("    {}\n", line)).collect()
}

pub fn replace_invalid_chars(selector: impl Into<String>) -> String {
    let invalid_chars = ['[', ']', '%', ':', '.', '/', '(', ')', '\'', '#'];
    let mut val: String = selector.into();

    if val.contains(invalid_chars) {
        let mut to_escape = Vec::new();
        for (index, c) in val.chars().enumerate() {
            if invalid_chars.contains(&c) {
                to_escape.push(index);
            }
        }

        to_escape.reverse();

        for index in to_escape {
            val.insert(index, '\\');
        }
    }

    val.replace(",", "\\2c ")
}

pub fn get_class_name<'a>(value: &'a str) -> &'a str {
    if value.starts_with('-') {
        if let Some(index) = value[1..].find('-') {
            return &value[..index + 1];
        }
        {
            value
        }
    } else if let Some(index) = value.find('-') {
        &value[..index]
    } else {
        value
    }
}

pub fn get_args<'a>(value: &'a str) -> Option<&'a str> {
    if value.starts_with('-') {
        if let Some(index) = value[1..].find('-') {
            return Some(&value[index + 2..]);
        }
    } else if let Some(index) = value.find('-') {
        return Some(&value[index + 1..]);
    }

    None
}

pub fn get_opt_args<'a>(value: &'a str) -> &'a str {
    if value.starts_with('-') {
        if let Some(index) = value[1..].find('-') {
            &value[index + 2..]
        } else {
            ""
        }
    } else if let Some(index) = value.find('-') {
        &value[index + 1..]
    } else {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent_string() {
        assert_eq!(indent_string("test"), "    test\n");
        assert_eq!(
            indent_string(
                r#"test
    123
"#
            ),
            r#"    test
        123
"#
        );
    }

    #[test]
    fn test_replace_invalid_chars() {
        assert_eq!(replace_invalid_chars("space-x-5"), "space-x-5");
        assert_eq!(replace_invalid_chars("space-x-[25%]"), r"space-x-\[25\%\]");
    }

    #[test]
    fn test_get_class_name() {
        assert_eq!(get_class_name("aspect-auto"), "aspect");
        assert_eq!(get_class_name("flex"), "flex");
        assert_eq!(get_class_name("-mx-5"), "-mx");
        assert_eq!(get_class_name("-m"), "-m");
    }

    #[test]
    fn test_get_args() {
        assert_eq!(get_args("aspect-auto"), Some("auto"));
        assert_eq!(get_args("flex"), None);
        assert_eq!(get_args("space-x-5"), Some("x-5"));
        assert_eq!(get_args("-space-x-5"), Some("x-5"));
        assert_eq!(get_args("-mx-5"), Some("5"));
        let args = get_args("space-x-5").unwrap();
        let params = get_args(args).unwrap();

        assert_eq!(params, "5");
    }

    #[test]
    fn test_get_opt_args() {
        assert_eq!(get_opt_args("aspect-auto"), "auto");
        assert_eq!(get_opt_args("flex"), "");
        assert_eq!(get_opt_args("space-x-5"), "x-5");
        assert_eq!(get_opt_args("-space-x-5"), "x-5");
        assert_eq!(get_opt_args("-mx-5"), "5");
    }
}
