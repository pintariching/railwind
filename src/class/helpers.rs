use swc_common::Span;
use swc_css::ast::{
    ClassSelector, ComplexSelector, ComplexSelectorChildren, ComponentValue, CompoundSelector,
    Declaration, DeclarationName, Delimiter, DelimiterValue, Dimension, Ident, Integer, Length,
    Number, Percentage, PseudoClassSelector, QualifiedRule, QualifiedRulePrelude, Ratio, Rule,
    SelectorList, SimpleBlock, Str, StyleBlock, SubclassSelector,
};

use super::BaseClass;

pub fn new_rule(modifiers: Option<BaseClass>, class_selector: &str, block: SimpleBlock) -> Rule {
    if let Some(base_class) = modifiers {
        return Rule::QualifiedRule(Box::new(QualifiedRule {
            span: Span::default(),
            prelude: new_qualified_rule_with_pseudoclass_prelude(
                class_selector,
                base_class.to_string_vec(),
            ),
            block,
        }));
    } else {
        return Rule::QualifiedRule(Box::new(QualifiedRule {
            span: Span::default(),
            prelude: new_qualified_rule_prelude(class_selector),
            block,
        }));
    }
}

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

pub fn new_simple_block_many(declarations: Vec<Box<Declaration>>) -> SimpleBlock {
    SimpleBlock {
        span: Span::default(),
        name: '{',
        value: declarations
            .into_iter()
            .map(|d| ComponentValue::StyleBlock(StyleBlock::Declaration(d)))
            .collect(),
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

pub fn new_number(value: f32) -> Number {
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

pub fn new_component_value_length(value: f32, unit: &str) -> ComponentValue {
    ComponentValue::Dimension(Dimension::Length(Length {
        span: Span::default(),
        value: new_number(value),
        unit: new_ident(unit),
    }))
}

pub fn new_component_value_ratio(left: i32, right: Option<i32>) -> ComponentValue {
    ComponentValue::Ratio(Ratio {
        span: Span::default(),
        left: new_number(left as f32),
        right: match right {
            Some(r) => Some(new_number(r as f32)),
            None => None,
        },
    })
}

pub fn new_component_value_percentage(value: i32) -> ComponentValue {
    ComponentValue::Percentage(Percentage {
        span: Span::default(),
        value: new_number(value as f32),
    })
}

pub fn new_ident(value: &str) -> Ident {
    Ident {
        span: Span::default(),
        value: value.into(),
        raw: None,
    }
}
