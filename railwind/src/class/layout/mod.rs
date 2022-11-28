mod types;

pub use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name};

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

#[derive(Debug)]
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
    pub fn new(value: &'a str) -> Option<Self> {
        let layout = match get_class_name(value) {
            "aspect" => Layout::AspectRatio(AspectRatio(get_args(value)?)),
            "container" => Layout::Container(Container),
            "columns" => Layout::Columns(Columns(get_args(value)?)),
            "break" => {
                if let Some(args) = get_args(value) {
                    match get_class_name(args) {
                        "after" => Layout::BreakAfter(BreakAfter::new(get_args(args)?)?),
                        "before" => Layout::BreakBefore(BreakBefore::new(get_args(args)?)?),
                        "inside" => Layout::BreakInside(BreakInside::new(get_args(args)?)?),
                        _ => return None,
                    }
                } else {
                    return None;
                }
            }
            "box" => {
                if let Some(args) = get_args(value) {
                    match get_class_name(args) {
                        "decoration" => Layout::BoxDecoration(BoxDecoration::new(get_args(args)?)?),
                        _ => Layout::BoxSizing(BoxSizing::new(args)?),
                    }
                } else {
                    return None;
                }
            }
            "float" => Layout::Floats(Floats::new(get_args(value)?)?),
            "clear" => Layout::Clear(Clear::new(get_args(value)?)?),
            "object" => {
                if let Some(object_fit) = ObjectFit::new(get_args(value)?) {
                    Layout::ObjectFit(object_fit)
                } else {
                    Layout::ObjectPosition(ObjectPosition(get_args(value)?))
                }
            }
            "overflow" => Layout::Overflow(Overflow::new(get_args(value)?)?),
            "overscroll" => Layout::Overscroll(Overscroll::new(get_args(value)?)?),
            "z" => Layout::ZIndex(ZIndex::new(get_class_name(value), get_args(value)?)),
            _ => {
                if let Some(display) = Display::new(value) {
                    Layout::Display(display)
                } else if let Some(isolation) = Isolation::new(value) {
                    Layout::Isolation(isolation)
                } else if let Some(position) = Position::new(value) {
                    Layout::Position(position)
                } else if let Some(top_right_bottom_left) =
                    TopRightBottomLeft::new(get_class_name(value), get_args(value)?)
                {
                    Layout::TopRightBottomLeft(top_right_bottom_left)
                } else if let Some(visibility) = Visibility::new(value) {
                    Layout::Visibility(visibility)
                } else {
                    return None;
                }
            }
        };

        Some(layout)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Layout::AspectRatio(l) => l.to_decl(),
            Layout::Container(l) => Some(l.to_decl()),
            Layout::Columns(l) => l.to_decl(),
            Layout::BreakAfter(l) => Some(l.to_decl()),
            Layout::BreakBefore(l) => Some(l.to_decl()),
            Layout::BreakInside(l) => Some(l.to_decl()),
            Layout::BoxDecoration(l) => Some(l.to_decl()),
            Layout::BoxSizing(l) => Some(l.to_decl()),
            Layout::Display(l) => Some(l.to_decl()),
            Layout::Floats(l) => Some(l.to_decl()),
            Layout::Clear(l) => Some(l.to_decl()),
            Layout::Isolation(l) => Some(l.to_decl()),
            Layout::ObjectFit(l) => Some(l.to_decl()),
            Layout::ObjectPosition(l) => l.to_decl(),
            Layout::Overflow(l) => Some(l.to_decl()),
            Layout::Overscroll(l) => Some(l.to_decl()),
            Layout::Position(l) => Some(l.to_decl()),
            Layout::TopRightBottomLeft(l) => l.to_decl(),
            Layout::Visibility(l) => Some(l.to_decl()),
            Layout::ZIndex(l) => l.to_decl(),
        }
    }
}
