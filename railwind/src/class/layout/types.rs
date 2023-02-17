use crate::class::utils::{get_value, get_value_neg};
use crate::class::{Decl, Z_INDEX};
use crate::utils::{get_args, get_class_name};
use crate::warning::WarningType;

use super::{ASPECT_RATIO, BOTTOM, COLUMNS, INSET, LEFT, OBJECT_POSITION, RIGHT, TOP};

#[derive(Debug)]
pub struct AspectRatio<'a>(pub &'a str);

impl<'a> AspectRatio<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &ASPECT_RATIO)?;
        Ok(Decl::Single(format!("aspect-ratio: {}", value)))
    }
}

#[derive(Debug)]
pub struct Container;

impl Container {
    pub fn to_decl(self) -> Decl {
        Decl::FullClass(
            r#".container {
    width: 100%;
}

@media (min-width: 640px) {
    .container {
        max-width: 640px;
    }
}

@media (min-width: 768px) {
    .container {
        max-width: 768px;
    }
}

@media (min-width: 1024px) {
    .container {
        max-width: 1024px;
    }
}

@media (min-width: 1280px) {
    .container {
        max-width: 1280px;
    }
}

@media (min-width: 1536px) {
    .container {
        max-width: 1536px;
    }
}"#
            .into(),
        )
    }
}

#[derive(Debug)]
pub struct Columns<'a>(pub &'a str);

impl<'a> Columns<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &COLUMNS)?;
        Ok(Decl::Single(format!("columns: {}", value)))
    }
}

#[derive(Debug)]
pub enum BreakAfter {
    Auto,
    Avoid,
    All,
    AvoidPage,
    Page,
    Left,
    Right,
    Column,
}

impl BreakAfter {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        let val = match arg {
            "auto" => Self::Auto,
            "avoid" => Self::Avoid,
            "all" => Self::All,
            "avoid-page" => Self::AvoidPage,
            "page" => Self::Page,
            "left" => Self::Left,
            "right" => Self::Right,
            "column" => Self::Column,
            _ => {
                return Err(WarningType::InvalidArg(
                    arg.into(),
                    "Break After".into(),
                    vec![
                        "auto",
                        "avoid",
                        "all",
                        "avoid-page",
                        "page",
                        "left",
                        "right",
                        "column",
                    ],
                ))
            }
        };

        Ok(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Auto => "auto",
            Self::Avoid => "avoid",
            Self::All => "all",
            Self::AvoidPage => "avoid-page",
            Self::Page => "page",
            Self::Left => "left",
            Self::Right => "right",
            Self::Column => "column",
        };

        Decl::Single(format!("break-after: {}", val))
    }
}

#[derive(Debug)]
pub enum BreakBefore {
    Auto,
    Avoid,
    All,
    AvoidPage,
    Page,
    Left,
    Right,
    Column,
}

impl BreakBefore {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        let val = match arg {
            "auto" => Self::Auto,
            "avoid" => Self::Avoid,
            "all" => Self::All,
            "avoid-page" => Self::AvoidPage,
            "page" => Self::Page,
            "left" => Self::Left,
            "right" => Self::Right,
            "column" => Self::Column,
            _ => {
                return Err(WarningType::InvalidArg(
                    arg.into(),
                    "Break Before".into(),
                    vec![
                        "auto",
                        "avoid",
                        "all",
                        "avoid-page",
                        "page",
                        "left",
                        "right",
                        "column",
                    ],
                ))
            }
        };

        Ok(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Auto => "auto",
            Self::Avoid => "avoid",
            Self::All => "all",
            Self::AvoidPage => "avoid-page",
            Self::Page => "page",
            Self::Left => "left",
            Self::Right => "right",
            Self::Column => "column",
        };

        Decl::Single(format!("break-before: {}", val))
    }
}

#[derive(Debug)]
pub enum BreakInside {
    Auto,
    Avoid,
    AvoidPage,
    AvoidColumn,
}

impl BreakInside {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        let val = match arg {
            "auto" => Self::Auto,
            "avoid" => Self::Avoid,
            "avoid-page" => Self::AvoidPage,
            "avoid-column" => Self::AvoidColumn,
            _ => {
                return Err(WarningType::InvalidArg(
                    arg.into(),
                    "Break Inside".into(),
                    vec!["auto", "avoid", "avoid-page", "avoid-column"],
                ))
            }
        };

