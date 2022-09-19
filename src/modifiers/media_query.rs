#[derive(Debug, PartialEq)]
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
            MediaQuery::Sm => "sm",
            MediaQuery::Md => "md",
            MediaQuery::Lg => "lg",
            MediaQuery::Xl => "xl",
            MediaQuery::Xxl => "2xl",
            MediaQuery::Dark => "dark",
            MediaQuery::MotionReduce => "motion-reduce",
            MediaQuery::MotionSafe => "motion-safe",
            MediaQuery::ContrastMore => "contrast-more",
            MediaQuery::ContrastLess => "contrast-less",
            MediaQuery::Portrait => "portrait",
            MediaQuery::Landscape => "landscape",
            MediaQuery::Print => "print",
            MediaQuery::Ltr => "ltr",
            MediaQuery::Rtl => "rtl",
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
