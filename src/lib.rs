//! opstr is a library to apply operations to strings.
//! 
//! ## Objects
//! 
//! * `Configuration` represents how the output shall be represented and which locale shall be used for Unicode operations.
//! * `LibError` is an `enum` of all possible error types that can occur
//! * `Arg` represents one input argument (final content, in the case of stdin/file). `Args` is a sequence of them.
//! 
//! ## Concepts
//! 
//! * Operations provide a *priority*. A priority defines how much sense this request made.
//!   A priority of 1.0 means “very likely what you looked for” (on the CLI, the result will be shown on the bottom).
//!   A priority close to 0.0 means “unlikely helpful” (on the CLI, the result will be printed out first).
//!   Results with priority 0.0 make no sense and will be discarded immediately.
//! 
//! ## API
//! 
//! * `list_all_ops` returns the list of supported operations
//! * `list_matching_ops` returns the list of possible operations for the provided arguments
//! * `matcher::run_op` returns the `Output` after running the one operation specified
//! * `matcher::run_matching_ops` runs all operations appropriate for the provided arguments and writes the result to stdout & stderr
//! 
//! ## Notes
//! 
//! 1. The CLI output is always valid UTF-8. This might change in the future, but in the current release, this is the case.
//! 2. `Arg` is either Unicode content (`Chars`) or an arbitrary byte sequence (`Bytes`). `Bytes` is not yet supported.
//! 3. `Output` abstracts the type of result of an operation.
//! 4. `Configuration.syntax` defines which formal grammar shall be used for representation. The default representation for humans does not have a specification.

pub(crate) mod auxiliary;
pub(crate) mod config;
pub(crate) mod errors;
pub(crate) mod input;
pub(crate) mod matcher;
pub(crate) mod ops;
pub(crate) mod range;
pub(crate) mod output;

/// This is a binary blob generated with the icu4x library.
/// It contains various locale-specific convention data.
/// The README file describes how to generate it as part of the release management.
pub(crate) const DEFAULT_LOCALE_DATA: &[u8] = include_bytes!("../data/icu4x_en-US.blob2");

pub use config::Configuration;
pub use errors::LibError;
pub use input::{Arg, Args};
pub use output::Output;
pub use matcher::list_all_ops;
pub use matcher::list_matching_ops;
pub use matcher::run_op;
pub use matcher::run_matching_ops;