        Ok(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Auto => "auto",
            Self::Avoid => "avoid",
            Self::AvoidPage => "avoid-page",
            Self::AvoidColumn => "avoid-column",
        };

        Decl::Single(format!("break-inside: {}", val))
    }
}

#[derive(Debug)]
pub enum BoxDecoration {
    Clone,
    Slice,
}

impl BoxDecoration {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "clone" => Ok(Self::Clone),
            "slice" => Ok(Self::Slice),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Box Decoration".into(),
                vec!["clone", "slice"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Clone => "clone",
            Self::Slice => "slice",
        };

        Decl::Double([
            format!("-webkit-box-decoration-break: {}", val),
            format!("box-decoration-break: {}", val),
        ])
    }
}

#[derive(Debug)]
pub enum BoxSizing {
    Border,
    Content,
}

impl BoxSizing {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "border" => Ok(Self::Border),
            "content" => Ok(Self::Content),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Box Sizing".into(),
                vec!["border", "content"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Border => "border-box",
            Self::Content => "content-box",
        };

        Decl::Single(format!("box-sizing: {}", val))
    }
}

#[derive(Debug)]
pub enum Display {
    Block,
    InlineBlock,
    Inline,
    Flex,
    InlineFlex,
    Table,
    InlineTable,
    TableCaption,
    TableCell,
    TableColumn,
    TableColumnGroup,
    TableFooterGroup,
    TableHeaderGroup,
    TableRowGroup,
    TableRow,
    FlowRoot,
    Grid,
    InlineGrid,
    Contents,
    ListItem,
    Hidden,
}

impl Display {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "block" => Self::Block,
            "inline-block" => Self::InlineBlock,
            "inline" => Self::Inline,
            "flex" => Self::Flex,
            "inline-flex" => Self::InlineFlex,
            "table" => Self::Table,
            "inline-table" => Self::InlineTable,
            "table-caption" => Self::TableCaption,
            "table-cell" => Self::TableCell,
            "table-column" => Self::TableColumn,
            "table-column-group" => Self::TableColumnGroup,
            "table-footer-group" => Self::TableFooterGroup,
            "table-header-group" => Self::TableHeaderGroup,
            "table-row-group" => Self::TableRowGroup,
            "table-row" => Self::TableRow,
            "flow-root" => Self::FlowRoot,
            "grid" => Self::Grid,
            "inline-grid" => Self::InlineGrid,
            "contents" => Self::Contents,
            "list-item" => Self::ListItem,
            "hidden" => Self::Hidden,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Block => "block",
            Self::InlineBlock => "inline-block",
            Self::Inline => "inline",
            Self::Flex => "flex",
            Self::InlineFlex => "inline-flex",
            Self::Table => "table",
            Self::InlineTable => "inline-table",
            Self::TableCaption => "table-caption",
            Self::TableCell => "table-cell",
            Self::TableColumn => "table-column",
            Self::TableColumnGroup => "table-column-group",
            Self::TableFooterGroup => "table-footer-group",
            Self::TableHeaderGroup => "table-header-group",
            Self::TableRowGroup => "table-row-group",
            Self::TableRow => "table-row",
            Self::FlowRoot => "flow-root",
            Self::Grid => "grid",
            Self::InlineGrid => "inline-grid",
            Self::Contents => "contents",
            Self::ListItem => "list-item",
            Self::Hidden => "none",
        };

        Decl::Single(format!("display: {}", val))
    }
}

#[derive(Debug)]
pub enum Floats {
    Right,
    Left,
    None,
}

impl Floats {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "right" => Ok(Self::Right),
            "left" => Ok(Self::Left),
            "none" => Ok(Self::None),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Floats".into(),
                vec!["right", "left", "none"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Right => "right",
            Self::Left => "left",
            Self::None => "none",
        };

        Decl::Single(format!("float: {}", val))
    }
}

#[derive(Debug)]
pub enum Clear {
    Left,
    Right,
    Both,
    None,
}

