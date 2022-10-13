#[derive(Debug, PartialEq, Eq)]
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
    pub fn as_str(&self) -> &'static str {
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

    pub fn parse_from_str(str: &str) -> Option<MediaQuery> {
        match str {
            "sm" => Some(MediaQuery::Sm),
            "md" => Some(MediaQuery::Md),
            "lg" => Some(MediaQuery::Lg),
            "xl" => Some(MediaQuery::Xl),
            "2xl" => Some(MediaQuery::Xxl),
            "dark" => Some(MediaQuery::Dark),
            "motion-reduce" => Some(MediaQuery::MotionReduce),
            "motion-safe" => Some(MediaQuery::MotionSafe),
            "contrast-more" => Some(MediaQuery::ContrastMore),
            "contrast-less" => Some(MediaQuery::ContrastLess),
            "portrait" => Some(MediaQuery::Portrait),
            "landscape" => Some(MediaQuery::Landscape),
            "print" => Some(MediaQuery::Print),
            "ltr" => Some(MediaQuery::Ltr),
            "rtl" => Some(MediaQuery::Rtl),
            _ => None,
        }
    }
}
