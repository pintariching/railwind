use core::fmt;

#[derive(Debug, PartialEq, Hash)]
pub enum WarningType {
    StateNotFound(String),
    ClassNotFound,
    InvalidArg(String, String, Vec<&'static str>),
    ValueNotFound(String),
    InvalidArgCount(String),
    InvalidArbitraryArg(String),
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
            WarningType::InvalidArg(recieved, tried_as, required) => {
                format!(
                    "Could not match class '{class}' to the class '{tried_as}', invalid argument '{recieved}', possible arguments: '{}'",
                    required.join(", ")
                )
            }
            WarningType::ValueNotFound(value) => {
                format!(
                    "Could not match class '{}', argument '{}' could not be found",
                    class, value
                )
            }
            WarningType::InvalidArgCount(value) => {
                format!("Could not match class '{class}', invalid argument count '{value}'")
            }
            WarningType::InvalidArbitraryArg(value) => {
                format!("Could not extract arbitrary value from class '{class}', invalid argument '{value}'")
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
            "Warning on Line: {}, Col: {} in file: '{}'; {}",
            self.position.line, self.position.column, self.position.file, self.message
        ))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    file: String,
    line: usize,
    column: usize,
}

impl Position {
    pub fn new(file: impl ToString, line: usize, column: usize) -> Self {
        Self {
            file: file.to_string(),
            line,
            column,
        }
    }
}

impl From<(usize, usize)> for Position {
    fn from(pos: (usize, usize)) -> Self {
        Self {
            file: String::new(),
            line: pos.0,
            column: pos.1,
        }
    }
}
