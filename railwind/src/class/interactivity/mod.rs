mod types;

use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name, get_opt_args};

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

#[derive(Debug)]
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
    UserSelect(UserSelect<'a>),
    WillChange(WillChange<'a>),
}

impl<'a> Interactivity<'a> {
    pub fn new(value: &'a str) -> Option<Self> {
        let class_name = get_class_name(value);

        let interactivity = match class_name {
            "accent" => Interactivity::AccentColor(AccentColor(get_args(value)?)),
            "appearance" => Interactivity::Appearance(Appearance::new(get_args(value)?)?),
            "cursor" => Interactivity::Cursor(Cursor(get_args(value)?)),
            "caret" => Interactivity::CaretColor(CaretColor(get_args(value)?)),
            "pointer" => {
                let args = get_args(value)?;
                match get_class_name(args) {
                    "events" => {
                        Interactivity::PointerEvents(PointerEvents::new(get_opt_args(args))?)
                    }
                    _ => return None,
                }
            }
            "resize" => Interactivity::Resize(Resize::new(value)?),
            "scroll" | "-scroll" => {
                if let Some(scroll) = ScrollBehavior::new(get_args(value)?) {
                    Interactivity::ScrollBehavior(scroll)
                } else if let Some(scroll) = ScrollMargin::new(class_name, get_args(value)?) {
                    Interactivity::ScrollMargin(scroll)
                } else if let Some(scroll) = ScrollPadding::new(class_name, get_args(value)?) {
                    Interactivity::ScrollPadding(scroll)
                } else {
                    return None;
                }
            }
            "snap" => {
                if let Some(snap) = ScrollSnapAlign::new(get_args(value)?) {
                    Interactivity::ScrollSnapAlign(snap)
                } else if let Some(scroll) = ScrollSnapStop::new(get_args(value)?) {
                    Interactivity::ScrollSnapStop(scroll)
                } else if let Some(scroll) = ScrollSnapType::new(get_args(value)?) {
                    Interactivity::ScrollSnapType(scroll)
                } else {
                    return None;
                }
            }
            "touch" => Interactivity::TouchAction(TouchAction::new(get_args(value)?)?),
            "select" => Interactivity::UserSelect(UserSelect::new(get_args(value)?)?),
            "will" => Interactivity::WillChange(WillChange::new(get_args(value)?)?),
            _ => return None,
        };

        Some(interactivity)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Interactivity::AccentColor(s) => s.to_decl(),
            Interactivity::Appearance(s) => s.to_decl(),
            Interactivity::Cursor(s) => s.to_decl(),
            Interactivity::CaretColor(s) => s.to_decl(),
            Interactivity::PointerEvents(s) => s.to_decl(),
            Interactivity::Resize(s) => s.to_decl(),
            Interactivity::ScrollBehavior(s) => s.to_decl(),
            Interactivity::ScrollMargin(s) => s.to_decl(),
            Interactivity::ScrollPadding(s) => s.to_decl(),
            Interactivity::ScrollSnapAlign(s) => s.to_decl(),
            Interactivity::ScrollSnapStop(s) => s.to_decl(),
            Interactivity::ScrollSnapType(s) => s.to_decl(),
            Interactivity::TouchAction(s) => s.to_decl(),
            Interactivity::UserSelect(s) => s.to_decl(),
            Interactivity::WillChange(s) => s.to_decl(),
        }
    }
}
