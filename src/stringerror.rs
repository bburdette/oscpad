use std::fmt;
use std::error::Error as StdError;
use std::result::Result;

#[derive(Debug)]
pub struct Error {
    message: String
}

impl Error {
    pub fn new(msg: &str) -> Error {
        Error { message: msg.to_string() }
    }
}

pub fn stringBoxErr(s: &str) -> Box<Error> {
  Box::new(Error::new(s))
} 

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        &self.message[..]
    }
}
