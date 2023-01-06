use crate::class::utils::get_value;
use crate::class::Decl;
use crate::utils::get_args;

use super::{DELAY, DURATION, TIMING_FUNCTION};

#[derive(Debug)]
pub enum Transition {
    None,
    All,
    ColorsOpacityShadowTransform,
    Colors,
    Opacity,
    Shadow,
    Transform,
}

impl Transition {
    pub fn new(value: &str) -> Option<Self> {
        let val = match get_args(value) {
            Some("none") => Self::None,
            Some("all") => Self::All,
            Some("colors") => Self::Colors,
            Some("opacity") => Self::Opacity,
            Some("shadow") => Self::Shadow,
            Some("transform") => Self::Transform,
            _ => match value {
                "transition" => Self::ColorsOpacityShadowTransform,
                _ => return None,
            },
        };

        Some(val)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Transition::None => {
                Some(Decl::Lit("transition-property: none"))
            }
            Transition::All => {
                Some(Decl::Triple([
                    "transition-property: all".into(),
                    "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)".into(),
                    "transition-duration: 150ms".into(),
                ]))
            }
            Transition::ColorsOpacityShadowTransform => {
                let mut strings = vec![];
                strings.push(String::from("transition-property: color, background-color, border-color, fill, stroke, opacity, box-shadow, transform, filter, -webkit-text-decoration-color, -webkit-backdrop-filter"));
                strings.push(String::from("transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter"));
                strings.push(String::from("transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, opacity, box-shadow, transform, filter, backdrop-filter, -webkit-text-decoration-color, -webkit-backdrop-filter"));
                strings.push(String::from("transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)"));
                strings.push(String::from("transition-duration: 150ms"));
                Some(Decl::Multiple(strings))
            }
            Transition::Colors => {
                let mut strings = vec![];
                strings.push(String::from("transition-property: color, background-color, border-color, fill, stroke, -webkit-text-decoration-color"));
                strings.push(String::from("transition-property: color, background-color, border-color, text-decoration-color, fill, stroke"));
                strings.push(String::from("transition-property: color, background-color, border-color, text-decoration-color, fill, stroke, -webkit-text-decoration-color"));
                strings.push(String::from("transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)"));
                strings.push(String::from("transition-duration: 150ms"));
                Some(Decl::Multiple(strings))
            }
            Transition::Opacity => {
                Some(Decl::Triple([
                    "transition-property: opacity".into(),
                    "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)".into(),
                    "transition-duration: 150ms".into(),
                ]))
            }
            Transition::Shadow => {
                Some(Decl::Triple([
                    "transition-property: box-shadow".into(),
                    "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)".into(),
                    "transition-duration: 150ms".into(),
                ]))
            }
            Transition::Transform => {
                Some(Decl::Triple([
                    "transition-property: transform".into(),
                    "transition-timing-function: cubic-bezier(0.4, 0, 0.2, 1)".into(),
                    "transition-duration: 150ms".into(),
                ]))
            }
        }
    }
}

#[derive(Debug)]
pub struct Duration<'a>(pub &'a str);

impl<'a> Duration<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &DURATION)?;
        Some(Decl::Single(format!("transition-duration: {}", value)))
    }
}

#[derive(Debug)]
pub struct TimingFunction<'a>(pub &'a str);

impl<'a> TimingFunction<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &TIMING_FUNCTION)?;
        Some(Decl::Single(format!("transition-timing-function: {}", value)))
    }
}

#[derive(Debug)]
pub struct Delay<'a>(pub &'a str);

impl<'a> Delay<'a> {
    pub fn to_decl(self) -> Option<Decl> {
        let value = get_value(self.0, &DELAY)?;
        Some(Decl::Single(format!("transition-delay: {}", value)))
    }
}

#[derive(Debug)]
pub enum Animation {
    None,
    Spin,
    Ping,
    Pulse,
    Bounce,
}

impl Animation {
    pub fn new(arg: &str) -> Option<Self> {
        let val = match arg {
            "none" => Self::None,
            "spin" => Self::Spin,
            "ping" => Self::Ping,
            "pulse" => Self::Pulse,
            "bounce" => Self::Bounce,
            _ => return None,
        };

        Some(val)
    }

    pub fn to_decl(self) -> Option<Decl> {
        match self {
            Self::None => Some(Decl::Lit("animation: none")),
            Self::Spin => Some(Decl::FullClass(
                r#"@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

.animate-spin {
  animation: spin 1s linear infinite;
}"#
                .into(),
            )),
            Self::Ping => Some(Decl::FullClass(
                r#"@keyframes ping {
  75%, 100% {
    transform: scale(2);
    opacity: 0;
  }
}

.animate-ping {
  animation: ping 1s cubic-bezier(0, 0, 0.2, 1) infinite;
}"#
                .into(),
            )),
            Self::Pulse => Some(Decl::FullClass(
                r#"@keyframes pulse {
  50% {
    opacity: .5;
  }
}

.animate-pulse {
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}"#
                .into(),
            )),
            Self::Bounce => Some(Decl::FullClass(
                r#"@keyframes bounce {
  0%, 100% {
    transform: translateY(-25%);
    animation-timing-function: cubic-bezier(0.8,0,1,1);
  }

  50% {
    transform: none;
    animation-timing-function: cubic-bezier(0,0,0.2,1);
  }
}

.animate-bounce {
  animation: bounce 1s infinite;
}"#
                .into(),
            )),
        }
    }
}
