mod types;

use types::*;

use lazy_static::lazy_static;
use std::collections::HashMap;

use super::{utils::value_is_size, Decl};
use crate::{
    utils::{get_args, get_class_name},
    warning::WarningType,
};

lazy_static! {
    pub static ref FONT_FAMILY: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("font_family.ron")).unwrap();
    pub static ref FONT_SIZE: HashMap<&'static str, (&'static str, &'static str)> =
        ron::from_str(include_str!("font_size.ron")).unwrap();
    pub static ref FONT_WEIGHT: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("font_weight.ron")).unwrap();
    pub static ref LETTER_SPACING: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("letter_spacing.ron")).unwrap();
    pub static ref LINE_HEIGHT: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("line_height.ron")).unwrap();
    pub static ref LINE_STYLE_TYPE: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("line_style_type.ron")).unwrap();
    pub static ref TEXT_COLOR: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("../colors.ron")).unwrap();
    pub static ref TEXT_DECORATION_COLOR: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("../colors.ron")).unwrap();
    pub static ref TEXT_DECORATION_THICKNESS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("text_decoration_thickness.ron")).unwrap();
    pub static ref TEXT_UNDERLINE_OFFSET: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("text_underline_offset.ron")).unwrap();
    pub static ref TEXT_INDENT: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("text_indent.ron")).unwrap();
    pub static ref CONTENT: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("content.ron")).unwrap();
}

