mod types;

pub use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name, get_opt_args};
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref ASPECT_RATIO: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("aspect_ratio.ron")).unwrap();
    pub static ref COLUMNS: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("columns.ron")).unwrap();
    pub static ref OBJECT_POSITION: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("object_position.ron")).unwrap();
    pub static ref INSET: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("top_right_bottom_left.ron")).unwrap();
    pub static ref TOP: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("top_right_bottom_left.ron")).unwrap();
    pub static ref RIGHT: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("top_right_bottom_left.ron")).unwrap();
    pub static ref BOTTOM: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("top_right_bottom_left.ron")).unwrap();
    pub static ref LEFT: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("top_right_bottom_left.ron")).unwrap();
    pub static ref Z_INDEX: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("z_index.ron")).unwrap();
}

#[derive(Debug, PartialEq, Hash)]
pub enum Layout<'a> {
    AspectRatio(AspectRatio<'a>),
    Container(Container),
    Columns(Columns<'a>),
    BreakAfter(BreakAfter),
    BreakBefore(BreakBefore),
    BreakInside(BreakInside),
    BoxDecoration(BoxDecoration),
    BoxSizing(BoxSizing),
    Display(Display),
    Floats(Floats),
    Clear(Clear),
    Isolation(Isolation),
    ObjectFit(ObjectFit),
    ObjectPosition(ObjectPosition<'a>),
    Overflow(Overflow),
    Overscroll(Overscroll),
    Position(Position),
    TopRightBottomLeft(TopRightBottomLeft<'a>),
    Visibility(Visibility),
    ZIndex(ZIndex<'a>),
}

impl<'a> Layout<'a> {
    pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
        let layout = match get_class_name(value) {
            "aspect" => Self::AspectRatio(AspectRatio(get_args(value)?)),
            "container" => Self::Container(Container),
            "columns" => Self::Columns(Columns(get_args(value)?)),
            "break" => {
                let args = get_args(value)?;

                match get_class_name(args) {
                    "after" => Self::BreakAfter(BreakAfter::new(get_args(args)?)?),
                    "before" => Self::BreakBefore(BreakBefore::new(get_args(args)?)?),
                    "inside" => Self::BreakInside(BreakInside::new(get_args(args)?)?),
                    _ => return Ok(None),
                }
            }

            "box" => {
                let args = get_args(value)?;
                match get_class_name(args) {
                    "decoration" => Self::BoxDecoration(BoxDecoration::new(get_args(args)?)?),
                    _ => Self::BoxSizing(BoxSizing::new(args)?),
                }
            }
            "float" => Self::Floats(Floats::new(get_args(value)?)?),
            "clear" => Self::Clear(Clear::new(get_args(value)?)?),
            "object" => {
                if let Some(object_fit) = ObjectFit::new(get_args(value)?) {
                    Self::ObjectFit(object_fit)
                } else {
                    Self::ObjectPosition(ObjectPosition(get_args(value)?))
                }
            }
            "overflow" => Self::Overflow(Overflow::new(get_args(value)?)?),
            "overscroll" => Self::Overscroll(Overscroll::new(get_args(value)?)?),
            "z" | "-z" => Self::ZIndex(ZIndex::new(get_class_name(value), get_args(value)?)),
            _ => {
                if let Some(display) = Display::new(value) {
                    Self::Display(display)
                } else if let Some(isolation) = Isolation::new(value) {
                    Self::Isolation(isolation)
                } else if let Some(position) = Position::new(value) {
                    Self::Position(position)
                } else if let Some(top_right_bottom_left) =
                    TopRightBottomLeft::new(get_class_name(value), get_opt_args(value))
                {
                    Self::TopRightBottomLeft(top_right_bottom_left)
                } else if let Some(visibility) = Visibility::new(value) {
                    Self::Visibility(visibility)
                } else {
                    return Ok(None);
                }
            }
        };

        Ok(Some(layout))
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::AspectRatio(l) => l.to_decl(),
            Self::Container(l) => Ok(l.to_decl()),
            Self::Columns(l) => l.to_decl(),
            Self::BreakAfter(l) => Ok(l.to_decl()),
            Self::BreakBefore(l) => Ok(l.to_decl()),
            Self::BreakInside(l) => Ok(l.to_decl()),
            Self::BoxDecoration(l) => Ok(l.to_decl()),
            Self::BoxSizing(l) => Ok(l.to_decl()),
            Self::Display(l) => Ok(l.to_decl()),
            Self::Floats(l) => Ok(l.to_decl()),
            Self::Clear(l) => Ok(l.to_decl()),
            Self::Isolation(l) => Ok(l.to_decl()),
            Self::ObjectFit(l) => Ok(l.to_decl()),
            Self::ObjectPosition(l) => l.to_decl(),
            Self::Overflow(l) => Ok(l.to_decl()),
            Self::Overscroll(l) => Ok(l.to_decl()),
            Self::Position(l) => Ok(l.to_decl()),
            Self::TopRightBottomLeft(l) => l.to_decl(),
            Self::Visibility(l) => Ok(l.to_decl()),
            Self::ZIndex(l) => l.to_decl(),
        }
    }
}
