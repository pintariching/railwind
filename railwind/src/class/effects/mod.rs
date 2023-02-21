mod types;

use types::*;

use lazy_static::lazy_static;
use std::collections::HashMap;

use super::Decl;
use crate::{
    utils::{get_args, get_class_name, get_opt_args},
    warning::WarningType,
};

lazy_static! {
    pub static ref BOX_SHADOW: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("box_shadow.ron")).unwrap();
    pub static ref BOX_SHADOW_COLOR: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("../colors.ron")).unwrap();
    pub static ref OPACITY: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("opacity.ron")).unwrap();
}

#[derive(Debug)]
pub enum Effects<'a> {
    BoxShadow(BoxShadow<'a>),
    BoxShadowColor(BoxShadowColor<'a>),
    Opacity(Opacity<'a>),
    MixBlendMode(MixBlendMode),
    BackgroundBlendMode(BackgroundBlendMode),
}

impl<'a> Effects<'a> {
    pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
        let effect = match get_class_name(value) {
            "shadow" => {
                if BOX_SHADOW.contains_key(get_opt_args(value)) {
                    Self::BoxShadow(BoxShadow(get_opt_args(value)))
                } else {
                    Self::BoxShadowColor(BoxShadowColor(get_args(value)?))
                }
            }
            "opacity" => Self::Opacity(Opacity(get_args(value)?)),
            "mix" => {
                let args = get_args(value)?;
                match get_class_name(args) {
                    "blend" => Self::MixBlendMode(MixBlendMode::new(get_args(args)?)?),
                    v => {
                        return Err(WarningType::InvalidArg(
                            v.into(),
                            "Mix Blend Mode".into(),
                            vec!["blend"],
                        ))
                    }
                }
            }
            "bg" => {
                let args = get_args(value)?;
                match get_class_name(args) {
                    "blend" => {
                        Self::BackgroundBlendMode(BackgroundBlendMode::new(get_args(args)?)?)
                    }
                    _ => return Ok(None),
                }
            }
            _ => return Ok(None),
        };

        Ok(Some(effect))
    }

    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::BoxShadow(e) => e.to_decl(),
            Self::BoxShadowColor(e) => e.to_decl(),
            Self::Opacity(e) => e.to_decl(),
            Self::MixBlendMode(e) => Ok(e.to_decl()),
            Self::BackgroundBlendMode(e) => Ok(e.to_decl()),
        }
    }
}