#[derive(Debug)]
pub enum Typography<'a> {
    FontFamily(FontFamily<'a>),
    FontSize(FontSize<'a>),
    FontSmoothing(FontSmoothing),
    FontStyle(FontStyle),
    FontWeight(FontWeight<'a>),
    FontVariantNumeric(FontVariantNumeric),
    LetterSpacing(LetterSpacing<'a>),
    LineHeight(LineHeight<'a>),
    LineStyleType(LineStyleType<'a>),
    ListStylePosition(ListStylePosition),
    TextAlign(TextAlign),
    TextColor(TextColor<'a>),
    TextDecoration(TextDecoration),
    TextDecorationColor(TextDecorationColor<'a>),
    TextDecorationStyle(TextDecorationStyle),
    TextDecorationThickness(TextDecorationThickness<'a>),
    TextUnderlineOffset(TextUnderlineOffset<'a>),
    TextTransform(TextTransform),
    TextOverflow(TextOverflow),
    TextIndent(TextIndent<'a>),
    VerticalAlign(VerticalAlign),
    Whitespace(Whitespace),
    WordBreak(WordBreak),
    Content(Content<'a>),
}

impl<'a> Typography<'a> {
    pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
        let typography = match get_class_name(value) {
            "font" => {
                let args = get_args(value)?;
                if FONT_FAMILY.contains_key(args)
                    || (args.starts_with("['") && args.ends_with("']"))
                {
                    Self::FontFamily(FontFamily(get_args(value)?))
                } else {
                    Self::FontWeight(FontWeight(get_args(value)?))
                }
            }
            "text" => {
                if let Some(text_align) = TextAlign::new(get_args(value)?) {
                    Self::TextAlign(text_align)
                } else if let Some(text_overflow) = TextOverflow::new(get_args(value)?) {
                    Self::TextOverflow(text_overflow)
                } else if FONT_SIZE.contains_key(get_args(value)?)
                    || value_is_size(get_args(value)?)
                {
                    Self::FontSize(FontSize(get_args(value)?))
                } else {
                    Self::TextColor(TextColor(get_args(value)?))
                }
            }
            "tracking" => Self::LetterSpacing(LetterSpacing::new(
                get_class_name(value),
                get_args(value)?,
            )),
            "leading" => Self::LineHeight(LineHeight(get_args(value)?)),
            "list" => {
                if let Some(list_style_position) = ListStylePosition::new(get_args(value)?) {
                    Self::ListStylePosition(list_style_position)
                } else {
                    Self::LineStyleType(LineStyleType(get_args(value)?))
                }
            }
            "decoration" => {
                if let Some(text_decoration) = TextDecoration::new(get_args(value)?) {
                    Self::TextDecoration(text_decoration)
                } else if let Some(text_decoration_style) =
                    TextDecorationStyle::new(get_args(value)?)
                {
                    Self::TextDecorationStyle(text_decoration_style)
                } else if TEXT_DECORATION_THICKNESS.contains_key(get_args(value)?)
                    || value_is_size(get_args(value)?)
                {
                    Self::TextDecorationThickness(TextDecorationThickness(get_args(value)?))
                } else {
                    Self::TextDecorationColor(TextDecorationColor(get_args(value)?))
                }
            }
            "underline" => {
                if let Ok(args) = get_args(value) {
                    if get_class_name(args) == "offset" {
                        Self::TextUnderlineOffset(TextUnderlineOffset(get_args(args)?))
                    } else {
                        return Err(WarningType::InvalidArg(
                            args.into(),
                            "Text Underline Offset".into(),
                            vec!["offset"],
                        ));
                    }
                } else {
                    Self::TextDecoration(TextDecoration::Underline)
                }
            }
            "indent" => {
                Self::TextIndent(TextIndent::new(get_class_name(value), get_args(value)?))
            }
            "align" => Self::VerticalAlign(VerticalAlign::new(get_args(value)?)?),
            "whitespace" => Self::Whitespace(Whitespace::new(get_args(value)?)?),
            "break" => Self::WordBreak(WordBreak::new(get_args(value)?)?),
            "content" => Self::Content(Content(get_args(value)?)),
            _ => {
                if let Some(font_smoothing) = FontSmoothing::new(value) {
                    Self::FontSmoothing(font_smoothing)
                } else if let Some(font_style) = FontStyle::new(value) {
                    Self::FontStyle(font_style)
                } else if let Some(font_variant_numeric) = FontVariantNumeric::new(value) {
                    Self::FontVariantNumeric(font_variant_numeric)
                } else if let Some(text_decoration) = TextDecoration::new(value) {
                    Self::TextDecoration(text_decoration)
                } else if let Some(text_transform) = TextTransform::new(value) {
                    Self::TextTransform(text_transform)
                } else if let Some(text_overflow) = TextOverflow::new(value) {
                    Self::TextOverflow(text_overflow)
                } else {
                    return Ok(None);
                }
            }
        };

        Ok(Some(typography))
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::FontFamily(t) => t.to_decl(),
            Self::FontSize(t) => t.to_decl(),
            Self::FontSmoothing(t) => Ok(t.to_decl()),
            Self::FontStyle(t) => Ok(t.to_decl()),
            Self::FontWeight(t) => t.to_decl(),
            Self::FontVariantNumeric(t) => Ok(t.to_decl()),
            Self::LetterSpacing(t) => t.to_decl(),
            Self::LineHeight(t) => t.to_decl(),
            Self::LineStyleType(t) => t.to_decl(),
            Self::ListStylePosition(t) => Ok(t.to_decl()),
            Self::TextAlign(t) => Ok(t.to_decl()),
            Self::TextColor(t) => t.to_decl(),
            Self::TextDecoration(t) => Ok(t.to_decl()),
            Self::TextDecorationColor(t) => t.to_decl(),
            Self::TextDecorationStyle(t) => Ok(t.to_decl()),
            Self::TextDecorationThickness(t) => t.to_decl(),
            Self::TextUnderlineOffset(t) => t.to_decl(),
            Self::TextTransform(t) => Ok(t.to_decl()),
            Self::TextOverflow(t) => Ok(t.to_decl()),
            Self::TextIndent(t) => t.to_decl(),
            Self::VerticalAlign(t) => t.to_decl(),
            Self::Whitespace(t) => Ok(t.to_decl()),
            Self::WordBreak(t) => Ok(t.to_decl()),
            Self::Content(t) => t.to_decl(),
        }
    }
}
