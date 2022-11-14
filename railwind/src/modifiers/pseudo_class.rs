use std::str::FromStr;

use crate::traits::ToStaticStr;

#[derive(Debug, PartialEq, Eq)]
pub enum PseudoClass {
    Hover,
    Focus,
    FocusWithin,
    FocusVisible,
    Active,
    Visited,
    Target,
    First,
    Last,
    Only,
    Odd,
    Even,
    FirstOfType,
    LastOfType,
    OnlyOfType,
    Empty,
    Disabled,
    Enabled,
    Checked,
    Indeterminate,
    Default,
    Required,
    Valid,
    Invalid,
    InRange,
    OutOfRange,
    PlaceholderShown,
    Autofill,
    ReadOnly,
    Open,
}

impl FromStr for PseudoClass {
    type Err = ();

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "hover" => Ok(PseudoClass::Hover),
            "focus" => Ok(PseudoClass::Focus),
            "focus-within" => Ok(PseudoClass::FocusWithin),
            "focus-visible" => Ok(PseudoClass::FocusVisible),
            "active" => Ok(PseudoClass::Active),
            "visited" => Ok(PseudoClass::Visited),
            "target" => Ok(PseudoClass::Target),
            "first-child" => Ok(PseudoClass::First),
            "last-child" => Ok(PseudoClass::Last),
            "only-child" => Ok(PseudoClass::Only),
            "nth-child(odd)" => Ok(PseudoClass::Odd),
            "nth-child(even)" => Ok(PseudoClass::Even),
            "first-of-type" => Ok(PseudoClass::FirstOfType),
            "last-of-type" => Ok(PseudoClass::LastOfType),
            "only-of-type" => Ok(PseudoClass::OnlyOfType),
            "empty" => Ok(PseudoClass::Empty),
            "disabled" => Ok(PseudoClass::Disabled),
            "enabled" => Ok(PseudoClass::Enabled),
            "checked" => Ok(PseudoClass::Checked),
            "indeterminate" => Ok(PseudoClass::Indeterminate),
            "default" => Ok(PseudoClass::Default),
            "required" => Ok(PseudoClass::Required),
            "valid" => Ok(PseudoClass::Valid),
            "invalid" => Ok(PseudoClass::Invalid),
            "in-range" => Ok(PseudoClass::InRange),
            "out-of-range" => Ok(PseudoClass::OutOfRange),
            "placeholder-shown" => Ok(PseudoClass::PlaceholderShown),
            "autofill" => Ok(PseudoClass::Autofill),
            "readonly" => Ok(PseudoClass::ReadOnly),
            "open" => Ok(PseudoClass::Open),
            _ => Err(()),
        }
    }
}

impl ToStaticStr for &PseudoClass {
    fn to_static_str(&self) -> &'static str {
        match self {
            PseudoClass::Hover => "hover",
            PseudoClass::Focus => "focus",
            PseudoClass::FocusWithin => "focus-within",
            PseudoClass::FocusVisible => "focus-visible",
            PseudoClass::Active => "active",
            PseudoClass::Visited => "visited",
            PseudoClass::Target => "target",
            PseudoClass::First => "first-child",
            PseudoClass::Last => "last-child",
            PseudoClass::Only => "only-child",
            PseudoClass::Odd => "nth-child(odd)",
            PseudoClass::Even => "nth-child(even)",
            PseudoClass::FirstOfType => "first-of-type",
            PseudoClass::LastOfType => "last-of-type",
            PseudoClass::OnlyOfType => "only-of-type",
            PseudoClass::Empty => "empty",
            PseudoClass::Disabled => "disabled",
            PseudoClass::Enabled => "enabled",
            PseudoClass::Checked => "checked",
            PseudoClass::Indeterminate => "indeterminate",
            PseudoClass::Default => "default",
            PseudoClass::Required => "required",
            PseudoClass::Valid => "valid",
            PseudoClass::Invalid => "invalid",
            PseudoClass::InRange => "in-range",
            PseudoClass::OutOfRange => "out-of-range",
            PseudoClass::PlaceholderShown => "placeholder-shown",
            PseudoClass::Autofill => "autofill",
            PseudoClass::ReadOnly => "readonly",
            PseudoClass::Open => "open",
        }
    }
}
