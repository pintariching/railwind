// Indents every line in a string with two spaces
// and removes empty lines
pub fn indent_string(str: &str) -> String {
    str.lines().map(|line| format!("    {}\n", line)).collect()
}

#[cfg(test)]
mod tests {
    use super::indent_string;

    #[test]
    fn test_indent_string() {
        assert_eq!(indent_string("test"), "  test\n");
        assert_eq!(
            indent_string(
                r#"test
  123
"#
            ),
            r#"  test
    123
"#
        );
    }
}
