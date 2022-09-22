use swc_common::Span;
use swc_css::ast::{
    AtRule, AtRuleName, AtRulePrelude, ClassSelector, ComplexSelector, ComplexSelectorChildren,
    ComponentValue, CompoundSelector, Declaration, DeclarationName, DeclarationOrAtRule, Delimiter,
    DelimiterValue, Dimension, Ident, Integer, Length, MediaCondition, MediaConditionAllType,
    MediaConditionType, MediaConditionWithoutOr, MediaFeature, MediaFeatureName, MediaFeaturePlain,
    MediaFeatureValue, MediaInParens, MediaQuery, MediaQueryList, Number, Percentage,
    PseudoClassSelector, QualifiedRule, QualifiedRulePrelude, Ratio, Rule, SelectorList,
    SimpleBlock, Str, StyleBlock, SubclassSelector,
};

use crate::modifiers::Modifier;

pub fn new_rule(
    modifiers: Option<Vec<Modifier>>,
    class_selector: &str,
    block: SimpleBlock,
) -> Rule {
    let at_rule = Rule::AtRule(Box::new(AtRule {
        span: Span::default(),
        name: AtRuleName::Ident(new_ident("media")),
        prelude: None,
        block: None,
    }));

    if let Some(modifiers) = modifiers {
        return Rule::QualifiedRule(Box::new(QualifiedRule {
            span: Span::default(),
            prelude: new_qualified_rule_with_pseudoclass_prelude(
                class_selector,
                modifiers.iter().map(|m| m.to_string()).collect(),
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

pub fn new_simple_block(value: Vec<ComponentValue>) -> SimpleBlock {
    SimpleBlock {
        span: Span::default(),
        name: '{',
        value,
    }
}

pub fn new_declaration(declaration_name_value: &str, value: Vec<ComponentValue>) -> Declaration {
    Declaration {
        span: Span::default(),
        name: DeclarationName::Ident(Ident {
            span: Span::default(),
            value: declaration_name_value.into(),
            raw: None,
        }),
        value,
        important: None,
    }
}

pub fn new_at_rule(at_rule_name_value: &str, block: SimpleBlock) -> AtRule {
    AtRule {
        span: Span::default(),
        name: AtRuleName::Ident(Ident {
            span: Span::default(),
            value: at_rule_name_value.into(),
            raw: None,
        }),
        prelude: Some(Box::new(AtRulePrelude::MediaPrelude(MediaQueryList {
            span: Span::default(),
            queries: vec![MediaQuery {
                span: Span::default(),
                modifier: None,
                media_type: None,
                keyword: None,
                condition: Some(Box::new(MediaConditionType::All(MediaCondition {
                    span: Span::default(),
                    conditions: vec![MediaConditionAllType::MediaInParens(
                        MediaInParens::Feature(Box::new(MediaFeature::Plain(MediaFeaturePlain {
                            span: Span::default(),
                            name: MediaFeatureName::Ident(new_ident("min-width")),
                            value: Box::new(MediaFeatureValue::Dimension(Dimension::Length(
                                Length {
                                    span: Span::default(),
                                    value: new_number(640.),
                                    unit: new_ident("px"),
                                },
                            ))),
                        }))),
                    )],
                }))),
            }],
        }))),
        block: Some(block),
    }
}

pub fn new_number(value: f32) -> Number {
    Number {
        span: Span::default(),
        value: value.into(),
        raw: None,
    }
}

pub fn new_component_value_declaration(
    declaration_name_value: &str,
    value: Vec<ComponentValue>,
) -> ComponentValue {
    ComponentValue::DeclarationOrAtRule(DeclarationOrAtRule::Declaration(Box::new(
        new_declaration(declaration_name_value, value),
    )))
}

pub fn new_component_value_at_rule() -> ComponentValue {
    //ComponentValue::DeclarationOrAtRule(DeclarationOrAtRule::AtRule(Box::new(new_at_rule())));

    todo!()
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
