mod types;

use types::*;

use crate::class::Decl;
use crate::utils::{get_args, get_class_name};

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref DELAY: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("delay.ron")).unwrap();
    pub static ref DURATION: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("duration.ron")).unwrap();
    pub static ref TIMING_FUNCTION: HashMap<&'static str, &'static str> =
        ron::from_str(include_str!("timing_function.ron")).unwrap();
}

#[derive(Debug)]
pub enum TransitionsAnimation<'a> {
    Transition(Transition),
    Duration(Duration<'a>),
    TimingFunction(TimingFunction<'a>),
    Delay(Delay<'a>),
    Animation(Animation),
}

impl<'a> TransitionsAnimation<'a> {
    pub fn new(value: &'a str) -> Option<Self> {
        let transitions_animation = match get_class_name(value) {
            "transition" => TransitionsAnimation::Transition(Transition::new(value)?),
            "duration" => TransitionsAnimation::Duration(Duration(get_args(value)?)),
            "ease" => TransitionsAnimation::TimingFunction(TimingFunction(get_args(value)?)),
            "delay" => TransitionsAnimation::Delay(Delay(get_args(value)?)),
            "animate" => TransitionsAnimation::Animation(Animation::new(get_args(value)?)?),
            _ => return None,
        };
        Some(transitions_animation)
    }
    pub fn to_decl(self) -> Option<Decl> {
        match self {
            TransitionsAnimation::Transition(s) => s.to_decl(),
            TransitionsAnimation::Duration(s) => s.to_decl(),
            TransitionsAnimation::TimingFunction(s) => s.to_decl(),
            TransitionsAnimation::Delay(s) => s.to_decl(),
            TransitionsAnimation::Animation(s) => s.to_decl(),
        }
    }
}
