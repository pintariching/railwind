mod accessibility;
mod backgrounds;
mod borders;
mod effects;
mod filters;
mod flexbox_grid;
mod interactivity;
mod layout;
mod prose;
mod sizing;
mod spacing;
mod svg;
mod tables;
mod transforms;
mod transitions_animation;
mod typography;
mod utils;

pub use accessibility::*;
pub use backgrounds::*;
pub use borders::*;
pub use effects::*;
pub use filters::*;
pub use flexbox_grid::*;
pub use interactivity::*;
pub use layout::*;
pub use prose::*;
pub use sizing::*;
pub use spacing::*;
pub use svg::*;
pub use tables::*;
pub use transforms::*;
pub use transitions_animation::*;
pub use typography::*;

use crate::warning::{Position, Warning, WarningType};

#[derive(Debug)]
pub enum Class<'a> {
    Interactivity(Interactivity<'a>),
    Layout(Layout<'a>),
    Spacing(Spacing<'a>),
    FlexboxGrid(FlexboxGrid<'a>),
    Sizing(Sizing<'a>),
    Svg(Svg<'a>),
    Table(Table<'a>),
    TransitionsAnimation(TransitionsAnimation<'a>),
    Transform(Transform<'a>),
    Typography(Typography<'a>),
    Accessibility(Accessibility),
    Backgrounds(Backgrounds<'a>),
    Borders(Borders<'a>),
    Effects(Effects<'a>),
    Filters(Filter<'a>),
    Prose(Prose<'a>),
}

impl<'a> Class<'a> {
    pub fn new(raw_class: &'a str, value: &'a str, position: &Position) -> Result<Self, Warning> {
        let class = if let Some(interactivity) =
            Interactivity::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Self::Interactivity(interactivity)
        } else if let Some(layout) =
            Layout::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Self::Layout(layout)
        } else if let Some(flexbox_grid) =
            FlexboxGrid::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Self::FlexboxGrid(flexbox_grid)
        } else if let Some(spacing) =
            Spacing::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Self::Spacing(spacing)
        } else if let Some(sizing) =
            Sizing::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Self::Sizing(sizing)
        } else if let Some(svg) =
            Svg::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Self::Svg(svg)
        } else if let Some(table) =
            Table::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Self::Table(table)
        } else if let Some(transitions_animation) =
            TransitionsAnimation::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Self::TransitionsAnimation(transitions_animation)
        } else if let Some(transform) =
            Transform::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Self::Transform(transform)
        } else if let Some(typography) =
            Typography::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Self::Typography(typography)
        } else if let Some(accessibility) = Accessibility::new(value) {
            Self::Accessibility(accessibility)
        } else if let Some(effects) =
            Effects::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Self::Effects(effects)
        } else if let Some(backgrounds) =
            Backgrounds::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Self::Backgrounds(backgrounds)
        } else if let Some(borders) =
            Borders::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Self::Borders(borders)
        } else if let Some(filter) =
            Filter::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Self::Filters(filter)
        } else if let Some(prose) =
            Prose::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Self::Prose(prose)
        } else {
            return Err(Warning::new(
                raw_class,
                position,
                WarningType::ClassNotFound,
            ));
        };

        Ok(class)
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::Interactivity(c) => c.to_decl(),
            Self::Layout(c) => c.to_decl(),
            Self::FlexboxGrid(c) => c.to_decl(),
            Self::Spacing(c) => c.to_decl(),
            Self::Sizing(c) => c.to_decl(),
            Self::Svg(c) => c.to_decl(),
            Self::Table(c) => c.to_decl(),
            Self::TransitionsAnimation(c) => c.to_decl(),
            Self::Transform(c) => c.to_decl(),
            Self::Typography(c) => c.to_decl(),
            Self::Accessibility(c) => Ok(c.to_decl()),
            Self::Backgrounds(c) => c.to_decl(),
            Self::Borders(c) => c.to_decl(),
            Self::Effects(c) => c.to_decl(),
            Self::Filters(c) => c.to_decl(),
            Self::Prose(c) => c.to_decl(),
        }
    }
}

#[derive(Debug)]
pub enum Decl {
    Lit(&'static str),
    Single(String),
    Double([String; 2]),
    Triple([String; 3]),
    Quad([String; 4]),
    Multiple(Vec<String>),
    FullClass(String),
}

impl Decl {
    pub fn to_string(self) -> String {
        match self {
            Self::Lit(lit) => lit.to_string(),
            Self::Single(s) => s,
            Self::Triple(t) => t.join(";\n    "),
            Self::Double(d) => d.join(";\n    "),
            Self::Quad(q) => q.join(";\n    "),
            Self::Multiple(m) => m.join(";\n    "),
            Self::FullClass(fc) => fc,
        }
    }
}
