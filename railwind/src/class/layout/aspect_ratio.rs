use crate::class::generate_class;

#[derive(Debug)]
pub struct AspectRatio;

impl AspectRatio {
    pub fn parse_from_str(class: &str, ratio: &str) -> Option<String> {
        if let Some(ratio) = match ratio {
            "aspect-auto" => Some("auto"),
            "aspect-square" => Some("1 / 1"),
            "aspect-video" => Some("16 / 9"),
            _ => return None,
        } {
            let template = format!(".[class-selector] {{\n  aspect-ratio: {};\n}}\n", ratio);
            return Some(generate_class(class, &template));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_from_str_auto() {
        let result = AspectRatio::parse_from_str("aspect-auto", "aspect-auto");
        assert!(result.is_some());
        assert_eq!(
            result.unwrap(),
            ".aspect-auto {\n  aspect-ratio: auto;\n}\n"
        );
    }

    #[test]
    fn test_parse_from_str_square() {
        let result = AspectRatio::parse_from_str("aspect-square", "aspect-square");
        assert!(result.is_some());
        assert_eq!(
            result.unwrap(),
            ".aspect-square {\n  aspect-ratio: 1 / 1;\n}\n"
        );
    }

    #[test]
    fn test_parse_from_str_video() {
        let result = AspectRatio::parse_from_str("aspect-video", "aspect-video");
        assert!(result.is_some());
        assert_eq!(
            result.unwrap(),
            ".aspect-video {\n  aspect-ratio: 16 / 9;\n}\n"
        );
    }

    #[test]
    fn test_parse_from_str_none() {
        let result = AspectRatio::parse_from_str("aspect-vidjeo", "aspect-vidjeo");
        assert!(result.is_none());
    }
}
