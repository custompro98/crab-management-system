use std::fmt::Display;

#[derive(Debug)]
pub enum ValidationError {
    TimeParseError(chrono::ParseError),
    ValidatorError(validator::ValidationErrors),
    FailedPrecondition(String),
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::TimeParseError(e) => write!(f, "{}", e),
            ValidationError::ValidatorError(e) => write!(f, "{}", e),
            ValidationError::FailedPrecondition(e) => write!(f, "{}", e),
        }
    }
}

impl From<chrono::ParseError> for ValidationError {
    fn from(v: chrono::ParseError) -> Self {
        Self::TimeParseError(v)
    }
}

impl From<validator::ValidationErrors> for ValidationError {
    fn from(v: validator::ValidationErrors) -> Self {
        Self::ValidatorError(v)
    }
}
