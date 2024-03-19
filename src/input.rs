//! Module implementing a type system for input arguments.
//! 
//! The input is pretty simple and generally considered as
//! string or byteslice. However, additional conversions
//! should syntactically make handling input arguments simpler.

use std::slice;
use crate::errors;

/// An argument for an operation.
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Arg {
    /// A sequence of Unicode codepoints (acc. to Unicode) and the index of this argument.
    Chars(String, usize),
    /// A sequence of bytes (only recommended for non-Unicode content) and the index of this argument.
    /// TODO: bytes support is currently not implemented
    Bytes(Vec<u8>, usize),
}

impl Arg {
    pub fn from_str(arg: &str, arg_id: usize) -> Self {
        Self::Chars( arg.to_owned(), arg_id )
    }

    pub fn from_bytes(arg: &[u8], arg_id: usize) -> Self {
        Self::Bytes( arg.to_owned(), arg_id )
    }

    /// Return the string of this argument or panic
    pub fn str_or_panic(&self) -> &str {
        match self {
            Arg::Chars(s, _) => s,
            Arg::Bytes(_, idx) => panic!("argument #{} cannot be converted into a string", idx),
        }
    }
}

impl Eq for Arg {}

impl TryFrom<&Arg> for i64 {
    type Error = errors::Errors;

    fn try_from(value: &Arg) -> Result<Self, Self::Error> {
        match value {
            Arg::Chars(s, idx) => {
                match s.parse::<i64>() {
                    Ok(int) => Ok(int),
                    Err(_) => Err(errors::Errors::ArgValueError(*idx, format!("cannot be converted into an integer: '{}'", s))),
                }
            },
            Arg::Bytes(_, idx) => Err(errors::Errors::ArgTypeError(*idx, "this argument cannot be converted into an integer".to_owned())),
        }
    }
}

impl<'s> TryFrom<&'s Arg> for &'s str {
    type Error = errors::Errors;

    fn try_from(value: &'s Arg) -> Result<Self, Self::Error> {
        match value {
            Arg::Chars(s, idx) => Ok(s),
            Arg::Bytes(_, idx) => Err(errors::Errors::ArgTypeError(*idx, "this argument cannot be converted into a string".to_owned())),
        }
    }
}


pub struct Args {
    args: Vec<Arg>,
}

impl Args {
    pub fn from(arguments: &[Arg]) -> Args {
        Args { args: arguments.to_vec() }
    }

    /// Return number of arguments
    pub fn len(&self) -> usize {
        self.args.len()
    }

    pub fn get(&self, index: usize) -> Result<&Arg, errors::Errors> {
        match self.args.get(index) {
            Some(arg) => Ok(arg),
            None => Err(errors::Errors::IOError(format!("argument {} does not exist", index))),
        }
    }

    pub fn get_or_default<'s, 'd: 's>(&'s self, index: usize, default: &'d Arg) -> &Arg {
        match self.args.get(index) {
            Some(arg) => arg,
            None => default,
        }
    }

    pub fn iter<'s>(&'s self) -> slice::Iter<'s, Arg> {
        self.args.iter()
    }
}

