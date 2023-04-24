use crate::class::utils::{
    get_arbitrary_value, get_tuple_value, get_value, get_value_neg, hex_to_rgb_color,
};
use crate::class::Decl;
use crate::warning::WarningType;

use super::{
    CONTENT, FONT_FAMILY, FONT_SIZE, FONT_WEIGHT, LETTER_SPACING, LINE_HEIGHT, LINE_STYLE_TYPE,
    TEXT_COLOR, TEXT_DECORATION_COLOR, TEXT_DECORATION_THICKNESS, TEXT_INDENT,
    TEXT_UNDERLINE_OFFSET,
};

#[derive(Debug, PartialEq, Hash)]
pub struct FontFamily<'a>(pub &'a str);

impl<'a> FontFamily<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &FONT_FAMILY)?;
        Ok(Decl::String(format!("font-family: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct FontSize<'a>(pub &'a str);

impl<'a> FontSize<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_tuple_value(self.0, &FONT_SIZE)?;

        if FONT_SIZE.contains_key(self.0) {
            Ok(Decl::Double([
                format!("font-size: {}", value.0),
                format!("line-height: {}", value.1),
            ]))
        } else {
            Ok(Decl::String(format!("font-size: {}", value.0)))
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
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
            Self::Antialiased => Decl::Double([
                "-webkit-font-smoothing: antialiased".into(),
                "-moz-osx-font-smoothing: grayscale".into(),
            ]),
            Self::SubpixelAntialiased => Decl::Double([
                "-webkit-font-smoothing: auto".into(),
                "-moz-osx-font-smoothing: auto".into(),
            ]),
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum FontStyle {
    Italic,
    NonItalic,
}

impl FontStyle {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "italic" => Self::Italic,
            "not-italic" => Self::NonItalic,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            Self::Italic => "italic",
            Self::NonItalic => "normal",
        };

        Decl::String(format!("font-style: {}", value))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct FontWeight<'a>(pub &'a str);

impl<'a> FontWeight<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &FONT_WEIGHT)?;
        Ok(Decl::String(format!("font-weight: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
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
            Self::NormalNums => return Decl::Lit("font-variant-numeric: normal"),
            Self::Ordinal => "--tw-ordinal: ordinal",
            Self::SlashedZero => "--tw-slashed-zero: slashed-zero",
            Self::LiningNums => "--tw-numeric-figure: lining-nums",
            Self::OldstyleNums => "--tw-numeric-figure: oldstyle-nums",
            Self::ProportialNums => "--tw-numeric-spacing: proportional-nums",
            Self::TabularNums => "--tw-numeric-spacing: tabular-nums",
            Self::DiagonalFractions => "--tw-numeric-fraction: diagonal-fractions",
            Self::StackedFractions => "--tw-numeric-fraction: stacked-fractions",
        };

        Decl::Double([
            value.into(),
            "font-variant-numeric: var(--tw-ordinal) var(--tw-slashed-zero)
        var(--tw-numeric-figure) var(--tw-numeric-spacing)
        var(--tw-numeric-fraction)"
                .into(),
        ])
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct LetterSpacing<'a>(pub &'a str, bool);

impl<'a> LetterSpacing<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Self {
        let negative = name.starts_with('-');
        Self(arg, negative)
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value_neg(self.1, self.0, &LETTER_SPACING)?;
        Ok(Decl::String(format!("letter-spacing: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct LineHeight<'a>(pub &'a str);

impl<'a> LineHeight<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &LINE_HEIGHT)?;
        Ok(Decl::String(format!("line-height: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct LineStyleType<'a>(pub &'a str);

impl<'a> LineStyleType<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &LINE_STYLE_TYPE)?;
        Ok(Decl::String(format!("list-style-type: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
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
            Self::Inside => "inside",
            Self::Outside => "outside",
        };

        Decl::String(format!("list-style-position: {}", value))
    }
}

#[derive(Debug, PartialEq, Hash)]
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
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
            Self::Justify => "justify",
            Self::Start => "start",
            Self::End => "end",
        };

        Decl::String(format!("text-align: {}", value))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct TextColor<'a>(pub &'a str);

impl<'a> TextColor<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &TEXT_COLOR)?;

        if let Some(color) = hex_to_rgb_color(&value) {
            Ok(Decl::Double([
                "--tw-text-opacity: 1".into(),
                format!(
                    "color: rgb({} {} {} / var(--tw-text-opacity))",
                    color[0], color[1], color[2]
                ),
            ]))
        } else {
            return Ok(Decl::String(format!("color: {}", value)));
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum TextDecoration {
    Underline,
    Overline,
    LineThrough,
    NoUnderline,
}

impl TextDecoration {
    pub fn new(arg: &str) -> Option<Self> {
        match arg {
            "underline" => Some(Self::Underline),
            "overline" => Some(Self::Overline),
            "line-through" => Some(Self::LineThrough),
            "no-underline" => Some(Self::NoUnderline),
            _ => None,
        }
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            Self::Underline => "underline",
            Self::Overline => "overline",
            Self::LineThrough => "line-through",
            Self::NoUnderline => "none",
        };

        Decl::Double([
            format!("-webkit-text-decoration-line: {}", value),
            format!("text-decoration-line: {}", value),
        ])
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct TextDecorationColor<'a>(pub &'a str);

impl<'a> TextDecorationColor<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &TEXT_DECORATION_COLOR)?;
        Ok(Decl::Double([
            format!("-webkit-text-decoration-color: {}", value),
            format!("text-decoration-color: {}", value),
        ]))
    }
}

#[derive(Debug, PartialEq, Hash)]
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
            Self::Solid => "solid",
            Self::Double => "double",
            Self::Dotted => "dotted",
            Self::Dashed => "dashed",
            Self::Wavy => "wavy",
        };

        Decl::Double([
            format!("-webkit-text-decoration-style: {}", value),
            format!("text-decoration-style: {}", value),
        ])
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct TextDecorationThickness<'a>(pub &'a str);

impl<'a> TextDecorationThickness<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &TEXT_DECORATION_THICKNESS)?;
        Ok(Decl::String(format!(
            "text-decoration-thickness: {}",
            value
        )))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct TextUnderlineOffset<'a>(pub &'a str);

impl<'a> TextUnderlineOffset<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &TEXT_UNDERLINE_OFFSET)?;
        Ok(Decl::String(format!("text-underline-offset: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
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
            Self::Uppercase => "uppercase",
            Self::Lowercase => "lowercase",
            Self::Capitalize => "capitalize",
            Self::NormalCase => "none",
        };

        Decl::String(format!("text-transform: {}", value))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum TextOverflow {
    Truncate,
    TextEllipsis,
    TextClip,
}

impl TextOverflow {
    pub fn new(arg: &str) -> Option<Self> {
        let value = match arg {
            "truncate" => Self::Truncate,
            "ellipsis" => Self::TextEllipsis,
            "clip" => Self::TextClip,
            _ => return None,
        };

        Some(value)
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            Self::Truncate => {
                return Decl::Triple([
                    "overflow: hidden".into(),
                    "text-overflow: ellipsis".into(),
                    "white-space: nowrap".into(),
                ])
            }
            Self::TextEllipsis => "ellipsis",
            Self::TextClip => "clip",
        };

        Decl::String(format!("text-overflow: {}", value))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct TextIndent<'a>(pub &'a str, pub bool);

impl<'a> TextIndent<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Self {
        let negative = name.starts_with('-');
        Self(arg, negative)
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value_neg(self.1, self.0, &TEXT_INDENT)?;
        Ok(Decl::String(format!("text-indent: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
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
    pub fn new(arg: &str) -> Result<Self, WarningType> {
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
                    return Err(WarningType::InvalidArg(
                        arg.into(),
                        "Vertical Align".into(),
                        vec![
                            "baseline",
                            "top",
                            "middle",
                            "bottom",
                            "text-top",
                            "text-bottom",
                            "sub",
                            "super",
                        ],
                    ));
                }
            }
        };

        Ok(value)
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = match self {
            Self::Baseline => "baseline",
            Self::Top => "top",
            Self::Middle => "middle",
            Self::Bottom => "bottom",
            Self::TextTop => "text-top",
            Self::TextBottom => "text-bottom",
            Self::Sub => "sub",
            Self::Super => "super",
            Self::Arbitrary(a) => return Ok(Decl::String(format!("vertical-align: {}", a))),
        };

        Ok(Decl::String(format!("vertical-align: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum Whitespace {
    Normal,
    NoWrap,
    Pre,
    PreLine,
    PreWrap,
}

impl Whitespace {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "normal" => Ok(Self::Normal),
            "nowrap" => Ok(Self::NoWrap),
            "pre" => Ok(Self::Pre),
            "pre-line" => Ok(Self::PreLine),
            "pre-wrap" => Ok(Self::PreWrap),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Whitespace".into(),
                vec!["normal", "nowrap", "pre", "pre-line", "pre-wrap"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            Self::Normal => "normal",
            Self::NoWrap => "nowrap",
            Self::Pre => "pre",
            Self::PreLine => "pre-line",
            Self::PreWrap => "pre-wrap",
        };

        Decl::String(format!("white-space: {}", value))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum WordBreak {
    Normal,
    Words,
    All,
    Keep,
}

impl WordBreak {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "normal" => Ok(Self::Normal),
            "words" => Ok(Self::Words),
            "all" => Ok(Self::All),
            "keep" => Ok(Self::Keep),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Word Break".into(),
                vec!["normal", "words", "all", "keep"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        let value = match self {
            Self::Normal => {
                return Decl::Double(["overflow-wrap: normal".into(), "word-break: normal".into()])
            }
            Self::Words => return Decl::Lit("overflow-wrap: break-word"),
            Self::All => "break-all",
            Self::Keep => "keep-all",
        };

        Decl::String(format!("word-break: {}", value))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct Content<'a>(pub &'a str);

impl<'a> Content<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &CONTENT)?;
        Ok(Decl::Double([
            format!("--tw-content: {}", value),
            "content: var(--tw-content)".into(),
        ]))
    }
}
