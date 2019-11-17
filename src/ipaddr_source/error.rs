use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct AddressDeterminationError;

impl error::Error for AddressDeterminationError {

}

impl fmt::Display for AddressDeterminationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}