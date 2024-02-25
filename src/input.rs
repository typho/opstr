//! Module implementing a type system for input arguments.
//! 
//! The input is pretty simple and generally considered as
//! string or byteslice. However, additional conversions
//! should syntactically make handling input arguments simpler.

use crate::errors;

/// A simple string according to Unicode as argument.
/// The second element specifies the position of the argument on the CLI.
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct StrArg(String, usize);

impl StrArg {
    pub fn new(arg: &str, arg_id: usize) -> Self {
        Self( arg.to_owned(), arg_id )
    }
}

impl Eq for StrArg {}

impl TryFrom<StrArg> for i64 {
    type Error = errors::Errors;

    fn try_from(value: StrArg) -> Result<Self, Self::Error> {
        match value.0.parse::<Self>() {
            Ok(int) => Ok(int),
            Err(_) => Err(errors::Errors::ArgValueError(value.1, format!("cannot be converted into an integer: '{}'", value.0))),
        }
    }
}

impl From<StrArg> for String {
    fn from(value: StrArg) -> Self {
        value.0.to_owned()
    }
}

// same conversion trait implementations for &StrArg

impl TryFrom<&StrArg> for i64 {
    type Error = errors::Errors;

    fn try_from(value: &StrArg) -> Result<Self, Self::Error> {
        match value.0.parse::<i64>() {
            Ok(int) => Ok(int),
            Err(_) => Err(errors::Errors::ArgValueError(value.1, format!("cannot be converted into an integer: '{}'", value.0))),
        }
    }
}

impl<'s> From<&StrArg> for String {
    fn from(value: &StrArg) -> Self {
        value.0.to_owned()
    }
}

impl<'s> From<&'s StrArg> for &'s str {
    fn from(value: &'s StrArg) -> Self {
        &value.0
    }
}

impl<'s> From<&'s StrArg> for &'s String {
    fn from(value: &'s StrArg) -> Self {
        &value.0
    }
}



/// A sequence of string (acc. to Unicode) arguments
pub(crate) type StrArgs = [StrArg];

/// A sequence of bytes considered as input argument
/// (recommended for some operations where the input
/// does not follow the rules of Unicode)
pub struct BytesArg(Vec<u8>);

/// A sequence of byte slice arguments
pub(crate) type BytesArgs<'s> = &'s [BytesArg];
