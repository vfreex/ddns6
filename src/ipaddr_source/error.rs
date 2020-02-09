use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct AddressDeterminationError(pub String);

impl error::Error for AddressDeterminationError {
}

impl fmt::Display for AddressDeterminationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error determining IP address: {}", self.0.as_str())
    }
}