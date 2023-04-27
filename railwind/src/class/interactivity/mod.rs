mod types;

use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name, get_opt_args};
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref COLORS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("../colors.ron")).unwrap();
    pub static ref CURSOR: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("cursor.ron")).unwrap();
    pub static ref MARGIN: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("margin.ron")).unwrap();
    pub static ref PADDING: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("padding.ron")).unwrap();
}

#[derive(Debug, PartialEq, Hash)]
pub enum Interactivity<'a> {
    AccentColor(AccentColor<'a>),
    Appearance(Appearance),
    Cursor(Cursor<'a>),
    CaretColor(CaretColor<'a>),
    PointerEvents(PointerEvents),
    Resize(Resize),
    ScrollBehavior(ScrollBehavior),
    ScrollMargin(ScrollMargin<'a>),
    ScrollPadding(ScrollPadding<'a>),
    ScrollSnapAlign(ScrollSnapAlign),
    ScrollSnapStop(ScrollSnapStop),
    ScrollSnapType(ScrollSnapType),
    TouchAction(TouchAction),
    UserSelect(UserSelect),
    WillChange(WillChange),
}

impl<'a> Interactivity<'a> {
    pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
        let class_name = get_class_name(value);

        let interactivity = match class_name {
            "accent" => Self::AccentColor(AccentColor(get_args(value)?)),
            "appearance" => Self::Appearance(Appearance::new(get_args(value)?)?),
            "cursor" => Self::Cursor(Cursor(get_args(value)?)),
            "caret" => Self::CaretColor(CaretColor(get_args(value)?)),
            "pointer" => {
                let args = get_args(value)?;
                match get_class_name(args) {
                    "events" => Self::PointerEvents(PointerEvents::new(get_opt_args(args))?),
                    v => {
                        return Err(WarningType::InvalidArg(
                            v.into(),
                            "Pointer Events".into(),
                            vec!["events"],
                        ))
                    }
                }
            }
            "resize" => Self::Resize(Resize::new(value)?),
            "scroll" | "-scroll" => {
                if let Some(scroll) = ScrollBehavior::new(get_args(value)?) {
                    Self::ScrollBehavior(scroll)
                } else if let Some(scroll) = ScrollMargin::new(class_name, get_args(value)?) {
                    Self::ScrollMargin(scroll)
                } else if let Some(scroll) = ScrollPadding::new(class_name, get_args(value)?) {
                    Self::ScrollPadding(scroll)
                } else {
                    return Err(WarningType::InvalidArg(
                        value.into(),
                        "Scroll Behavior / Margin / Padding".into(),
                        vec![
                            "auto", "smooth", "m", "mx", "my", "mt", "mr", "mb", "ml", "p", "px",
                            "py", "pt", "pr", "pb", "pl",
                        ],
                    ));
                }
            }
            "snap" => {
                if let Some(snap) = ScrollSnapAlign::new(get_args(value)?) {
                    Self::ScrollSnapAlign(snap)
                } else if let Some(scroll) = ScrollSnapStop::new(get_args(value)?) {
                    Self::ScrollSnapStop(scroll)
                } else if let Some(scroll) = ScrollSnapType::new(get_args(value)?) {
                    Self::ScrollSnapType(scroll)
                } else {
                    return Err(WarningType::InvalidArg(
                        value.into(),
                        "Scroll Snap Align / Stop / Type".into(),
                        vec![
                            "start",
                            "end",
                            "center",
                            "align-none",
                            "normal",
                            "always",
                            "none",
                            "x",
                            "y",
                            "both",
                            "mandatory",
                            "proximity",
                        ],
                    ));
                }
            }
            "touch" => Self::TouchAction(TouchAction::new(get_args(value)?)?),
            "select" => Self::UserSelect(UserSelect::new(get_args(value)?)?),
            "will" => Self::WillChange(WillChange::new(get_args(value)?)?),
            _ => return Ok(None),
        };

        Ok(Some(interactivity))
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::AccentColor(s) => s.to_decl(),
            Self::Appearance(s) => Ok(s.to_decl()),
            Self::Cursor(s) => s.to_decl(),
            Self::CaretColor(s) => s.to_decl(),
            Self::PointerEvents(s) => Ok(s.to_decl()),
            Self::Resize(s) => Ok(s.to_decl()),
            Self::ScrollBehavior(s) => Ok(s.to_decl()),
            Self::ScrollMargin(s) => s.to_decl(),
            Self::ScrollPadding(s) => s.to_decl(),
            Self::ScrollSnapAlign(s) => Ok(s.to_decl()),
            Self::ScrollSnapStop(s) => Ok(s.to_decl()),
            Self::ScrollSnapType(s) => Ok(s.to_decl()),
            Self::TouchAction(s) => Ok(s.to_decl()),
            Self::UserSelect(s) => Ok(s.to_decl()),
            Self::WillChange(s) => Ok(s.to_decl()),
        }
    }
}
