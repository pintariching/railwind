use crate::class::utils::{get_arbitrary_value, get_value, get_value_neg};
use crate::class::Decl;
use crate::utils::{get_args, get_class_name, get_opt_args};

use super::{
    BASIS, FLEX, GAP, GAP_X, GAP_Y, GRID_AUTO_COLUMNS, GRID_AUTO_ROWS, GRID_COLUMN_END,
    GRID_COLUMN_SPAN, GRID_COLUMN_START, GRID_ROW_END, GRID_ROW_SPAN, GRID_ROW_START,
    GRID_TEMPLATE_COLUMNS, GRID_TEMPLATE_ROWS, GROW, ORDER, SHRINK,
};

#[derive(Debug)]
pub struct Basis<'a>(pub &'a str);

impl<'a> Basis<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &BASIS)?;
        Some(Decl::Single(format!("flex-basis: {}", value)))
    }
}

#[derive(Debug)]
pub enum Direction {
    Row,
    RowReverse,
    Col,
    ColReverse,
}

impl Direction {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "row" => Self::Row,
            "row-reverse" => Self::RowReverse,
            "col" => Self::Col,
            "col-reverse" => Self::ColReverse,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Row => "row",
            Self::RowReverse => "row-reverse",
            Self::Col => "column",
            Self::ColReverse => "column-reverse",
        };

        Decl::Single(format!("flex-direction: {}", val))
    }
}

#[derive(Debug)]
pub enum Wrap {
    Wrap,
    WrapReverse,
    NoWrap,
}

impl Wrap {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "wrap" => Self::Wrap,
            "wrap-reverse" => Self::WrapReverse,
            "nowrap" => Self::NoWrap,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Wrap => "wrap",
            Self::WrapReverse => "wrap-reverse",
            Self::NoWrap => "nowrap",
        };

        Decl::Single(format!("flex-wrap: {}", val))
    }
}

#[derive(Debug)]
pub struct Flex<'a>(pub &'a str);

impl<'a> Flex<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &FLEX)?;
        Some(Decl::Single(format!("flex: {}", value)))
    }
}

#[derive(Debug)]
pub struct Grow<'a>(pub &'a str);

impl<'a> Grow<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &GROW)?;
        Some(Decl::Single(format!("flex-grow: {}", value)))
    }
}

#[derive(Debug)]
pub struct Shrink<'a>(pub &'a str);

impl<'a> Shrink<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &SHRINK)?;
        Some(Decl::Single(format!("flex-shrink: {}", value)))
    }
}

#[derive(Debug)]
pub struct Order<'a>(pub &'a str, bool);

impl<'a> Order<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Self {
        let negative = name.starts_with('-');
        Self(arg, negative)
    }

    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value_neg(self.1, self.0, &ORDER)?;
        Some(Decl::Single(format!("order: {}", value)))
    }
}

#[derive(Debug)]
pub struct GridTemplateColumns<'a>(pub &'a str);

impl<'a> GridTemplateColumns<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &GRID_TEMPLATE_COLUMNS)?;
        Some(Decl::Single(format!("grid-template-columns: {}", value)))
    }
}

#[derive(Debug)]
pub enum GridColumn<'a> {
    Auto,
    Span(&'a str),
    Start(&'a str),
    End(&'a str),
}

impl<'a> GridColumn<'a> {
    pub fn new(args: &'a str) -> Option<Self> {
        let arg = get_args(args)?;

        let val = match get_class_name(args) {
            "auto" => Self::Auto,
            "span" => Self::Span(arg),
            "start" => Self::Start(arg),
            "end" => Self::End(arg),
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Self::Auto => Some(Decl::Lit("grid-column: auto")),
            Self::Span(v) => {
                let value = get_value(v, &GRID_COLUMN_SPAN)?;
                Some(Decl::Single(format!("grid-column: {}", value)))
            }
            Self::Start(v) => {
                let value = get_value(v, &GRID_COLUMN_START)?;
                Some(Decl::Single(format!("grid-column-start: {}", value)))
            }
            Self::End(v) => {
                let value = get_value(v, &GRID_COLUMN_END)?;
                Some(Decl::Single(format!("grid-column-end: {}", value)))
            }
        }
    }
}

#[derive(Debug)]
pub struct GridTepmlateRows<'a>(pub &'a str);

impl<'a> GridTepmlateRows<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &GRID_TEMPLATE_ROWS)?;
        Some(Decl::Single(format!("grid-template-rows: {}", value)))
    }
}

