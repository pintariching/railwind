use swc_css::ast::SimpleBlock;

use crate::{
    class::{
        helpers::{
            new_component_value_declaration, new_component_value_percentage, new_declaration,
            new_simple_block,
        },
        GenerateSimpleBlock, ParseFromStr,
    },
    modifiers::Modifier,
};

#[derive(Debug)]
pub struct Container {
    modifiers: Option<Vec<Modifier>>,
    class_selector: String,
}

impl ParseFromStr for Container {
    fn parse_from_str(class: &str) -> Self {
        Self {
            modifiers: Modifier::parse_many_from_str(class),
            class_selector: class.into(),
        }
    }
}

impl GenerateSimpleBlock for Container {
    fn generate_simple_block(&self) -> SimpleBlock {
        new_simple_block(vec![new_component_value_declaration(
            "width",
            vec![new_component_value_percentage(100)],
        )])
    }
}

// pub fn generate_rule(self) -> Rule {
//     let block = new_simple_block(new_declaration(
//         "width",
//         vec![new_component_value_percentage(100)],
//     ));

//     if let Some(modifiers) = self.0 {
//         return Rule::QualifiedRule(Box::new(QualifiedRule {
//             span: Span::default(),
//             prelude: new_qualified_rule_with_pseudoclass_prelude(
//                 "container",
//                 modifiers.iter().map(|m| m.to_string()).collect(),
//             ),
//             block,
//         }));
//     } else {
//         return Rule::QualifiedRule(Box::new(QualifiedRule {
//             span: Span::default(),
//             prelude: new_qualified_rule_prelude("container"),
//             block,
//         }));
//     }
// }
