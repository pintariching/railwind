use self::background_color::BackgroundColor;

mod background_color;

pub struct Background;

impl Background {
    pub fn parse_from_str(class: &str, background: &str) -> Option<String> {
        let mut split = background.split('-');
        split.next();
        let utility = split.next()?;
        let value = split.next()?;

        match utility {
            _ => BackgroundColor::parse_from_str(class, utility, value),
        }
    }
}
