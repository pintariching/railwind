use crate::class::utils::{get_arbitrary_value, get_tuple_value, get_value, get_value_neg};
use crate::class::Decl;

use super::{
    CONTENT, FONT_FAMILY, FONT_SIZE, FONT_WEIGHT, LETTER_SPACING, LINE_STYLE_TYPE, TEXT_COLOR,
    TEXT_DECORATION_COLOR, TEXT_DECORATION_THICKNESS, TEXT_INDENT, TEXT_UNDERLINE_OFFSET,
};

#[derive(Debug)]
pub struct FontFamily<'a>(pub &'a str);

impl<'a> FontFamily<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &FONT_FAMILY)?;
        Some(Decl::Single(format!("font-family: {}", value)))
    }
}

#[derive(Debug)]
pub struct FontSize<'a>(pub &'a str);

impl<'a> FontSize<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_tuple_value(self.0, &FONT_SIZE)?;
        Some(Decl::Double([
            format!("font-size: {}", value.0),
            format!("line-height: {}", value.1),
        ]))
    }
}

#[derive(Debug)]
pub enum FontSmoothing {
    Antialiased,
    SubpixelAntialiased,
}

impl FontSmoothing {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "antialiased" => Self::Antialiased,
            "subpixel-antialiased" => Self::SubpixelAntialiased,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Decl {
        match self {
            FontSmoothing::Antialiased => Decl::Double([
                "-webkit-font-smoothing: antialiased".into(),
                "-moz-osx-font-smoothing: grayscale".into(),
            ]),
            FontSmoothing::SubpixelAntialiased => Decl::Double([
                "-webkit-font-smoothing: auto".into(),
                "-moz-osx-font-smoothing: auto".into(),
            ]),
        }
    }
}

#[derive(Debug)]
pub enum FontStyle {
    Italic,
    NonItalic,
}

impl FontStyle {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "italic" => Self::Italic,
            "non-italic" => Self::NonItalic,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            FontStyle::Italic => "italic",
            FontStyle::NonItalic => "normal",
        };

        Decl::Single(format!("font-style: {}", value))
    }
}

#[derive(Debug)]
pub struct FontWeight<'a>(pub &'a str);

impl<'a> FontWeight<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &FONT_WEIGHT)?;
        Some(Decl::Single(format!("font-weight: {}", value)))
    }
}

#[derive(Debug)]
pub enum FontVariantNumeric {
    NormalNums,
    Ordinal,
    SlashedZero,
    LiningNums,
    OldstyleNums,
    ProportialNums,
    TabularNums,
    DiagonalFractions,
    StackedFractions,
}

impl FontVariantNumeric {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "normal-nums" => Self::NormalNums,
            "ordinal" => Self::Ordinal,
            "slashed-zero" => Self::SlashedZero,
            "lining-nums" => Self::LiningNums,
            "oldstyle-nums" => Self::OldstyleNums,
            "proportional-nums" => Self::ProportialNums,
            "tabular-nums" => Self::TabularNums,
            "diagonal-fractions" => Self::DiagonalFractions,
            "stacked-fractions" => Self::StackedFractions,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            FontVariantNumeric::NormalNums => "normal",
            FontVariantNumeric::Ordinal => "ordinal",
            FontVariantNumeric::SlashedZero => "slashed-zero",
            FontVariantNumeric::LiningNums => "lining-nums",
            FontVariantNumeric::OldstyleNums => "oldstyle-nums",
            FontVariantNumeric::ProportialNums => "proportional-nums",
            FontVariantNumeric::TabularNums => "tabular-nums",
            FontVariantNumeric::DiagonalFractions => "diagonal-fractions",
            FontVariantNumeric::StackedFractions => "stacked-fractions",
        };

        Decl::Single(format!("font-variant-numeric: {}", value))
    }
}

#[derive(Debug)]
pub struct LetterSpacing<'a>(pub &'a str, bool);

impl<'a> LetterSpacing<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Self {
        let negative = name.starts_with('-');
        Self(arg, negative)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value_neg(self.1, self.0, &LETTER_SPACING)?;
        Some(Decl::Single(format!("font-weight: {}", value)))
    }
}

