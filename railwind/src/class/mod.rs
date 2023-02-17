mod accessibility;
mod backgrounds;
mod borders;
mod effects;
mod filters;
mod flexbox_grid;
mod interactivity;
mod layout;
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
}

impl<'a> Class<'a> {
    pub fn new(raw_class: &'a str, value: &'a str, position: &Position) -> Result<Self, Warning> {
        let class = if let Some(interactivity) =
            Interactivity::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Class::Interactivity(interactivity)
        } else if let Some(layout) =
            Layout::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Class::Layout(layout)
        } else if let Some(flexbox_grid) =
            FlexboxGrid::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Class::FlexboxGrid(flexbox_grid)
        } else if let Some(spacing) =
            Spacing::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Class::Spacing(spacing)
        } else if let Some(sizing) =
            Sizing::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Class::Sizing(sizing)
        } else if let Some(svg) =
            Svg::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Class::Svg(svg)
        } else if let Some(table) =
            Table::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Class::Table(table)
        } else if let Some(transitions_animation) =
            TransitionsAnimation::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Class::TransitionsAnimation(transitions_animation)
        } else if let Some(transform) =
            Transform::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Class::Transform(transform)
        } else if let Some(typography) =
            Typography::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Class::Typography(typography)
        } else if let Some(accessibility) = Accessibility::new(value) {
            Class::Accessibility(accessibility)
        } else if let Some(effects) =
            Effects::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Class::Effects(effects)
        } else if let Some(backgrounds) =
            Backgrounds::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Class::Backgrounds(backgrounds)
        } else if let Some(borders) =
            Borders::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Class::Borders(borders)
        } else if let Some(filter) =
            Filter::new(value).map_err(|e| Warning::new(raw_class, position, e))?
        {
            Class::Filters(filter)
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
            Class::Interactivity(c) => c.to_decl(),
            Class::Layout(c) => c.to_decl(),
            Class::FlexboxGrid(c) => c.to_decl(),
            Class::Spacing(c) => c.to_decl(),
            Class::Sizing(c) => c.to_decl(),
            Class::Svg(c) => c.to_decl(),
            Class::Table(c) => c.to_decl(),
            Class::TransitionsAnimation(c) => c.to_decl(),
            Class::Transform(c) => c.to_decl(),
            Class::Typography(c) => c.to_decl(),
            Class::Accessibility(c) => Ok(c.to_decl()),
            Class::Backgrounds(c) => c.to_decl(),
            Class::Borders(c) => c.to_decl(),
            Class::Effects(c) => c.to_decl(),
            Class::Filters(c) => c.to_decl(),
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
            Decl::Lit(lit) => lit.to_string(),
            Decl::Single(s) => s,
            Decl::Triple(t) => t.join(";\n    "),
            Decl::Double(d) => d.join(";\n    "),
            Decl::Quad(q) => q.join(";\n    "),
            Decl::Multiple(m) => m.join(";\n    "),
            Decl::FullClass(fc) => fc,
        }
    }
}
