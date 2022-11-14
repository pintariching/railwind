use std::str::FromStr;

use crate::traits::ToStaticStr;

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

impl FromStr for MediaQuery {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "sm" => Ok(MediaQuery::Sm),
            "md" => Ok(MediaQuery::Md),
            "lg" => Ok(MediaQuery::Lg),
            "xl" => Ok(MediaQuery::Xl),
            "2xl" => Ok(MediaQuery::Xxl),
            "dark" => Ok(MediaQuery::Dark),
            "motion-reduce" => Ok(MediaQuery::MotionReduce),
            "motion-safe" => Ok(MediaQuery::MotionSafe),
            "contrast-more" => Ok(MediaQuery::ContrastMore),
            "contrast-less" => Ok(MediaQuery::ContrastLess),
            "portrait" => Ok(MediaQuery::Portrait),
            "landscape" => Ok(MediaQuery::Landscape),
            "print" => Ok(MediaQuery::Print),
            "ltr" => Ok(MediaQuery::Ltr),
            "rtl" => Ok(MediaQuery::Rtl),
            _ => Err(()),
        }
    }
}

impl ToStaticStr for MediaQuery {
    fn to_static_str(&self) -> &'static str {
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