#[derive(Debug)]
pub struct LineHeight<'a>(pub &'a str);

impl<'a> LineHeight<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &LETTER_SPACING)?;
        Some(Decl::Single(format!("line-height: {}", value)))
    }
}

#[derive(Debug)]
pub struct LineStyleType<'a>(pub &'a str);

impl<'a> LineStyleType<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &LINE_STYLE_TYPE)?;
        Some(Decl::Single(format!("list-style-type: {}", value)))
    }
}

#[derive(Debug)]
pub enum ListStylePosition {
    Inside,
    Outside,
}

impl ListStylePosition {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "inside" => Self::Inside,
            "outside" => Self::Outside,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            ListStylePosition::Inside => "inside",
            ListStylePosition::Outside => "outside",
        };

        Decl::Single(format!("list-style-position: {}", value))
    }
}

#[derive(Debug)]
pub enum TextAlign {
    Left,
    Center,
    Right,
    Justify,
    Start,
    End,
}

impl TextAlign {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "left" => Self::Left,
            "center" => Self::Center,
            "right" => Self::Right,
            "justify" => Self::Justify,
            "start" => Self::Start,
            "end" => Self::End,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            TextAlign::Left => "left",
            TextAlign::Center => "center",
            TextAlign::Right => "right",
            TextAlign::Justify => "justify",
            TextAlign::Start => "start",
            TextAlign::End => "end",
        };

        Decl::Single(format!("text-align: {}", value))
    }
}

#[derive(Debug)]
pub struct TextColor<'a>(pub &'a str);

impl<'a> TextColor<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &TEXT_COLOR)?;
        Some(Decl::Single(format!("color: {}", value)))
    }
}

#[derive(Debug)]
pub enum TextDecoration {
    Underline,
    Overline,
    LineThrough,
    NoUnderline,
}

impl TextDecoration {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "underline" => Self::Underline,
            "overline" => Self::Overline,
            "line-through" => Self::LineThrough,
            "no-underline" => Self::NoUnderline,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            TextDecoration::Underline => "underline",
            TextDecoration::Overline => "overline",
            TextDecoration::LineThrough => "line-through",
            TextDecoration::NoUnderline => "none",
        };

        Decl::Single(format!("text-decoration-line: {}", value))
    }
}

#[derive(Debug)]
pub struct TextDecorationColor<'a>(pub &'a str);

impl<'a> TextDecorationColor<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &TEXT_DECORATION_COLOR)?;
        Some(Decl::Single(format!("text-decoration-color: {}", value)))
    }
}

#[derive(Debug)]
pub enum TextDecorationStyle {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

impl TextDecorationStyle {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "solid" => Self::Solid,
            "double" => Self::Double,
            "dotted" => Self::Dotted,
            "dashed" => Self::Dashed,
            "wavy" => Self::Wavy,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            TextDecorationStyle::Solid => "solid",
            TextDecorationStyle::Double => "double",
            TextDecorationStyle::Dotted => "dotted",
            TextDecorationStyle::Dashed => "dashed",
            TextDecorationStyle::Wavy => "wavy",
        };

        Decl::Single(format!("text-decoration-style: {}", value))
    }
}

#[derive(Debug)]
pub struct TextDecorationThickness<'a>(pub &'a str);

impl<'a> TextDecorationThickness<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &TEXT_DECORATION_THICKNESS)?;
        Some(Decl::Single(format!(
            "text-decoration-thickness: {}",
            value
        )))
    }
}

#[derive(Debug)]
pub struct TextUnderlineOffset<'a>(pub &'a str);

impl<'a> TextUnderlineOffset<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &TEXT_UNDERLINE_OFFSET)?;
        Some(Decl::Single(format!("text-underline-offset: {}", value)))
    }
}

#[derive(Debug)]
pub enum TextTransform {
    Uppercase,
    Lowercase,
    Capitalize,
    NormalCase,
}

impl TextTransform {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "uppercase" => Self::Uppercase,
            "lowercase" => Self::Lowercase,
            "capitalize" => Self::Capitalize,
            "normal-case" => Self::NormalCase,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            TextTransform::Uppercase => "uppercase",
            TextTransform::Lowercase => "lowercase",
            TextTransform::Capitalize => "capitalize",
            TextTransform::NormalCase => "none",
        };

        Decl::Single(format!("text-transform: {}", value))
    }
}

