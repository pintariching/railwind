mod types;

use types::*;

use lazy_static::lazy_static;
use std::collections::HashMap;

use super::{utils::value_is_size, Decl};
use crate::utils::{get_args, get_class_name};

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
    pub fn new(value: &'a str) -> Option<Self> {
        let typography = match get_class_name(value) {
            "font" => {
                let args = get_args(value)?;
                if FONT_FAMILY.contains_key(args) || args.starts_with("['") && args.ends_with("']")
                {
                    Typography::FontFamily(FontFamily(get_args(value)?))
                } else {
                    Typography::FontWeight(FontWeight(get_args(value)?))
                }
            }
            "text" => {
                if let Some(text_align) = TextAlign::new(get_args(value)?) {
                    Typography::TextAlign(text_align)
                } else if let Some(text_overflow) = TextOverflow::new(get_args(value)?) {
                    Typography::TextOverflow(text_overflow)
                } else if FONT_SIZE.contains_key(get_args(value)?)
                    || value_is_size(get_args(value)?)
                {
                    Typography::FontSize(FontSize(get_args(value)?))
                } else {
                    Typography::TextColor(TextColor(get_args(value)?))
                }
            }
            "tracking" => Typography::LetterSpacing(LetterSpacing::new(
                get_class_name(value),
                get_args(value)?,
            )),
            "leading" => Typography::LineHeight(LineHeight(get_args(value)?)),
            "list" => {
                if let Some(list_style_position) = ListStylePosition::new(get_args(value)?) {
                    Typography::ListStylePosition(list_style_position)
                } else {
                    Typography::LineStyleType(LineStyleType(get_args(value)?))
                }
            }
            "decoration" => {
                if let Some(text_decoration) = TextDecoration::new(get_args(value)?) {
                    Typography::TextDecoration(text_decoration)
                } else if let Some(text_decoration_style) =
                    TextDecorationStyle::new(get_args(value)?)
                {
                    Typography::TextDecorationStyle(text_decoration_style)
                } else if TEXT_DECORATION_THICKNESS.contains_key(get_args(value)?)
                    || value_is_size(get_args(value)?)
                {
                    Typography::TextDecorationThickness(TextDecorationThickness(get_args(value)?))
                } else {
                    Typography::TextDecorationColor(TextDecorationColor(get_args(value)?))
                }
            }
            "underline" => {
                if let Some(args) = get_args(value) {
                    if get_class_name(args) == "offset" {
                        Typography::TextUnderlineOffset(TextUnderlineOffset(get_args(args)?))
                    } else {
                        return None;
                    }
                } else {
                    Typography::TextDecoration(TextDecoration::new(value)?)
                }
            }
            "indent" => {
                Typography::TextIndent(TextIndent::new(get_class_name(value), get_args(value)?))
            }
            "align" => Typography::VerticalAlign(VerticalAlign::new(get_args(value)?)?),
            "whitespace" => Typography::Whitespace(Whitespace::new(get_args(value)?)?),
            "break" => Typography::WordBreak(WordBreak::new(get_args(value)?)?),
            "content" => Typography::Content(Content(get_args(value)?)),
            _ => {
                if let Some(font_smoothing) = FontSmoothing::new(value) {
                    Typography::FontSmoothing(font_smoothing)
                } else if let Some(font_style) = FontStyle::new(value) {
                    Typography::FontStyle(font_style)
                } else if let Some(font_variant_numeric) = FontVariantNumeric::new(value) {
                    Typography::FontVariantNumeric(font_variant_numeric)
                } else if let Some(text_decoration) = TextDecoration::new(value) {
                    Typography::TextDecoration(text_decoration)
                } else if let Some(text_transform) = TextTransform::new(value) {
                    Typography::TextTransform(text_transform)
                } else if let Some(text_overflow) = TextOverflow::new(value) {
                    Typography::TextOverflow(text_overflow)
                } else {
                    return None;
                }
            }
        };

        Some(typography)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Typography::FontFamily(t) => t.to_decl(),
            Typography::FontSize(t) => t.to_decl(),
            Typography::FontSmoothing(t) => Some(t.to_decl()),
            Typography::FontStyle(t) => Some(t.to_decl()),
            Typography::FontWeight(t) => t.to_decl(),
            Typography::FontVariantNumeric(t) => Some(t.to_decl()),
            Typography::LetterSpacing(t) => t.to_decl(),
            Typography::LineHeight(t) => t.to_decl(),
            Typography::LineStyleType(t) => t.to_decl(),
            Typography::ListStylePosition(t) => Some(t.to_decl()),
            Typography::TextAlign(t) => Some(t.to_decl()),
            Typography::TextColor(t) => t.to_decl(),
            Typography::TextDecoration(t) => Some(t.to_decl()),
            Typography::TextDecorationColor(t) => t.to_decl(),
            Typography::TextDecorationStyle(t) => Some(t.to_decl()),
            Typography::TextDecorationThickness(t) => t.to_decl(),
            Typography::TextUnderlineOffset(t) => t.to_decl(),
            Typography::TextTransform(t) => Some(t.to_decl()),
            Typography::TextOverflow(t) => Some(t.to_decl()),
            Typography::TextIndent(t) => t.to_decl(),
            Typography::VerticalAlign(t) => t.to_decl(),
            Typography::Whitespace(t) => Some(t.to_decl()),
            Typography::WordBreak(t) => Some(t.to_decl()),
            Typography::Content(t) => t.to_decl(),
        }
    }
}
