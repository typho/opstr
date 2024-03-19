use std::fmt;
use std::io;

use crate::range::Range;

// TODO: rename to "LibErrors"?
#[derive(Debug, Clone)]
pub enum Errors {
    /// A value error for a CLI argument, e.g. "--radix" receives a negative number;
    /// specified by (CLI argument name, error message)
    CLIValueError(&'static str, String),
    /// A value error for a CLI argument, e.g. "--radix" receives a floating point number;
    /// specified by (CLI argument name, error message)
    CLITypeError(&'static str, String),
    /// A value error for a positional argument, e.g. "repeat" receives a negative number as argument;
    /// specified by (zero-based argument ID, error message)
    ArgValueError(usize, String),
    /// A value error for a positional argument, e.g. "repeat" receives a boolean as argument;
    /// specified by (zero-based argument ID, error message)
    ArgTypeError(usize, String),
    /// The number of arguments received does not match this function's signature.
    /// Specified by (expected, actual) number of arguments and an optional error message.
    ArgumentCountError(Range, usize, Option<String>),
    /// The user-provided operation name is unknown;
    /// specified by the op name.
    UnknownOp(String),
    /// Internal error, where generated data does not satisfy required format;
    /// specified by an error message.
    InvalidData(String),
    /// internal error related to I/O; specified by an error message.
    IOError(String),
}


impl From<io::Error> for Errors {
    fn from(err: io::Error) -> Self {
        Errors::IOError(err.to_string())
    }
}

impl fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CLIValueError(name, errmsg) => write!(f, "invalid CLI argument for '--{}': {}", name, errmsg),
            Self::CLITypeError(name, errmsg) => write!(f, "invalid type for '--{}': {}", name, errmsg),
            Self::ArgTypeError(position, errmsg) => write!(f, "invalid type for argument #{}: {}", position + 1, errmsg),
            Self::ArgValueError(position, errmsg) => write!(f, "invalid CLI argument for #{}: {}", position + 1, errmsg),
            Self::ArgumentCountError(expected, actual, opt_errmsg) => match opt_errmsg {
                Some(msg) => write!(f, "invalid number of CLI arguments, expected {} got {}; provide these arguments: {}", expected, actual, msg),
                None => write!(f, "invalid number of CLI arguments, expected {} got {}", expected, actual),
            },
            Self::UnknownOp(op) => write!(f, "unknown operation '{}'", op),
            Self::InvalidData(msg) => write!(f, "internal data error: '{}'", msg),
            Self::IOError(msg) => write!(f, "I/O error: '{}'", msg),
        }
    }
}
