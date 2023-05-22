use macro_derive::{ConfigurableParser, EnumParser, IntoDeclaration};

#[derive(Debug, PartialEq, Hash)]
pub enum FlexboxGrid<'a> {
    Basis(Basis<'a>),
    Direction(Direction),
    Wrap(Wrap),
    Flex(Flex<'a>),
    Grow(Grow<'a>),
    Shrink(Shrink<'a>),
    Order(Order<'a>),
    GridTemplateColumns(GridTemplateColumns<'a>),
    GridColumn(GridColumn<'a>),
    GridTepmlateRows(GridTepmlateRows<'a>),
    GridRow(GridRow<'a>),
    GridAutoFlow(GridAutoFlow),
    GridAutoColumns(GridAutoColumns<'a>),
    GridAutoRows(GridAutoRows<'a>),
    Gap(Gap<'a>),
    JustifyContent(JustifyContent),
    JustifyItems(JustifyItems),
    JustifySelf(JustifySelf),
    AlignContent(AlignContent),
    AlignItems(AlignItems),
    AlignSelf(AlignSelf),
    PlaceContent(PlaceContent),
    PlaceItems(PlaceItems),
    PlaceSelf(PlaceSelf),
}

#[derive(Debug, PartialEq, Hash, ConfigurableParser)]
#[name(basis)]
#[config(flexbox_grid.get_basis)]
pub struct Basis<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash, EnumParser, IntoDeclaration)]
#[name(flex_direction)]
#[decl("flex-direction")]
pub enum Direction {
    #[tag("row")]
    Row,

    #[tag("row-reverse")]
    RowReverse,

    #[tag("col")]
    #[decl("column")]
    Col,

    #[tag("col-reverse")]
    #[decl("column-reverse")]
    ColReverse,
}

#[derive(Debug, PartialEq, Hash, EnumParser, IntoDeclaration)]
#[name(flex_wrap)]
#[decl("flex-wrap")]
pub enum Wrap {
    #[tag("wrap")]
    Wrap,

    #[tag("wrap-reverse")]
    WrapReverse,

    #[tag("nowrap")]
    NoWrap,
}

#[derive(Debug, PartialEq, Hash)]
pub struct Flex<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash)]
pub struct Grow<'a>(pub &'a str);
#[derive(Debug, PartialEq, Hash)]
pub struct Shrink<'a>(pub &'a str);
#[derive(Debug, PartialEq, Hash)]
pub struct Order<'a>(pub &'a str, bool);
#[derive(Debug, PartialEq, Hash)]
pub struct GridTemplateColumns<'a>(pub &'a str);

#[derive(Debug, PartialEq, Hash)]
pub enum GridColumn<'a> {
    Auto,
    Span(&'a str),
    Start(&'a str),
    End(&'a str),
}
#[derive(Debug, PartialEq, Hash)]
pub struct GridTepmlateRows<'a>(pub &'a str);
#[derive(Debug, PartialEq, Hash)]

pub enum GridRow<'a> {
    Auto,
    Span(&'a str),
    Start(&'a str),
    End(&'a str),
    Arbitrary(&'a str),
}

#[derive(Debug, PartialEq, Hash)]
pub enum GridAutoFlow {
    Row,
    Col,
    Dense,
    RowDense,
    ColDense,
}
#[derive(Debug, PartialEq, Hash)]
pub struct GridAutoColumns<'a>(pub &'a str);
#[derive(Debug, PartialEq, Hash)]
pub struct GridAutoRows<'a>(pub &'a str);
#[derive(Debug, PartialEq, Hash)]
pub struct Gap<'a>(pub &'a str);
#[derive(Debug, PartialEq, Hash)]
pub enum JustifyContent {
    Start,
    End,
    Center,
    Between,
    Around,
    Evenly,
}

#[derive(Debug, PartialEq, Hash)]
pub enum JustifyItems {
    Start,
    End,
    Center,
    Stretch,
}
#[derive(Debug, PartialEq, Hash)]
pub enum JustifySelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
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
#[derive(Debug, PartialEq, Hash)]
pub enum AlignItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
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

#[derive(Debug, PartialEq, Hash)]
pub enum PlaceItems {
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}
#[derive(Debug, PartialEq, Hash)]
pub enum PlaceSelf {
    Auto,
    Start,
    End,
    Center,
    Stretch,
}
// impl<'a> FlexboxGrid<'a> {
//     pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
//         let flexbox_grid = match get_class_name(value) {
//             "basis" => Self::Basis(Basis(get_args(value)?)),
//             "flex" => {
//                 let args = get_args(value)?;

//                 if let Some(direction) = Direction::new(args) {
//                     Self::Direction(direction)
//                 } else if let Some(wrap) = Wrap::new(args) {
//                     Self::Wrap(wrap)
//                 } else {
//                     Self::Flex(Flex(args))
//                 }
//             }
//             "grow" => Self::Grow(Grow(get_opt_args(value))),
//             "shrink" => Self::Shrink(Shrink(get_opt_args(value))),
//             "order" | "-order" => Self::Order(Order::new(get_class_name(value), get_args(value)?)),
//             "grid" => {
//                 let args = get_args(value)?;

//                 match get_class_name(args) {
//                     "cols" => Self::GridTemplateColumns(GridTemplateColumns(get_args(args)?)),
//                     "rows" => Self::GridTepmlateRows(GridTepmlateRows(get_args(args)?)),
//                     "flow" => Self::GridAutoFlow(GridAutoFlow::new(get_args(args)?)?),
//                     v => {
//                         return Err(WarningType::InvalidArg(
//                             v.into(),
//                             "Grid Template / Auto Flow".into(),
//                             vec!["cols", "rows", "flow"],
//                         ))
//                     }
//                 }
//             }
//             "col" => Self::GridColumn(GridColumn::new(get_args(value)?)?),
//             "row" => Self::GridRow(GridRow::new(get_args(value)?)),
//             "auto" => {
//                 let args = get_args(value)?;

//                 match get_class_name(args) {
//                     "cols" => Self::GridAutoColumns(GridAutoColumns(get_args(args)?)),
//                     "rows" => Self::GridAutoRows(GridAutoRows(get_args(args)?)),
//                     v => {
//                         return Err(WarningType::InvalidArg(
//                             v.into(),
//                             "Grid Auto Columns / Rows".into(),
//                             vec!["cols", "rows"],
//                         ))
//                     }
//                 }
//             }
//             "gap" => Self::Gap(Gap(get_args(value)?)),
//             "justify" => {
//                 let args = get_args(value)?;

//                 if let Some(content) = JustifyContent::new(args) {
//                     Self::JustifyContent(content)
//                 } else {
//                     match get_class_name(args) {
//                         "items" => Self::JustifyItems(JustifyItems::new(get_args(args)?)?),
//                         "self" => Self::JustifySelf(JustifySelf::new(get_args(args)?)?),
//                         v => {
//                             return Err(WarningType::InvalidArg(
//                                 v.into(),
//                                 "Justify Items / Self".into(),
//                                 vec![
//                                     "start", "end", "center", "between", "around", "evenly",
//                                     "items", "self",
//                                 ],
//                             ))
//                         }
//                     }
//                 }
//             }
//             "content" => {
//                 if let Some(align_content) = AlignContent::new(get_args(value)?) {
//                     Self::AlignContent(align_content)
//                 } else {
//                     return Ok(None);
//                 }
//             }
//             "items" => Self::AlignItems(AlignItems::new(get_args(value)?)?),
//             "self" => Self::AlignSelf(AlignSelf::new(get_args(value)?)?),
//             "place" => {
//                 let args = get_args(value)?;

//                 match get_class_name(args) {
//                     "content" => Self::PlaceContent(PlaceContent::new(get_args(args)?)?),
//                     "items" => Self::PlaceItems(PlaceItems::new(get_args(args)?)?),
//                     "self" => Self::PlaceSelf(PlaceSelf::new(get_args(args)?)?),
//                     v => {
//                         return Err(WarningType::InvalidArg(
//                             v.into(),
//                             "Place Content / Items / Self".into(),
//                             vec!["content", "items", "self"],
//                         ))
//                     }
//                 }
//             }
//             _ => return Ok(None),
//         };

//         Ok(Some(flexbox_grid))
//     }

//     pub fn to_decl(self) -> Result<Decl, WarningType> {
//         match self {
//             Self::Basis(fg) => fg.to_decl(),
//             Self::Direction(fg) => Ok(fg.to_decl()),
//             Self::Wrap(fg) => Ok(fg.to_decl()),
//             Self::Flex(fg) => fg.to_decl(),
//             Self::Grow(fg) => fg.to_decl(),
//             Self::Shrink(fg) => fg.to_decl(),
//             Self::Order(fg) => fg.to_decl(),
//             Self::GridTemplateColumns(fg) => fg.to_decl(),
//             Self::GridColumn(fg) => fg.to_decl(),
//             Self::GridTepmlateRows(fg) => fg.to_decl(),
//             Self::GridRow(fg) => fg.to_decl(),
//             Self::GridAutoFlow(fg) => Ok(fg.to_decl()),
//             Self::GridAutoColumns(fg) => fg.to_decl(),
//             Self::GridAutoRows(fg) => fg.to_decl(),
//             Self::Gap(fg) => fg.to_decl(),
//             Self::JustifyContent(fg) => Ok(fg.to_decl()),
//             Self::JustifyItems(fg) => Ok(fg.to_decl()),
//             Self::JustifySelf(fg) => Ok(fg.to_decl()),
//             Self::AlignContent(fg) => Ok(fg.to_decl()),
//             Self::AlignItems(fg) => Ok(fg.to_decl()),
//             Self::AlignSelf(fg) => Ok(fg.to_decl()),
//             Self::PlaceContent(fg) => Ok(fg.to_decl()),
//             Self::PlaceItems(fg) => Ok(fg.to_decl()),
//             Self::PlaceSelf(fg) => Ok(fg.to_decl()),
//         }
//     }
// }
