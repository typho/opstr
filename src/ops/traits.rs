use crate::errors::LibError;
use crate::input::Args;
use crate::range::Range;
use crate::output::Output;

pub(crate) trait Op {
    /// `name` gives the name of this function. Must match `[a-zA-Z0-9_-]`
    fn name() -> &'static str;
    /// `usage` returns a string describing which arguments shall be provided.
    /// This string will be shown if the user fails to provide the correct number of arguments.
    fn usage() -> &'static str;
    /// `description` gives a single line string describing the functionality.
    /// Try to limit it to 60 characters.
    fn description() -> &'static str;
    /// `acceptable_number_of_arguments` returns which number of arguments can be processed by `run`
    fn acceptable_number_of_arguments() -> Range;
    /// `priority` returns a guess between 0.0 and 1.0 how interesting the result
    /// - given the arguments - is for the user. The implementation can assume
    /// that `args.len()` is within the range returned by `acceptable_number_of_arguments`.
    fn priority(args: &Args) -> Result<f32, LibError>;
    /// `run` implements the operation. The implementation can assume that `args.len()`
    /// is within the range returned by `acceptable_number_of_arguments`.
    fn run(args: &Args) -> Result<Output, LibError>;
}

