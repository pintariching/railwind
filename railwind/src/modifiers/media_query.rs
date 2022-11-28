#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MediaQuery {
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
    Dark,
    MotionReduce,
    MotionSafe,
    ContrastMore,
    ContrastLess,
    Portrait,
    Landscape,
    Print,
    Ltr,
    Rtl,
}

impl MediaQuery {
    pub fn new(value: &str) -> Option<Self> {
        let mq = match value {
            "sm" => MediaQuery::Sm,
            "md" => MediaQuery::Md,
            "lg" => MediaQuery::Lg,
            "xl" => MediaQuery::Xl,
            "2xl" => MediaQuery::Xxl,
            "dark" => MediaQuery::Dark,
            "motion-reduce" => MediaQuery::MotionReduce,
            "motion-safe" => MediaQuery::MotionSafe,
            "contrast-more" => MediaQuery::ContrastMore,
            "contrast-less" => MediaQuery::ContrastLess,
            "portrait" => MediaQuery::Portrait,
            "landscape" => MediaQuery::Landscape,
            "print" => MediaQuery::Print,
            "ltr" => MediaQuery::Ltr,
            "rtl" => MediaQuery::Rtl,
            _ => return None,
        };

        Some(mq)
    }

    pub fn to_static_str(self) -> &'static str {
        match self {
            MediaQuery::Sm => "min-width: 640px",
            MediaQuery::Md => "min-width: 768px",
            MediaQuery::Lg => "min-width: 1024px",
            MediaQuery::Xl => "min-width: 1280px",
            MediaQuery::Xxl => "min-width: 1536px",
            MediaQuery::Dark => "prefers-color-scheme: dark",
            MediaQuery::MotionReduce => "prefers-reduced-motion: reduce",
            MediaQuery::MotionSafe => "prefers-reduced-motion: no-preference",
            MediaQuery::ContrastMore => "prefers-contrast: more",
            MediaQuery::ContrastLess => "prefers-contrast: less",
            MediaQuery::Portrait => "orientation: portrait",
            MediaQuery::Landscape => "orientation: landscape",
            MediaQuery::Print => "print",
            MediaQuery::Ltr => r#"[dir="ltr"]"#,
            MediaQuery::Rtl => r#"[dir="rtl"]"#,
        }
    }
}