#[derive(Debug)]
pub enum GridRow<'a> {
    Auto,
    Span(&'a str),
    Start(&'a str),
    End(&'a str),
    Arbitrary(&'a str),
}

impl<'a> GridRow<'a> {
    pub fn new(args: &'a str) -> Option<Self> {
        let arg = get_opt_args(args);
        let val = match get_class_name(args) {
            "auto" => Self::Auto,
            "span" => Self::Span(arg),
            "start" => Self::Start(arg),
            "end" => Self::End(arg),
            _ => Self::Arbitrary(args),
        };

        Some(val)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Self::Auto => Some(Decl::Lit("grid-row: auto")),
            Self::Span(v) => {
                let value = get_value(v, &GRID_ROW_SPAN)?;
                Some(Decl::Single(format!("grid-row: {}", value)))
            }
            Self::Start(v) => {
                let value = get_value(v, &GRID_ROW_START)?;
                Some(Decl::Single(format!("grid-row-start: {}", value)))
            }
            Self::End(v) => {
                let value = get_value(v, &GRID_ROW_END)?;
                Some(Decl::Single(format!("grid-row-end: {}", value)))
            }
            Self::Arbitrary(v) => {
                let value = get_arbitrary_value(v)?;
                Some(Decl::Single(format!("grid-row: {}", value)))
            }
        }
    }
}

#[derive(Debug)]
pub enum GridAutoFlow {
    Row,
    Col,
    Dense,
    RowDense,
    ColDense,
}

impl GridAutoFlow {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "row" => Self::Row,
            "col" => Self::Col,
            "dense" => Self::Dense,
            "row-dense" => Self::RowDense,
            "col-dense" => Self::ColDense,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Row => "row",
            Self::Col => "column",
            Self::Dense => "dense",
            Self::RowDense => "row dense",
            Self::ColDense => "column dense",
        };

        Decl::Single(format!("grid-auto-flow: {}", val))
    }
}

#[derive(Debug)]
pub struct GridAutoColumns<'a>(pub &'a str);

impl<'a> GridAutoColumns<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &GRID_AUTO_COLUMNS)?;
        Some(Decl::Single(format!("grid-auto-columns: {}", value)))
    }
}

#[derive(Debug)]
pub struct GridAutoRows<'a>(pub &'a str);

impl<'a> GridAutoRows<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &GRID_AUTO_ROWS)?;
        Some(Decl::Single(format!("grid-auto-rows: {}", value)))
    }
}

#[derive(Debug)]
pub struct Gap<'a>(pub &'a str);

impl<'a> Gap<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        match get_class_name(self.0) {
            "x" => {
                let val = get_value(get_args(self.0)?, &GAP_X)?;
                Some(Decl::Single(format!("column-gap: {}", val)))
            }
            "y" => {
                let val = get_value(get_args(self.0)?, &GAP_Y)?;
                Some(Decl::Single(format!("row-gap: {}", val)))
            }
            _ => {
                let val = get_value(self.0, &GAP)?;
                Some(Decl::Single(format!("gap: {}", val)))
            }
        }
    }
}

#[derive(Debug)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}

impl JustifyContent {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "start" => Self::Start,
            "end" => Self::End,
            "center" => Self::Center,
            "between" => Self::Between,
            "around" => Self::Around,
            "evenly" => Self::Evenly,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Start => "flex-start",
            Self::End => "flex-end",
            Self::Center => "center",
            Self::Between => "space-between",
            Self::Around => "space-around",
            Self::Evenly => "space-evenly",
        };

        Decl::Single(format!("justify-content: {}", val))
    }
}

