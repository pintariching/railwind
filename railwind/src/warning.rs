use core::fmt;

pub struct Warning {
    message: String,
    line: usize,
    column: usize,
}

impl Warning {
    pub fn new(message: &str, line: usize, column: usize) -> Self {
        Self {
            message: message.into(),
            line,
            column,
        }
    }
}

impl fmt::Display for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!(
            "Warning on Line: {}, Col: {}, {}",
            self.line, self.column, self.message
        ))
    }
}
