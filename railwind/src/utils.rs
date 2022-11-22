// Indents every line in a string with two spaces
// and removes empty lines
pub fn indent_string(str: &str) -> String {
    str.lines().map(|line| format!("    {}\n", line)).collect()
}

pub fn replace_invalid_chars(selector: impl Into<String>) -> String {
    let invalid_chars = ['[', ']', '%', ':'];
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

    val
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
}
