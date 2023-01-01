use core::fmt;

#[derive(Debug)]
pub enum WarningType {
    StateNotFound(String),
    ClassNotFound,

    /// (recieved, required)
    InvalidArg(String, Vec<&'static str>),

    /// (value)
    ValueNotFound(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Warning {
    message: String,
    position: Position,
}

impl Warning {
    pub fn new(class: &str, position: &Position, warning_type: WarningType) -> Self {
        let message = match warning_type {
            WarningType::StateNotFound(recieved) => format!(
                "Could not match state at class '{}', '{}' is not a valid state",
                class, recieved
            ),
            WarningType::ClassNotFound => format!("Could not match class '{}'", class),
            WarningType::InvalidArg(recieved, required) => {
                format!(
                    "Could not match class '{}', invalid argument '{}', possible arguments: '{}'",
                    class,
                    recieved,
                    required.join(", ")
                )
            }
            WarningType::ValueNotFound(value) => {
                format!(
                    "Could not match class '{}','{}' could not be found",
                    class, value
                )
            }
        };

        Self {
            message,
            position: position.clone(),
        }
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "Warning on Line: {}, Col: {}; {}",
            self.position.line, self.position.column, self.message
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

impl From<(usize, usize)> for Position {
    fn from(pos: (usize, usize)) -> Self {
        Self {
            line: pos.0,
            column: pos.1,
        }
    }
}
