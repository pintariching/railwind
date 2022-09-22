use swc_common::Span;
use swc_css::ast::{
    ClassSelector, Combinator, CombinatorValue, ComplexSelector, ComplexSelectorChildren,
    CompoundSelector, IdSelector, Ident, PseudoClassSelector, PseudoElementSelector, SelectorList,
    SubclassSelector,
};

use super::IdentBuilder;

trait SelectorListBuilder {
    fn new() -> Self;
    fn add_complex_selector(self, selector: ComplexSelector) -> Self;
    fn add_complex_selectors(self, selectors: Vec<ComplexSelector>) -> Self;
}

impl SelectorListBuilder for SelectorList {
    fn new() -> Self {
        Self {
            span: Span::default(),
            children: Vec::new(),
        }
    }

    fn add_complex_selector(mut self, selector: ComplexSelector) -> Self {
        self.children.push(selector);
        self
    }

    fn add_complex_selectors(mut self, selectors: Vec<ComplexSelector>) -> Self {
        self.children.append(&mut selectors.clone());
        self
    }
}

trait ComplexSelectorBuilder {
    fn new() -> Self;
    fn add_child(self, child: ComplexSelectorChildren) -> Self;
    fn add_children(self, children: Vec<ComplexSelectorChildren>) -> Self;
}

impl ComplexSelectorBuilder for ComplexSelector {
    fn new() -> Self {
        Self {
            span: Span::default(),
            children: Vec::new(),
        }
    }

    fn add_child(mut self, child: ComplexSelectorChildren) -> Self {
        self.children.push(child);
        self
    }

    fn add_children(mut self, children: Vec<ComplexSelectorChildren>) -> Self {
        self.children.append(&mut children.clone());
        self
    }
}

trait ComplexSelectorChildrenBuilder {
    fn new_compound_selector(compound_selector: CompoundSelector) -> Self;
    fn new_combinator(value: CombinatorValue) -> Self;
}

impl ComplexSelectorChildrenBuilder for ComplexSelectorChildren {
    fn new_compound_selector(compound_selector: CompoundSelector) -> Self {
        ComplexSelectorChildren::CompoundSelector(compound_selector)
    }

    fn new_combinator(value: CombinatorValue) -> Self {
        ComplexSelectorChildren::Combinator(Combinator {
            span: Span::default(),
            value,
        })
    }
}

trait CompoundSelectorBuilder {
    fn new() -> Self;
    fn add_subclass_selector(self, subclass_selector: SubclassSelector) -> Self;
    fn add_subclass_selectors(self, subclass_selectors: Vec<SubclassSelector>) -> Self;
}

impl CompoundSelectorBuilder for CompoundSelector {
    fn new() -> Self {
        Self {
            span: Span::default(),
            nesting_selector: None,
            type_selector: None,
            subclass_selectors: Vec::new(),
        }
    }
    fn add_subclass_selector(mut self, subclass_selector: SubclassSelector) -> Self {
        self.subclass_selectors.push(subclass_selector);
        self
    }

    fn add_subclass_selectors(mut self, subclass_selectors: Vec<SubclassSelector>) -> Self {
        self.subclass_selectors
            .append(&mut subclass_selectors.clone());
        self
    }
}

trait SubclassSelectorBuilder {
    fn new_id(text: &str) -> Self;
    fn new_class(text: &str) -> Self;
    fn new_pseudo_class(name: &str) -> Self;
    fn new_pseudo_element(name: &str) -> Self;
}

impl SubclassSelectorBuilder for SubclassSelector {
    fn new_id(text: &str) -> Self {
        SubclassSelector::Id(IdSelector {
            span: Span::default(),
            text: Ident::from_str(text),
        })
    }

    fn new_class(text: &str) -> Self {
        SubclassSelector::Class(ClassSelector {
            span: Span::default(),
            text: Ident::from_str(text),
        })
    }

    fn new_pseudo_class(name: &str) -> Self {
        SubclassSelector::PseudoClass(PseudoClassSelector {
            span: Span::default(),
            name: Ident::from_str(name),
            children: None,
        })
    }

    fn new_pseudo_element(name: &str) -> Self {
        SubclassSelector::PseudoElement(PseudoElementSelector {
            span: Span::default(),
            name: Ident::from_str(name),
            children: None,
        })
    }
}