#[derive(Debug)]
pub enum JustifyItems {
    Start,
    End,
    Center,
    Stretch,
}

impl JustifyItems {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "start" => Self::Start,
            "end" => Self::End,
            "center" => Self::Center,
            "stretch" => Self::Stretch,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::Stretch => "stretch",
        };

        Decl::Single(format!("justify-items: {}", val))
    }
}

#[derive(Debug)]
pub enum JustifySelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}

impl JustifySelf {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "auto" => Self::Auto,
            "start" => Self::Start,
            "end" => Self::End,
            "center" => Self::Center,
            "stretch" => Self::Stretch,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Auto => "auto",
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::Stretch => "stretch",
        };

        Decl::Single(format!("justify-self: {}", val))
    }
}

#[derive(Debug)]
pub enum AlignContent {
    Center,
    Start,
    End,
    Between,
    Around,
    Evenly,
    Baseline,
}

impl AlignContent {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "center" => Self::Center,
            "start" => Self::Start,
            "end" => Self::End,
            "between" => Self::Between,
            "around" => Self::Around,
            "evenly" => Self::Evenly,
            "baseline" => Self::Baseline,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Center => "center",
            Self::Start => "flex-start",
            Self::End => "flex-end",
            Self::Between => "space-between",
            Self::Around => "space-around",
            Self::Evenly => "space-evenly",
            Self::Baseline => "baseline",
        };

        Decl::Single(format!("align-content: {}", val))
    }
}

#[derive(Debug)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl AlignItems {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "start" => Self::Start,
            "end" => Self::End,
            "center" => Self::Center,
            "baseline" => Self::Baseline,
            "stretch" => Self::Stretch,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Start => "flex-start",
            Self::End => "flex-end",
            Self::Center => "center",
            Self::Baseline => "baseline",
            Self::Stretch => "stretch",
        };

        Decl::Single(format!("align-items: {}", val))
    }
}

#[derive(Debug)]
pub enum AlignSelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

impl AlignSelf {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "auto" => Self::Auto,
            "start" => Self::Start,
            "end" => Self::End,
            "center" => Self::Center,
            "stretch" => Self::Stretch,
            "baseline" => Self::Baseline,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Auto => "auto",
            Self::Start => "flex-start",
            Self::End => "flex-end",
            Self::Center => "center",
            Self::Stretch => "stretch",
            Self::Baseline => "baseline",
        };

        Decl::Single(format!("align-self: {}", val))
    }
}

#[derive(Debug)]
pub enum PlaceContent {
    Center,
    Start,
    End,
    Between,
    Around,
    Evenly,
    Baseline,
    Stretch,
}

impl PlaceContent {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "center" => Self::Center,
            "start" => Self::Start,
            "end" => Self::End,
            "between" => Self::Between,
            "around" => Self::Around,
            "evenly" => Self::Evenly,
            "baseline" => Self::Baseline,
            "stretch" => Self::Stretch,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Center => "center",
            Self::Start => "start",
            Self::End => "end",
            Self::Between => "space-between",
            Self::Around => "space-around",
            Self::Evenly => "space-evenly",
            Self::Baseline => "baseline",
            Self::Stretch => "stretch",
        };

        Decl::Single(format!("place-content: {}", val))
    }
}

#[derive(Debug)]
pub enum PlaceItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl PlaceItems {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "start" => Self::Start,
            "end" => Self::End,
            "center" => Self::Center,
            "baseline" => Self::Baseline,
            "stretch" => Self::Stretch,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::Baseline => "baseline",
            Self::Stretch => "stretch",
        };

        Decl::Single(format!("place-items: {}", val))
    }
}

#[derive(Debug)]
pub enum PlaceSelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}

impl PlaceSelf {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "auto" => Self::Auto,
            "start" => Self::Start,
            "end" => Self::End,
            "center" => Self::Center,
            "stretch" => Self::Stretch,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Auto => "auto",
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::Stretch => "stretch",
        };

        Decl::Single(format!("place-self: {}", val))
    }
}
