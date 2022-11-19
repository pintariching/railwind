use crate::class::check_arg_count;
use crate::utils::get_keys;
use crate::warning::WarningType;

use indexmap::IndexMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ASPECT_RATIO: IndexMap<&'static str, &'static str> =
        ron::from_str(include_str!("aspect_ratio.ron")).unwrap();
}

pub fn parse_aspect_ratio(
    args: &[&str; 3],
    warnings: &mut Vec<WarningType>,
) -> Option<Vec<String>> {
    check_arg_count(args, 1, warnings);

    if let Some(aspect_ratio) = ASPECT_RATIO.get(args[0]) {
        return Some(vec![format!("aspect-ratio: {}", aspect_ratio)]);
    }

    warnings.push(WarningType::InvalidArg(
        args[0].into(),
        get_keys(ASPECT_RATIO.keys()),
    ));

    None
}
