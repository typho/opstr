//! The idea of this library is to implement the utilities for the CLI tool.
//! Specifically, we take `String`s (common) or bytes (for usecases like non-UTF8 strings) as input.
//! Then we check which of all operations make sense for the given number of arguments.
//! We process them to the best of our knowledge and then generate an `Output` instance,
//! which might be as simple as a `String`, but can also be a description list (= dictionary).
//! The output at the CLI is always written as UTF-8.
//! The idea is that this result is then represented in various ways convenient for the user.
//! Furthermore the operations also return a priority score. It defines how much sense this request
//! made. A priority of 1.0 will push the result to the bottom (“likely what you looked for”).
//! A priority of almost 0.0 will push the result to the top (“unlikely helpful”). Results with
//! scores 0.0 will be immediately discarded.

pub(crate) mod auxiliary;
pub(crate) mod config;
pub(crate) mod errors;
pub(crate) mod input;
pub(crate) mod matcher;
pub(crate) mod ops;
pub(crate) mod range;
pub(crate) mod output;

pub use config::Configuration;
pub use errors::Errors;
pub use input::{Arg, Args};
pub use matcher::list_all_ops;
pub use matcher::list_matching_ops;
pub use matcher::run_op;
pub use matcher::run_unspecified_op;
