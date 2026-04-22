use std::fmt;

#[derive(Debug)]
pub enum InterpreterError {
    CreationFailed(String),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message: String = match self {
            Self::CreationFailed(message) => format!("[CreationFailed]: {message}"),
        };

        write!(f, "[InterpreterError] {}", message)
    }
}
