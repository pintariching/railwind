use swc_common::Span;
use swc_css::ast::{
    AtRule, AtRuleName, ComponentValue, DashedIdent, Declaration, DeclarationName,
    DeclarationOrAtRule, Ident, SimpleBlock,
};

use super::IdentBuilder;

trait SimpleBlockBuilder {
    fn new() -> Self;
    fn add_component_value(self, value: ComponentValue) -> Self;
}

impl SimpleBlockBuilder for SimpleBlock {
    fn new() -> Self {
        Self {
            span: Span::default(),
            name: '{',
            value: Vec::new(),
        }
    }

    fn add_component_value(mut self, value: ComponentValue) -> Self {
        self.value.push(value);
        self
    }
}

trait ComponentValueBuilder {
    fn new_declaration(name: &str) -> Self;
    fn new_at_rule(name: &str) -> Self;
}

impl ComponentValueBuilder for ComponentValue {
    fn new_declaration(name: &str) -> Self {
        let declaration_name = if name.contains('-') {
            DeclarationName::DashedIdent(DashedIdent::from_str(name))
        } else {
            DeclarationName::Ident(Ident::from_str(name))
        };

        ComponentValue::DeclarationOrAtRule(DeclarationOrAtRule::Declaration(Box::new(
            Declaration {
                span: Span::default(),
                name: declaration_name,
                value: Vec::new(),
                important: None,
            },
        )))
    }

    fn new_at_rule(name: &str) -> Self {
        let declaration_name = if name.contains('-') {
            AtRuleName::DashedIdent(DashedIdent::from_str(name))
        } else {
            AtRuleName::Ident(Ident::from_str(name))
        };

        ComponentValue::DeclarationOrAtRule(DeclarationOrAtRule::AtRule(Box::new(AtRule {
            span: Span::default(),
            name: declaration_name,
            prelude: None,
            block: None,
        })))
    }
}
