use swc_common::Span;
use swc_css::ast::{
    ClassSelector, ComplexSelector, ComplexSelectorChildren, ComponentValue, CompoundSelector,
    Declaration, DeclarationName, Delimiter, DelimiterValue, Ident, Integer, Number, Percentage,
    PseudoClassSelector, QualifiedRulePrelude, Ratio, SelectorList, SimpleBlock, Str, StyleBlock,
    SubclassSelector,
};

use crate::modifiers::PseudoClass;

pub fn new_qualified_rule_prelude(class_selector: &str) -> QualifiedRulePrelude {
    QualifiedRulePrelude::SelectorList(SelectorList {
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
                            value: class_selector.into(),
                            raw: None,
                        },
                    })],
                },
            )],
        }],
    })
}

pub fn new_qualified_rule_with_pseudoclass_prelude(
    class_selector: &str,
    pseudoclasses: Vec<String>,
) -> QualifiedRulePrelude {
    let mut subclass_selectors: Vec<SubclassSelector> = vec![new_subclass_selector(class_selector)];
    for pseudoclass in pseudoclasses {
        subclass_selectors.push(new_pseudo_subclass_selector(pseudoclass.as_str()));
    }

    QualifiedRulePrelude::SelectorList(SelectorList {
        span: Span::default(),
        children: vec![ComplexSelector {
            span: Span::default(),
            children: vec![ComplexSelectorChildren::CompoundSelector(
                CompoundSelector {
                    span: Span::default(),
                    nesting_selector: None,
                    type_selector: None,
                    subclass_selectors,
                },
            )],
        }],
    })
}

pub fn new_subclass_selector(selector: &str) -> SubclassSelector {
    SubclassSelector::Class(ClassSelector {
        span: Span::default(),
        text: Ident {
            span: Span::default(),
            value: selector.into(),
            raw: None,
        },
    })
}

pub fn new_pseudo_subclass_selector(selector: &str) -> SubclassSelector {
    SubclassSelector::PseudoClass(PseudoClassSelector {
        span: Span::default(),
        name: Ident {
            span: Span::default(),
            value: selector.into(),
            raw: None,
        },
        children: None,
    })
}

pub fn new_simple_block(declaration: Box<Declaration>) -> SimpleBlock {
    SimpleBlock {
        span: Span::default(),
        name: '{',
        value: vec![ComponentValue::StyleBlock(StyleBlock::Declaration(
            declaration,
        ))],
    }
}

pub fn new_declaration(
    declaration_name_value: &str,
    value: Vec<ComponentValue>,
) -> Box<Declaration> {
    Box::new(Declaration {
        span: Span::default(),
        name: DeclarationName::Ident(Ident {
            span: Span::default(),
            value: declaration_name_value.into(),
            raw: None,
        }),
        value,
        important: None,
    })
}

pub fn new_number(value: i32) -> Number {
    Number {
        span: Span::default(),
        value: value.into(),
        raw: None,
    }
}

pub fn new_component_value_str(value: &str) -> ComponentValue {
    ComponentValue::Str(Str {
        span: Span::default(),
        value: value.into(),
        raw: None,
    })
}

pub fn new_component_value_integer(value: i32) -> ComponentValue {
    ComponentValue::Integer(Integer {
        span: Span::default(),
        value: value.into(),
        raw: None,
    })
}

pub fn new_component_value_delimiter(value: DelimiterValue) -> ComponentValue {
    ComponentValue::Delimiter(Delimiter {
        span: Span::default(),
        value: value.into(),
    })
}

pub fn new_component_value_ratio(left: i32, right: Option<i32>) -> ComponentValue {
    ComponentValue::Ratio(Ratio {
        span: Span::default(),
        left: new_number(left),
        right: match right {
            Some(r) => Some(new_number(r)),
            None => None,
        },
    })
}

pub fn new_component_value_percentage(value: i32) -> ComponentValue {
    ComponentValue::Percentage(Percentage {
        span: Span::default(),
        value: Number {
            span: Span::default(),
            value: value.into(),
            raw: None,
        },
    })
}
