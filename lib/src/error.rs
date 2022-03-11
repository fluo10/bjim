use std::fmt;
use std::error::Error as StdError;
use std::convert::From;


#[derive(Debug, Clone)]
pub struct Error{
    message: String,
    kind: ErrorKind,
}

impl From<ErrorKind> for Error {
    fn from(e: ErrorKind) -> Error {
        Error {
            message: String::new(),
            kind: e,
        }
    }
}

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
            kind: ErrorKind::Simple,
        }
    }
}
impl From<String> for Error{
    fn from(e: String) -> Self {
        Self {
            message: e,
            kind: ErrorKind::Simple,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ErrorKind{
    Simple,
}