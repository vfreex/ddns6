use std::error::Error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

#[derive(Debug, Clone)]
pub struct Ddns6Error(pub String);

impl Error for Ddns6Error {}

impl fmt::Display for Ddns6Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.as_str())
    }
}