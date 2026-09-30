use nemesis::NemesisError;
use std::fmt;

/// Crate-level Result type using `NemesisError`
pub type $NAMEResult<T> = Result<T, NemesisError>;

#[derive(Debug)]
pub enum $NAMEError {
    Generic(String),
    Io(std::io::Error),
}

impl std::error::Error for $NAMEError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SSCCError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<$NAMEError> for NemesisError {
    fn from(err: SSCCError, source: &str) -> Self {
        NemesisError::new(&format!("$NAME Error raised in: '{}'", source), err)
    }
}

impl fmt::Display for $NAMEError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            $NAMEError::Generic(msg) => write!(f, "{}", msg),
            $NAMEError::Io(err) => write!(f, "{}", err),
        }
    }
}

impl From<std::io::Error> for $NAMEError {
    fn from(err: std::io::Error) -> Self {
        $NAMEError::Io(err)
    }
}
