#[macro_export]
macro_rules! ret_single_decl {
    ($decl:literal) => {
        return Some(Decl::Single($lit.to_string()))
    };
    ($decl:literal, $val:expr) => {
        return Some(Decl::Single(format!("{}: {}", $decl, $val)))
    };
    ($decl:expr, $val:expr) => {
        return Some(Decl::Single(format!("{}: {}", $decl, $val)))
    };
    ($decl:literal, $val1:expr, $val2:expr) => {
        return Some(Decl::Single(format!("{}: {} {}", $decl, $val1, $val2)))
    };
}

#[macro_export]
macro_rules! ret_single_decl_suf {
    ($decl:literal, $val:expr, $suf:literal) => {
        return Some(Decl::Single(format!("{}: {}-{}", $decl, $val, $suf)))
    };
}

#[macro_export]
macro_rules! ret_single_decl_pref {
    ($decl:literal, $val:literal, $pref:expr) => {
        return Some(Decl::Single(format!("{}: {}-{}", $decl, $pref, $val)))
    };
}

#[macro_export]
macro_rules! ret_lit {
    ($lit:literal) => {
        return Some(Decl::Lit($lit))
    };
}
