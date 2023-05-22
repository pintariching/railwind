use std::cell::OnceCell;
use std::collections::HashMap;

#[derive(Default)]
pub struct LayoutConfig {
    pub aspect_ratio: OnceCell<HashMap<&'static str, &'static str>>,
    pub columns: OnceCell<HashMap<&'static str, &'static str>>,
    pub break_after: OnceCell<HashMap<&'static str, &'static str>>,
    pub break_before: OnceCell<HashMap<&'static str, &'static str>>,
    pub break_inside: OnceCell<HashMap<&'static str, &'static str>>,
    pub decoration_break: OnceCell<HashMap<&'static str, &'static str>>,
}

impl LayoutConfig {
    pub fn aspect_ratio(&self) -> &HashMap<&'static str, &'static str> {
        self.aspect_ratio.get_or_init(|| init_aspect_ratio())
    }

    pub fn column(&self) -> &HashMap<&'static str, &'static str> {
        self.columns.get_or_init(|| init_columns())
    }
}

fn init_aspect_ratio() -> HashMap<&'static str, &'static str> {
    HashMap::from([("auto", "auto"), ("square", "1 / 1"), ("video", "16 / 9")])
}

fn init_columns() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("1", "1"),
        ("2", "2"),
        ("3", "3"),
        ("4", "4"),
        ("5", "5"),
        ("6", "6"),
        ("7", "7"),
        ("8", "8"),
        ("9", "9"),
        ("10", "10"),
        ("11", "11"),
        ("12", "12"),
        ("auto", "auto"),
        ("3xs", "16rem"),
        ("2xs", "18rem"),
        ("xs", "20rem"),
        ("sm", "24rem"),
        ("md", "28rem"),
        ("lg", "32rem"),
        ("xl", "36rem"),
        ("2xl", "42rem"),
        ("3xl", "48rem"),
        ("4xl", "56rem"),
        ("5xl", "64rem"),
        ("6xl", "72rem"),
        ("7xl", "80"),
    ])
}