#[derive(Debug)]
pub enum TextOverflow {
    Truncate,
    TextEllipsis,
    TextClip,
}

impl TextOverflow {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "truncate" => Self::Truncate,
            "text-ellipsis" => Self::TextEllipsis,
            "text-clip" => Self::TextClip,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            TextOverflow::Truncate => {
                return Decl::Triple([
                    "overflow: hidden".into(),
                    "text-overflow: ellipsis".into(),
                    "white-space: nowrap".into(),
                ])
            }
            TextOverflow::TextEllipsis => "elipsis",
            TextOverflow::TextClip => "clip",
        };

        Decl::Single(format!("text-overflow: {}", value))
    }
}

#[derive(Debug)]
pub struct TextIndent<'a>(pub &'a str, pub bool);

impl<'a> TextIndent<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Self {
        let negative = name.starts_with('-');
        Self(arg, negative)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value_neg(self.1, self.0, &TEXT_INDENT)?;
        Some(Decl::Single(format!("text-indent: {}", value)))
    }
}

#[derive(Debug)]
pub enum VerticalAlign {
    Baseline,
    Top,
    Middle,
    Bottom,
    TextTop,
    TextBottom,
    Sub,
    Super,
    Arbitrary(String),
}

impl VerticalAlign {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "baseline" => Self::Baseline,
            "top" => Self::Top,
            "middle" => Self::Middle,
            "bottom" => Self::Bottom,
            "text-top" => Self::TextTop,
            "text-bottom" => Self::TextBottom,
            "sub" => Self::Sub,
            "super" => Self::Super,
            _ => {
                if let Some(arbitrary) = get_arbitrary_value(arg) {
                    Self::Arbitrary(arbitrary)
                } else {
                    return None;
                }
            }
        };

        Some(value)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = match self {
            VerticalAlign::Baseline => "baseline",
            VerticalAlign::Top => "top",
            VerticalAlign::Middle => "middle",
            VerticalAlign::Bottom => "bottom",
            VerticalAlign::TextTop => "text-top",
            VerticalAlign::TextBottom => "text-bottom",
            VerticalAlign::Sub => "sub",
            VerticalAlign::Super => "super",
            VerticalAlign::Arbitrary(a) => {
                return Some(Decl::Single(format!(
                    "vertical-align: {}",
                    get_arbitrary_value(&a)?
                )))
            }
        };

        Some(Decl::Single(format!("vertical-align: {}", value)))
    }
}

#[derive(Debug)]
pub enum Whitespace {
    Normal,
    NoWrap,
    Pre,
    PreLine,
    PreWrap,
}

impl Whitespace {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "normal" => Self::Normal,
            "nowrap" => Self::NoWrap,
            "pre" => Self::Pre,
            "pre-line" => Self::PreLine,
            "pre-wrap" => Self::PreWrap,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            Whitespace::Normal => "normal",
            Whitespace::NoWrap => "nowrap",
            Whitespace::Pre => "pre",
            Whitespace::PreLine => "pre-line",
            Whitespace::PreWrap => "pre-wrap",
        };

        Decl::Single(format!("white-space: {}", value))
    }
}

#[derive(Debug)]
pub enum WordBreak {
    Normal,
    Words,
    All,
    Keep,
}

impl WordBreak {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "normal" => Self::Normal,
            "words" => Self::Words,
            "all" => Self::All,
            "keep" => Self::Keep,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            WordBreak::Normal => {
                return Decl::Double(["overflow-wrap: normal".into(), "word-break: normal".into()])
            }
            WordBreak::Words => return Decl::Lit("overflow-wrap: break-word"),
            WordBreak::All => "break-all",
            WordBreak::Keep => "keep-all",
        };

        Decl::Single(format!("word-break: {}", value))
    }
}

#[derive(Debug)]
pub struct Content<'a>(pub &'a str);

impl<'a> Content<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &CONTENT)?;
        Some(Decl::Single(format!("content: {}", value)))
    }
}
