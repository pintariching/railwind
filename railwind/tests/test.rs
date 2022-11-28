#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use std::path::Path;

    use railwind::parse_html_file;
    use walkdir::WalkDir;

    #[test]
    fn test_parsing() {
        for entry in WalkDir::new("tests") {
            match entry {
                Ok(ent) => {
                    if ent.metadata().unwrap().is_file() {
                        let path = ent.path().to_owned();

                        if path.file_name().unwrap() == "input.html" {
                            let input = path.clone();
                            let mut expected = path.clone();
                            let mut output = path.clone();

                            expected.set_file_name("expected.css");
                            output.set_file_name("output.css");

                            parse_html_file(Path::new(&input), Path::new(&output));

                            assert_eq!(
                                read_to_string(output).unwrap(),
                                read_to_string(expected).unwrap()
                            );
                        }
                    }
                }
                Err(e) => println!("{:#?}", e),
            }
        }
    }
}
