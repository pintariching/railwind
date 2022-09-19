use super::pseudo_class::PseudoClass;

#[derive(Debug)]
pub struct Group(PseudoClass);

impl Group {
    pub fn to_string(&self) -> String {
        format!("group-{}", self.0.as_str())
    }
}

#[derive(Debug)]
pub struct Peer(PseudoClass);

impl Peer {
    pub fn to_string(&self) -> String {
        format!("peer-{}", self.0.as_str())
    }
}
