mod types;

use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name, get_opt_args};
use crate::warning::WarningType;

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref DELAY: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("delay.ron")).unwrap();
    pub static ref DURATION: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("duration.ron")).unwrap();
    pub static ref TIMING_FUNCTION: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("timing_function.ron")).unwrap();
    pub static ref ANIMATION: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("animation.ron")).unwrap();
}

#[derive(Debug, PartialEq, Hash)]
pub enum TransitionsAnimation<'a> {
    Transition(Transition),
    Duration(Duration<'a>),
    TimingFunction(TimingFunction<'a>),
    Delay(Delay<'a>),
    Animation(Animation<'a>),
}

impl<'a> TransitionsAnimation<'a> {
    pub fn new(value: &'a str) -> Result<Option<Self>, WarningType> {
        let transitions_animation = match get_class_name(value) {
            "transition" => Self::Transition(Transition::new(get_opt_args(value))?),
            "duration" => Self::Duration(Duration(get_args(value)?)),
            "ease" => Self::TimingFunction(TimingFunction(get_args(value)?)),
            "delay" => Self::Delay(Delay(get_args(value)?)),
            "animate" => Self::Animation(Animation(get_args(value)?)),
            _ => return Ok(None),
        };

        Ok(Some(transitions_animation))
    }
    pub fn to_decl(self) -> Result<Decl, WarningType> {
        match self {
            Self::Transition(s) => Ok(s.to_decl()),
            Self::Duration(s) => s.to_decl(),
            Self::TimingFunction(s) => s.to_decl(),
            Self::Delay(s) => s.to_decl(),
            Self::Animation(s) => s.to_decl(),
        }
    }
}