impl Clear {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            "both" => Ok(Self::Both),
            "none" => Ok(Self::None),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Clear".into(),
                vec!["left", "right", "both", "none"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Left => "left",
            Self::Right => "right",
            Self::Both => "both",
            Self::None => "none",
        };

        Decl::Single(format!("clear: {}", val))
    }
}

#[derive(Debug)]
pub enum Isolation {
    Isolate,
    IsolationAuto,
}

impl Isolation {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "isolate" => Self::Isolate,
            "isolation-auto" => Self::IsolationAuto,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Isolate => "isolate",
            Self::IsolationAuto => "auto",
        };

        Decl::Single(format!("isolation: {}", val))
    }
}

#[derive(Debug)]
pub enum ObjectFit {
    Contain,
    Cover,
    Fill,
    None,
    ScaleDown,
}

impl ObjectFit {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "contain" => Self::Contain,
            "cover" => Self::Cover,
            "fill" => Self::Fill,
            "none" => Self::None,
            "scale-down" => Self::ScaleDown,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Contain => "contain",
            Self::Cover => "cover",
            Self::Fill => "fill",
            Self::None => "none",
            Self::ScaleDown => "scale-down",
        };

        Decl::Single(format!("object-fit: {}", val))
    }
}

#[derive(Debug)]
pub struct ObjectPosition<'a>(pub &'a str);

impl<'a> ObjectPosition<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &OBJECT_POSITION)?;
        Ok(Decl::Single(format!("object-position: {}", value)))
    }
}

#[derive(Debug)]
pub enum Overflow {
    Auto,
    Hidden,
    Clip,
    Visible,
    Scroll,
    XAuto,
    YAuto,
    XHidden,
    YHidden,
    XClip,
    YClip,
    XVisible,
    YVisible,
    XScroll,
    YScroll,
}

impl Overflow {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        let val = match arg {
            "auto" => Self::Auto,
            "hidden" => Self::Hidden,
            "clip" => Self::Clip,
            "visible" => Self::Visible,
            "scroll" => Self::Scroll,
            "x-auto" => Self::XAuto,
            "y-auto" => Self::YAuto,
            "x-hidden" => Self::XHidden,
            "y-hidden" => Self::YHidden,
            "x-clip" => Self::XClip,
            "y-clip" => Self::YClip,
            "x-visible" => Self::XVisible,
            "y-visible" => Self::YVisible,
            "x-scroll" => Self::XScroll,
            "y-scroll" => Self::YScroll,
            _ => {
                return Err(WarningType::InvalidArg(
                    arg.into(),
                    "Overflow".into(),
                    vec![
                        "auto",
                        "hidden",
                        "clip",
                        "visible",
                        "scroll",
                        "x-auto",
                        "y-auto",
                        "x-hidden",
                        "y-hidden",
                        "x-clip",
                        "y-clip",
                        "x-visible",
                        "y-visible",
                        "x-scroll",
                        "y-scroll",
                    ],
                ))
            }
        };

        Ok(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Auto => ": auto",
            Self::Hidden => ": hidden",
            Self::Clip => ": clip",
            Self::Visible => ": visible",
            Self::Scroll => ": scroll",
            Self::XAuto => "-x: auto",
            Self::YAuto => "-y: auto",
            Self::XHidden => "-x: hidden",
            Self::YHidden => "-y: hidden",
            Self::XClip => "-x: clip",
            Self::YClip => "-y: clip",
            Self::XVisible => "-x: visible",
            Self::YVisible => "-y: visible",
            Self::XScroll => "-x: scroll",
            Self::YScroll => "-y: scroll",
        };

        Decl::Single(format!("overflow{}", val))
    }
}

#[derive(Debug)]
pub enum Overscroll {
    Auto,
    Contain,
    None,
    YAuto,
    YContain,
    YNone,
    XAuto,
    XContain,
    XNone,
}

impl Overscroll {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        let val = match arg {
            "auto" => Self::Auto,
            "contain" => Self::Contain,
            "none" => Self::None,
            "y-auto" => Self::YAuto,
            "y-contain" => Self::YContain,
            "y-none" => Self::YNone,
            "x-auto" => Self::XAuto,
            "x-contain" => Self::XContain,
            "x-none" => Self::XNone,
            _ => {
                return Err(WarningType::InvalidArg(
                    arg.into(),
                    "Overscroll".into(),
                    vec![
                        "auto",
                        "contain",
                        "none",
                        "y-auto",
                        "y-contain",
                        "y-none",
                        "x-auto",
                        "x-contain",
                        "x-none",
                    ],
                ))
            }
        };

