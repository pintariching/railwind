#[cfg(test)]
mod tests {
    use std::fs::{read_to_string, File};
    use std::io::Write;
    use std::path::Path;

    use railwind::parse_html_to_string;
    use walkdir::WalkDir;

    #[test]
    fn test_parsing() {
        for entry in WalkDir::new("tests") {
            match entry {
                Ok(ent) => {
                    if ent.metadata().unwrap().is_file() {
                        let path = ent.path().to_owned();

                        if path.file_name().unwrap() == "input.html" {
                            println!("Running test with file: {:?}", &path);

                            let input = path.clone();
                            let mut expected = path.clone();

                            expected.set_file_name("expected.css");

                            let css = parse_html_to_string(Path::new(&input), false, &mut vec![]);
                            let expected_string = read_to_string(&expected).unwrap();

                            let mut output = path.clone();
                            output.set_file_name("output.css");
                            let mut file = File::create(&output).unwrap();
                            file.write_all(css.as_bytes()).unwrap();

                            assert_eq!(css, expected_string);
                        }
                    }
                }
                Err(e) => println!("{:#?}", e),
            }
        }
    }
}
