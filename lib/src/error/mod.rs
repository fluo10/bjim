use std::fmt;
use std::error::Error as StdError;
use std::convert::From;
use std::fmt::Result;

pub struct Error{
    message: String,
}

impl Debug for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl StdError for Error {}

impl From<&str> for Error{
    fn from(e: &str) -> Self {
        Self {
            message: e.to_string(),
        }
    }
}
impl From<String> for Error{
    fn from(e: String) -> Self {
        Self {
            message: e,
        }
    }
}