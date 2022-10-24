use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref COLORS: HashMap<&'static str, HashMap<&'static str, &'static str>> =
        ron::from_str(include_str!("colors.ron")).unwrap();
    static ref SPACING: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("spacing.ron")).unwrap();
}

// Accepts a color value like `slate-500` and returns a rgb value, seperated by spaces `100 116 139`
pub fn convert_color(color_name: &str, color_value: Option<&str>) -> Option<String> {
    if let Some(value) = color_value {
        let color = COLORS.get(color_name)?;
        let hex_str = color.get(value)?;

        let mut rgb_array = [0; 3];
        hex::decode_to_slice(hex_str, &mut rgb_array).ok()?;

        return Some(rgb_array.map(|v| v.to_string()).join(" "));
    }

    match color_name {
        "black" => return Some("0 0 0".into()),
        "white" => return Some("255 255 255".into()),
        _ => None,
    }
}

pub fn convert_spacing<'a>(spacing_value: &'a str) -> Result<&'a str, String> {
    if let Some(value) = SPACING.get(spacing_value) {
        return Ok(value);
    }

    Err(format!("failed to match spacing value: {}", spacing_value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_color() {
        let result = convert_color("slate", Some("500"));
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "100 116 139");
    }
}
