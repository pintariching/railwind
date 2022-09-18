use swc_css::ast::QualifiedRule;

use crate::class::BaseClass;

#[derive(Debug)]
pub struct Container(pub Option<BaseClass>);

impl Container {
    pub fn default() -> Self {
        Self(None)
    }
    pub fn generate_rule(&self) -> QualifiedRule {
        todo!()
    }
}
