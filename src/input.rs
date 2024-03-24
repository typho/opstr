//! Module implementing a type system for input arguments.
//! 
//! The input is pretty simple and generally considered as
//! string or byteslice. However, additional conversions
//! should syntactically make handling input arguments simpler.

use std::slice;
use crate::errors;

/// An argument for an operation
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Arg {
    /// A sequence of Unicode codepoints (acc. to Unicode) and the index of this argument.
    Chars(String, usize),
    /// A sequence of bytes (only recommended for non-Unicode content) and the index of this argument.
    /// NOTE: bytes support is currently not implemented
    Bytes(Vec<u8>, usize),
}

impl Arg {
    /// Construct a Unicode string `Arg` given the provided string, and the zero-based index of this argument
    pub fn from_str(arg: &str, arg_id: usize) -> Self {
        Self::Chars( arg.to_owned(), arg_id )
    }

    /// Construct a byteslice `Arg` given the provided bytes, and the zero-based index of this argument
    #[doc(hidden)]
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
    type Error = errors::LibError;

    fn try_from(value: &Arg) -> Result<Self, Self::Error> {
        match value {
            Arg::Chars(s, idx) => {
                match s.parse::<i64>() {
                    Ok(int) => Ok(int),
                    Err(_) => Err(errors::LibError::ArgValueError(*idx, format!("cannot be converted into an integer: '{}'", s))),
                }
            },
            Arg::Bytes(_, idx) => Err(errors::LibError::ArgTypeError(*idx, "this argument cannot be converted into an integer".to_owned())),
        }
    }
}

impl<'s> TryFrom<&'s Arg> for &'s str {
    type Error = errors::LibError;

    fn try_from(value: &'s Arg) -> Result<Self, Self::Error> {
        match value {
            Arg::Chars(s, _idx) => Ok(s),
            Arg::Bytes(_, idx) => Err(errors::LibError::ArgTypeError(*idx, "this argument cannot be converted into a string".to_owned())),
        }
    }
}

/// An ordered container for arguments
#[derive(Clone, Debug, PartialEq)]
pub struct Args {
    args: Vec<Arg>,
}

impl Args {
    /// Construct instance with provided `Arg` instances
    pub fn from(arguments: &[Arg]) -> Args {
        Args { args: arguments.to_vec() }
    }

    /// Consumes provided `Arg` instance and adds it as final element to the container
    pub fn add(&mut self, arg: Arg) {
        self.args.push(arg);
    }

    /// Return number of arguments
    pub fn len(&self) -> usize {
        self.args.len()
    }

    /// Return the (zero-based) index-th element, or return `Err` if it does not exist
    pub fn get(&self, index: usize) -> Result<&Arg, errors::LibError> {
        match self.args.get(index) {
            Some(arg) => Ok(arg),
            None => Err(errors::LibError::IOError(format!("argument {} does not exist", index))),
        }
    }

    /// Return the (zero-based) index-th element or return the provided `default` `Arg`
    pub fn get_or_default<'s, 'd: 's>(&'s self, index: usize, default: &'d Arg) -> &Arg {
        match self.args.get(index) {
            Some(arg) => arg,
            None => default,
        }
    }

    /// Return an iterator over the container
    pub fn iter<'s>(&'s self) -> slice::Iter<'s, Arg> {
        self.args.iter()
    }
}

