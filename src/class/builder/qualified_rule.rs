use swc_common::Span;
use swc_css::ast::{
    ListOfComponentValues, QualifiedRule, QualifiedRulePrelude, SelectorList, SimpleBlock,
};

trait QualifiedRuleBuilder {
    fn new(prelude: QualifiedRulePrelude, block: SimpleBlock) -> Self;
    fn add_prelude(self, prelude: QualifiedRulePrelude) -> Self;
    fn add_block(self, block: SimpleBlock) -> Self;
}

impl QualifiedRuleBuilder for QualifiedRule {
    fn new(prelude: QualifiedRulePrelude, block: SimpleBlock) -> Self {
        Self {
            span: Span::default(),
            prelude,
            block,
        }
    }

    fn add_prelude(mut self, prelude: QualifiedRulePrelude) -> Self {
        self.prelude = prelude;
        self
    }

    fn add_block(self, block: SimpleBlock) -> Self {
        todo!()
    }
}

trait QualifiedRulePreludeBuilder {
    fn new_list_of_component_values() -> Self;
    fn new_selector_list() -> Self;
}

impl QualifiedRulePreludeBuilder for QualifiedRulePrelude {
    fn new_list_of_component_values() -> Self {
        QualifiedRulePrelude::ListOfComponentValues(ListOfComponentValues {
            span: Span::default(),
            children: Vec::new(),
        })
    }

    fn new_selector_list() -> Self {
        QualifiedRulePrelude::SelectorList(SelectorList {
            span: Span::default(),
            children: Vec::new(),
        })
    }
}
