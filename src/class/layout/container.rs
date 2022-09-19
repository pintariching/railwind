use crate::class::BaseClass;
use swc_common::Span;
use swc_css::ast::{
    ClassSelector, ComplexSelector, ComplexSelectorChildren, ComponentValue, CompoundSelector,
    Declaration, DeclarationName, Dimension, Ident, Length, Number, Percentage, QualifiedRule,
    QualifiedRulePrelude, Rule, SelectorList, SimpleBlock, StyleBlock, SubclassSelector,
};

#[derive(Debug)]
pub struct Container(pub Option<BaseClass>);

impl Container {
    pub fn default() -> Self {
        Self(None)
    }
    pub fn generate_rule(self) -> Rule {
        if let Some(base_class) = self.0 {
            todo!()
        } else {
            return Rule::QualifiedRule(Box::new(QualifiedRule {
                span: Span::default(),
                prelude: QualifiedRulePrelude::SelectorList(SelectorList {
                    span: Span::default(),
                    children: vec![ComplexSelector {
                        span: Span::default(),
                        children: vec![ComplexSelectorChildren::CompoundSelector(
                            CompoundSelector {
                                span: Span::default(),
                                nesting_selector: None,
                                type_selector: None,
                                subclass_selectors: vec![SubclassSelector::Class(ClassSelector {
                                    span: Span::default(),
                                    text: Ident {
                                        span: Span::default(),
                                        value: "container".into(),
                                        raw: Some("container".into()),
                                    },
                                })],
                            },
                        )],
                    }],
                }),
                block: SimpleBlock {
                    span: Span::default(),
                    name: '{',
                    value: vec![ComponentValue::StyleBlock(StyleBlock::Declaration(
                        Box::new(Declaration {
                            span: Span::default(),
                            name: DeclarationName::Ident(Ident {
                                span: Span::default(),
                                value: "width".into(),
                                raw: Some("width".into()),
                            }),
                            value: vec![ComponentValue::Percentage(Percentage {
                                span: Span::default(),
                                value: Number {
                                    span: Span::default(),
                                    value: 100.into(),
                                    raw: Some("100".into()),
                                },
                            })],
                            important: None,
                        }),
                    ))],
                },
            }));
        }
    }
}
