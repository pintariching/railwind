#[derive(Debug, Hash, PartialEq, Eq, Clone)]
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
            "sm" => Self::Sm,
            "md" => Self::Md,
            "lg" => Self::Lg,
            "xl" => Self::Xl,
            "2xl" => Self::Xxl,
            "dark" => Self::Dark,
            "motion-reduce" => Self::MotionReduce,
            "motion-safe" => Self::MotionSafe,
            "contrast-more" => Self::ContrastMore,
            "contrast-less" => Self::ContrastLess,
            "portrait" => Self::Portrait,
            "landscape" => Self::Landscape,
            "print" => Self::Print,
            "ltr" => Self::Ltr,
            "rtl" => Self::Rtl,
            _ => return None,
        };

        Some(mq)
    }

    pub fn to_static_str(self) -> &'static str {
        match self {
            Self::Sm => "min-width: 640px",
            Self::Md => "min-width: 768px",
            Self::Lg => "min-width: 1024px",
            Self::Xl => "min-width: 1280px",
            Self::Xxl => "min-width: 1536px",
            Self::Dark => "prefers-color-scheme: dark",
            Self::MotionReduce => "prefers-reduced-motion: reduce",
            Self::MotionSafe => "prefers-reduced-motion: no-preference",
            Self::ContrastMore => "prefers-contrast: more",
            Self::ContrastLess => "prefers-contrast: less",
            Self::Portrait => "orientation: portrait",
            Self::Landscape => "orientation: landscape",
            Self::Print => "print",
            Self::Ltr => r#"[dir="ltr"]"#,
            Self::Rtl => r#"[dir="rtl"]"#,
        }
    }
}
