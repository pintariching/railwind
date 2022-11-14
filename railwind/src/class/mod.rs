use crate::traits::IntoDeclaration;
use crate::warning::WarningType;

use layout::Layout;

mod layout;

#[derive(Debug, PartialEq)]
pub enum Class {
    Layout(Layout),
}

impl Class {
    pub fn new(class_name: &str, args: &[&str; 3]) -> Result<Self, WarningType> {
        match Layout::new(class_name, args) {
            Ok(layout) => return Ok(Class::Layout(layout)),
            Err(e) => match e {
                WarningType::ClassNotFound => (),
                _ => return Err(e),
            },
        }

        Err(WarningType::ClassNotFound)
    }
}

impl IntoDeclaration for Class {
    fn into_decl(&self) -> Vec<String> {
        match self {
            Class::Layout(l) => l.into_decl(),
        }
    }
}

pub fn check_arg_count(args: &[&str], count: usize) -> Result<(), WarningType> {
    let mut non_empty_count = 0;
    for arg in args {
        if !arg.is_empty() {
            non_empty_count += 1;
        }
    }

    if non_empty_count > count {
        Err(WarningType::TooManyArgs(non_empty_count, count))
    } else {
        Ok(())
    }
}
