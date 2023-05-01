use syn::{parse::Parse, Attribute};

pub fn get_attr<T>(attrs: &Vec<Attribute>, name: &str) -> T
where
    T: Parse,
{
    for attr in attrs {
        if attr.path().is_ident(name) {
            let name: T = attr.parse_args().expect(&format!(
                "failed to parse attribute {name} to the required type"
            ));
            return name;
        }
    }
    panic!("macro requires '{name}' attribute")
}
