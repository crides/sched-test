use rlua::prelude::*;
use std::fmt;

#[derive(Debug)]
pub struct APIError {
    function: String,
    message: String,
}

#[derive(Debug)]
pub struct Error {
    pub method: String,
    pub kind: ErrorKind,
}

impl std::convert::Into<LuaError> for Error {
    fn into(self) -> LuaError {
        LuaError::external(self)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error in API call `{}`: {}", self.method, self.kind)
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    MissingField { typ: String, field: String },
    InvalidLogType(String),
    LuaError(LuaError),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            ErrorKind::MissingField { typ, field } => {
                write!(f, "Missing field '{}' in type '{}'", field, typ)
            }
            ErrorKind::InvalidLogType(s) => write!(f, "Invalid log type: '{}'", s),
            ErrorKind::LuaError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
