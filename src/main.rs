use std::fs::{read_to_string, File};
use std::io::{Read, Write};
use std::path::Path;

use railwindcss::parse_html;
use swc_atoms::{Atom, JsWord};
use swc_common::{BytePos, Span};
use swc_css::ast::{
    ClassSelector, ComplexSelector, ComplexSelectorChildren, ComponentValue, CompoundSelector,
    Declaration, DeclarationName, Dimension, Ident, Length, Number, QualifiedRule,
    QualifiedRulePrelude, Rule, SelectorList, SimpleBlock, StyleBlock, Stylesheet,
    SubclassSelector,
};
use swc_css::codegen::writer::basic::{BasicCssWriter, BasicCssWriterConfig};
use swc_css::codegen::{CodeGenerator, CodegenConfig, Emit};
use swc_css::parser::parse_str;
use swc_css::parser::parser::ParserConfig;

fn main() {
    let _ = parse_html(Path::new("index.html"), Path::new("railwind.css"));

    // let ast: Stylesheet = Stylesheet {
    //     span: Span::default(),
    //     rules: vec![Rule::QualifiedRule(Box::new(QualifiedRule {
    //         span: Span::default(),
    //         prelude: QualifiedRulePrelude::SelectorList(SelectorList {
    //             span: Span::default(),
    //             children: vec![ComplexSelector {
    //                 span: Span::default(),
    //                 children: vec![ComplexSelectorChildren::CompoundSelector(
    //                     CompoundSelector {
    //                         span: Span::default(),
    //                         nesting_selector: None,
    //                         type_selector: None,
    //                         subclass_selectors: vec![SubclassSelector::Class(ClassSelector {
    //                             span: Span::default(),
    //                             text: Ident {
    //                                 span: Span::default(),
    //                                 value: "padded".into(),
    //                                 raw: Some("padded".into()),
    //                             },
    //                         })],
    //                     },
    //                 )],
    //             }],
    //         }),
    //         block: SimpleBlock {
    //             span: Span::default(),
    //             name: '{',
    //             value: vec![ComponentValue::StyleBlock(StyleBlock::Declaration(
    //                 Box::new(Declaration {
    //                     span: Span::default(),
    //                     name: DeclarationName::Ident(Ident {
    //                         span: Span::default(),
    //                         value: "padding".into(),
    //                         raw: Some("padding".into()),
    //                     }),
    //                     value: vec![ComponentValue::Dimension(Dimension::Length(Length {
    //                         span: Span::default(),
    //                         value: Number {
    //                             span: Span::default(),
    //                             value: 10.0,
    //                             raw: Some("10".into()),
    //                         },
    //                         unit: Ident {
    //                             span: Span::default(),
    //                             value: "px".into(),
    //                             raw: Some("px".into()),
    //                         },
    //                     }))],
    //                     important: None,
    //                 }),
    //             ))],
    //         },
    //     }))],
    // };

    // let mut css_str = String::new();
    // let writer = BasicCssWriter::new(&mut css_str, None, BasicCssWriterConfig::default());
    // let mut generator = CodeGenerator::new(writer, CodegenConfig { minify: false });

    // generator.emit(&ast).unwrap();

    // let mut css_file = File::create("generated.css").unwrap();
    // css_file.write_all(css_str.as_bytes()).unwrap();

    // let src = read_to_string("sample.css").unwrap();
    // let ast: Stylesheet = parse_str(
    //     src.as_str(),
    //     BytePos(0),
    //     BytePos(src.len() as u32),
    //     ParserConfig::default(),
    //     &mut Vec::new(),
    // )
    // .unwrap();

    // println!("{:#?}", ast);
}
