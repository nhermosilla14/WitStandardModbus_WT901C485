use std::fmt;

/// Error types for WitMotion sensor operations
#[derive(Debug)]
pub enum WitError {
    /// Invalid parameters provided
    InvalidParameter(String),
    /// Communication timeout
    Timeout,
    /// CRC checksum mismatch
    CrcMismatch,
    /// Sensor not found during auto-scan
    SensorNotFound,
    /// Generic I/O error
    Io(std::io::Error),
}

impl fmt::Display for WitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WitError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            WitError::Timeout => write!(f, "Communication timeout"),
            WitError::CrcMismatch => write!(f, "CRC checksum mismatch"),
            WitError::SensorNotFound => write!(f, "Sensor not found"),
            WitError::Io(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl std::error::Error for WitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            WitError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for WitError {
    fn from(err: std::io::Error) -> Self {
        WitError::Io(err)
    }
}

/// Result type for WitMotion operations
pub type WitResult<T> = Result<T, WitError>;
