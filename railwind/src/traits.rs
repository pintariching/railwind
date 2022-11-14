pub trait IntoDeclaration {
    fn into_decl(&self) -> Vec<String>;
}

pub trait ToStaticStr {
    fn to_static_str(&self) -> &'static str;
}
