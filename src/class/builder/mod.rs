use std::str::FromStr;

use swc_common::Span;
use swc_css::ast::{DashedIdent, Ident};

pub mod base;
pub mod qualified_rule;
pub mod selector;

trait IdentBuilder {
    fn from_str(str: &str) -> Self;
}

impl IdentBuilder for Ident {
    fn from_str(str: &str) -> Self {
        Self {
            span: Span::default(),
            value: str.into(),
            raw: None,
        }
    }
}

impl IdentBuilder for DashedIdent {
    fn from_str(str: &str) -> Self {
        Self {
            span: Span::default(),
            value: str.into(),
            raw: None,
        }
    }
}
