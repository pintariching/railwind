pub fn indent_string<'a>(str: &'a str) -> String {
    str.lines()
        .filter_map(|line| {
            println!("{}", line);
            if line.trim().is_empty() {
                return None;
            }
            return Some(format!("  {}\n", line));
        })
        .collect()
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
