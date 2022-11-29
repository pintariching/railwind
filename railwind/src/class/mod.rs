mod flexbox_grid;
mod layout;
mod sizing;
mod spacing;
mod utils;

pub use flexbox_grid::*;
pub use layout::*;
pub use sizing::*;
pub use spacing::*;

#[derive(Debug)]
pub enum Class<'a> {
    Layout(Layout<'a>),
    Spacing(Spacing<'a>),
    FlexboxGrid(FlexboxGrid<'a>),
    Sizing(Sizing<'a>),
}

impl<'a> Class<'a> {
    pub fn new(value: &'a str) -> Option<Self> {
        let class = if let Some(layout) = Layout::new(value) {
            Class::Layout(layout)
        } else if let Some(spacing) = Spacing::new(value) {
            Class::Spacing(spacing)
        } else if let Some(flexbox_grid) = FlexboxGrid::new(value) {
            Class::FlexboxGrid(flexbox_grid)
        } else if let Some(sizing) = Sizing::new(value) {
            Class::Sizing(sizing)
        } else {
            return None;
        };

        Some(class)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Class::Layout(c) => c.to_decl(),
            Class::Spacing(c) => c.to_decl(),
            Class::FlexboxGrid(c) => c.to_decl(),
            Class::Sizing(c) => c.to_decl(),
        }
    }
}

#[derive(Debug)]
pub enum Decl {
    Lit(&'static str),
    Single(String),
    Double([String; 2]),
    Quad([String; 4]),
    FullClass(String),
}

impl Decl {
    pub fn to_string(self) -> String {
        match self {
            Decl::Lit(lit) => lit.to_string(),
            Decl::Single(s) => s,
            Decl::Double(d) => d.join(";\n    "),
            Decl::Quad(q) => q.join(";\n    "),
            Decl::FullClass(fc) => fc,
        }
    }
}
