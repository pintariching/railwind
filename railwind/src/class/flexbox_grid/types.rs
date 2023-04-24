use crate::class::utils::{get_arbitrary_value, get_value, get_value_neg};
use crate::class::Decl;
use crate::utils::{get_args, get_class_name, get_opt_args};
use crate::warning::WarningType;

use super::{
    BASIS, FLEX, GAP, GAP_X, GAP_Y, GRID_AUTO_COLUMNS, GRID_AUTO_ROWS, GRID_COLUMN_END,
    GRID_COLUMN_SPAN, GRID_COLUMN_START, GRID_ROW_END, GRID_ROW_SPAN, GRID_ROW_START,
    GRID_TEMPLATE_COLUMNS, GRID_TEMPLATE_ROWS, GROW, ORDER, SHRINK,
};

#[derive(Debug, PartialEq, Hash)]
pub struct Basis<'a>(pub &'a str);

impl<'a> Basis<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &BASIS)?;
        Ok(Decl::String(format!("flex-basis: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
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

        Decl::String(format!("flex-direction: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash)]
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

        Decl::String(format!("flex-wrap: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct Flex<'a>(pub &'a str);

impl<'a> Flex<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &FLEX)?;
        Ok(Decl::String(format!("flex: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct Grow<'a>(pub &'a str);

impl<'a> Grow<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &GROW)?;
        Ok(Decl::String(format!("flex-grow: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct Shrink<'a>(pub &'a str);

impl<'a> Shrink<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &SHRINK)?;
        Ok(Decl::String(format!("flex-shrink: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct Order<'a>(pub &'a str, bool);

impl<'a> Order<'a> {
    pub fn new(name: &'a str, arg: &'a str) -> Self {
        let negative = name.starts_with('-');
        Self(arg, negative)
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value_neg(self.1, self.0, &ORDER)?;
        Ok(Decl::String(format!("order: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct GridTemplateColumns<'a>(pub &'a str);

impl<'a> GridTemplateColumns<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &GRID_TEMPLATE_COLUMNS)?;
        Ok(Decl::String(format!("grid-template-columns: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum GridColumn<'a> {
    Auto,
    Span(&'a str),
    Start(&'a str),
    End(&'a str),
}

impl<'a> GridColumn<'a> {
    pub fn new(args: &'a str) -> Result<Self, WarningType> {
        let arg = get_args(args)?;

        match get_class_name(args) {
            "auto" => Ok(Self::Auto),
            "span" => Ok(Self::Span(arg)),
            "start" => Ok(Self::Start(arg)),
            "end" => Ok(Self::End(arg)),
            _ => Err(WarningType::InvalidArg(
                args.into(),
                "Grid Column".into(),
                vec!["auto", "span", "start", "end"],
            )),
        }
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::Auto => Ok(Decl::Lit("grid-column: auto")),
            Self::Span(v) => {
                let value = get_value(v, &GRID_COLUMN_SPAN)?;
                Ok(Decl::String(format!("grid-column: {}", value)))
            }
            Self::Start(v) => {
                let value = get_value(v, &GRID_COLUMN_START)?;
                Ok(Decl::String(format!("grid-column-start: {}", value)))
            }
            Self::End(v) => {
                let value = get_value(v, &GRID_COLUMN_END)?;
                Ok(Decl::String(format!("grid-column-end: {}", value)))
            }
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct GridTepmlateRows<'a>(pub &'a str);

impl<'a> GridTepmlateRows<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &GRID_TEMPLATE_ROWS)?;
        Ok(Decl::String(format!("grid-template-rows: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum GridRow<'a> {
    Auto,
    Span(&'a str),
    Start(&'a str),
    End(&'a str),
    Arbitrary(&'a str),
}

impl<'a> GridRow<'a> {
    pub fn new(args: &'a str) -> Self {
        let arg = get_opt_args(args);
        match get_class_name(args) {
            "auto" => Self::Auto,
            "span" => Self::Span(arg),
            "start" => Self::Start(arg),
            "end" => Self::End(arg),
            _ => Self::Arbitrary(args),
        }
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::Auto => Ok(Decl::Lit("grid-row: auto")),
            Self::Span(v) => {
                let value = get_value(v, &GRID_ROW_SPAN)?;
                Ok(Decl::String(format!("grid-row: {}", value)))
            }
            Self::Start(v) => {
                let value = get_value(v, &GRID_ROW_START)?;
                Ok(Decl::String(format!("grid-row-start: {}", value)))
            }
            Self::End(v) => {
                let value = get_value(v, &GRID_ROW_END)?;
                Ok(Decl::String(format!("grid-row-end: {}", value)))
            }
            Self::Arbitrary(v) => {
                if let Some(value) = get_arbitrary_value(v) {
                    Ok(Decl::String(format!("grid-row: {}", value)))
                } else {
                    Err(WarningType::InvalidArbitraryArg(v.into()))
                }
            }
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum GridAutoFlow {
    Row,
    Col,
    Dense,
    RowDense,
    ColDense,
}

impl GridAutoFlow {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "row" => Ok(Self::Row),
            "col" => Ok(Self::Col),
            "dense" => Ok(Self::Dense),
            "row-dense" => Ok(Self::RowDense),
            "col-dense" => Ok(Self::ColDense),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Grid Auto Flow".into(),
                vec!["row", "col", "dense", "row-dense", "col-dense"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Row => "row",
            Self::Col => "column",
            Self::Dense => "dense",
            Self::RowDense => "row dense",
            Self::ColDense => "column dense",
        };

        Decl::String(format!("grid-auto-flow: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct GridAutoColumns<'a>(pub &'a str);

impl<'a> GridAutoColumns<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &GRID_AUTO_COLUMNS)?;
        Ok(Decl::String(format!("grid-auto-columns: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct GridAutoRows<'a>(pub &'a str);

impl<'a> GridAutoRows<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        let value = get_value(self.0, &GRID_AUTO_ROWS)?;
        Ok(Decl::String(format!("grid-auto-rows: {}", value)))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct Gap<'a>(pub &'a str);

impl<'a> Gap<'a> {
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match get_class_name(self.0) {
            "x" => {
                let val = get_value(get_args(self.0)?, &GAP_X)?;
                Ok(Decl::String(format!("column-gap: {}", val)))
            }
            "y" => {
                let val = get_value(get_args(self.0)?, &GAP_Y)?;
                Ok(Decl::String(format!("row-gap: {}", val)))
            }
            _ => {
                let val = get_value(self.0, &GAP)?;
                Ok(Decl::String(format!("gap: {}", val)))
            }
        }
    }
}

#[derive(Debug, PartialEq, Hash)]
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

        Decl::String(format!("justify-content: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum JustifyItems {
    Start,
    End,
    Center,
    Stretch,
}

impl JustifyItems {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            "center" => Ok(Self::Center),
            "stretch" => Ok(Self::Stretch),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Justify Items".into(),
                vec!["start", "end", "center", "stretch"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::Stretch => "stretch",
        };

        Decl::String(format!("justify-items: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum JustifySelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}

impl JustifySelf {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "auto" => Ok(Self::Auto),
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            "center" => Ok(Self::Center),
            "stretch" => Ok(Self::Stretch),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Justify Self".into(),
                vec!["auto", "start", "end", "center", "stretch"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Auto => "auto",
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::Stretch => "stretch",
        };

        Decl::String(format!("justify-self: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash)]
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
        match arg {
            "center" => Some(Self::Center),
            "start" => Some(Self::Start),
            "end" => Some(Self::End),
            "between" => Some(Self::Between),
            "around" => Some(Self::Around),
            "evenly" => Some(Self::Evenly),
            "baseline" => Some(Self::Baseline),
            _ => None,
        }
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

        Decl::String(format!("align-content: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl AlignItems {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            "center" => Ok(Self::Center),
            "baseline" => Ok(Self::Baseline),
            "stretch" => Ok(Self::Stretch),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Align Items".into(),
                vec!["start", "end", "center", "baseline", "stretch"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Start => "flex-start",
            Self::End => "flex-end",
            Self::Center => "center",
            Self::Baseline => "baseline",
            Self::Stretch => "stretch",
        };

        Decl::String(format!("align-items: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum AlignSelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}

impl AlignSelf {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "auto" => Ok(Self::Auto),
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            "center" => Ok(Self::Center),
            "stretch" => Ok(Self::Stretch),
            "baseline" => Ok(Self::Baseline),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Align Self".into(),
                vec!["auto", "start", "end", "center", "stretch", "baseline"],
            )),
        }
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

        Decl::String(format!("align-self: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash)]
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
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        let val = match arg {
            "center" => Self::Center,
            "start" => Self::Start,
            "end" => Self::End,
            "between" => Self::Between,
            "around" => Self::Around,
            "evenly" => Self::Evenly,
            "baseline" => Self::Baseline,
            "stretch" => Self::Stretch,
            _ => {
                return Err(WarningType::InvalidArg(
                    arg.into(),
                    "Place Content".into(),
                    vec![
                        "center", "start", "end", "between", "around", "evenly", "baseline",
                        "stretch",
                    ],
                ))
            }
        };

        Ok(val)
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

        Decl::String(format!("place-content: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum PlaceItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl PlaceItems {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            "center" => Ok(Self::Center),
            "baseline" => Ok(Self::Baseline),
            "stretch" => Ok(Self::Stretch),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Place Items".into(),
                vec!["start", "end", "center", "baseline", "stretch"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::Baseline => "baseline",
            Self::Stretch => "stretch",
        };

        Decl::String(format!("place-items: {}", val))
    }
}

#[derive(Debug, PartialEq, Hash)]
pub enum PlaceSelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}

impl PlaceSelf {
    pub fn new(arg: &str) -> Result<Self, WarningType> {
        match arg {
            "auto" => Ok(Self::Auto),
            "start" => Ok(Self::Start),
            "end" => Ok(Self::End),
            "center" => Ok(Self::Center),
            "stretch" => Ok(Self::Stretch),
            _ => Err(WarningType::InvalidArg(
                arg.into(),
                "Place Self".into(),
                vec!["auto", "start", "end", "center", "stretch"],
            )),
        }
    }

    pub fn to_decl(self) -> Decl {
        let val = match self {
            Self::Auto => "auto",
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::Stretch => "stretch",
        };

        Decl::String(format!("place-self: {}", val))
    }
}
