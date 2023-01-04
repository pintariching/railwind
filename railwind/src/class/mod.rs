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

use self::filters::Filter;

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
    pub fn new(value: &'a str) -> Option<Self> {
        let class = if let Some(interactivity) = Interactivity::new(value) {
            Class::Interactivity(interactivity)
        } else if let Some(layout) = Layout::new(value) {
            Class::Layout(layout)
        } else if let Some(flexbox_grid) = FlexboxGrid::new(value) {
            Class::FlexboxGrid(flexbox_grid)
        } else if let Some(spacing) = Spacing::new(value) {
            Class::Spacing(spacing)
        } else if let Some(sizing) = Sizing::new(value) {
            Class::Sizing(sizing)
        } else if let Some(svg) = Svg::new(value) {
            Class::Svg(svg)
        } else if let Some(table) = Table::new(value) {
            Class::Table(table)
        } else if let Some(transitions_animation) = TransitionsAnimation::new(value) {
            Class::TransitionsAnimation(transitions_animation)
        } else if let Some(transform) = Transform::new(value) {
            Class::Transform(transform)
        } else if let Some(typography) = Typography::new(value) {
            Class::Typography(typography)
        } else if let Some(accessibility) = Accessibility::new(value) {
            Class::Accessibility(accessibility)
        } else if let Some(effects) = Effects::new(value) {
            Class::Effects(effects)
        } else if let Some(backgrounds) = Backgrounds::new(value) {
            Class::Backgrounds(backgrounds)
        } else if let Some(borders) = Borders::new(value) {
            Class::Borders(borders)
        } else if let Some(filter) = Filter::new(value) {
            Class::Filters(filter)
        } else {
            return None;
        };

        Some(class)
    }

    pub fn to_decl(self) -> Option<Decl> {
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
            Class::Accessibility(c) => c.to_decl(),
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
