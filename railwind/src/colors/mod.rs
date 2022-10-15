use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref COLORS: HashMap<&'static str, HashMap<&'static str, &'static str>> =
        ron::from_str(include_str!("colors.ron")).unwrap();
}

// Accepts a color value like `slate-500` and returns a rgb value, seperated by spaces `100 116 139`
pub fn convert_color(color: &str) -> Option<String> {
    let mut split = color.split('-');
    let color_name = split.next()?;
    let color_value = split.next()?;

    let color = COLORS.get(color_name)?;
    let hex_str = color.get(color_value)?;

    let mut rgb_array = [0; 3];
    hex::decode_to_slice(hex_str, &mut rgb_array).ok()?;

    Some(rgb_array.map(|v| v.to_string()).join(" "))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_color() {
        let result = convert_color("slate-500");
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "100 116 139");
    }
}