        Ok(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Auto => ": auto",
            Self::Contain => ": contain",
            Self::None => ": none",
            Self::YAuto => "-y: auto",
            Self::YContain => "-y: contain",
            Self::YNone => "-y: none",
            Self::XAuto => "-x: auto",
            Self::XContain => "-x: contain",
            Self::XNone => "-x: none",
        };

        Decl::Single(format!("overscroll-behavior{}", val))
    }
}

#[derive(Debug)]
pub enum Position {
    Static,
    Fixed,
    Absolute,
    Relative,
    Sticky,
}

impl Position {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "static" => Self::Static,
            "fixed" => Self::Fixed,
            "absolute" => Self::Absolute,
            "relative" => Self::Relative,
            "sticky" => Self::Sticky,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Static => "static",
            Self::Fixed => "fixed",
            Self::Absolute => "absolute",
            Self::Relative => "relative",
            Self::Sticky => "sticky",
        };

        Decl::Single(format!("position: {}", val))
    }
}

#[derive(Debug)]
pub enum TopRightBottomLeft<'a> {
    Inset(&'a str, bool),
    Top(&'a str, bool),
    Right(&'a str, bool),
    Bottom(&'a str, bool),
    Left(&'a str, bool),
}

impl<'a> TopRightBottomLeft<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Option<Self> {
        let negative = name.starts_with('-');
        let name = if negative { &name[1..] } else { name };

        let val = match name {
            "inset" => Self::Inset(arg, negative),
            "top" => Self::Top(arg, negative),
            "right" => Self::Right(arg, negative),
            "bottom" => Self::Bottom(arg, negative),
            "left" => Self::Left(arg, negative),
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::Inset(arg, neg) => match get_class_name(arg) {
                "x" => {
                    let val = get_value_neg(neg, get_args(arg)?, &INSET)?;

                    Ok(Decl::Double([
                        format!("left: {}", val),
                        format!("right: {}", val),
                    ]))
                }
                "y" => {
                    let val = get_value_neg(neg, get_args(arg)?, &INSET)?;
                    Ok(Decl::Double([
                        format!("top: {}", val),
                        format!("bottom: {}", val),
                    ]))
                }
                _ => {
                    let val = get_value_neg(neg, arg, &INSET)?;
                    Ok(Decl::Quad([
                        format!("top: {}", val),
                        format!("right: {}", val),
                        format!("bottom: {}", val),
                        format!("left: {}", val),
                    ]))
                }
            },
            Self::Top(arg, neg) => {
                let val = get_value_neg(neg, arg, &TOP)?;
                Ok(Decl::Single(format!("top: {}", val)))
            }
            Self::Right(arg, neg) => {
                let val = get_value_neg(neg, arg, &RIGHT)?;
                Ok(Decl::Single(format!("right: {}", val)))
            }
            Self::Bottom(arg, neg) => {
                let val = get_value_neg(neg, arg, &BOTTOM)?;
                Ok(Decl::Single(format!("bottom: {}", val)))
            }
            Self::Left(arg, neg) => {
                let val = get_value_neg(neg, arg, &LEFT)?;
                Ok(Decl::Single(format!("left: {}", val)))
            }
        }
    }
}

#[derive(Debug)]
pub enum Visibility {
    Visible,
    Invisible,
    Collapse,
}

impl Visibility {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "visible" => Self::Visible,
            "invisible" => Self::Invisible,
            "collapse" => Self::Collapse,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Visible => "visible",
            Self::Invisible => "hidden",
            Self::Collapse => "collapse",
        };

        Decl::Single(format!("visibility: {}", val))
    }
}

#[derive(Debug)]
pub struct ZIndex<'a>(pub &'a str, bool);

impl<'a> ZIndex<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Self {
        let negative = name.starts_with('-');
        Self(arg, negative)
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value_neg(self.1, self.0, &Z_INDEX)?;
        Ok(Decl::Single(format!("z-index: {}", value)))
    }
}
