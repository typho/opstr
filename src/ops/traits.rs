use crate::errors::Errors;
use crate::input::{StrArg, StrArgs};
use crate::output::Output;

pub(crate) trait OpZero {
    /// `name` gives the name of this function. Must match `[a-zA-Z0-9_-]`
    fn name() -> &'static str;
    /// `description` gives a single line string (try to limit it to 60 characters) describing the functionality
    fn description() -> &'static str;
    /// `priority` returns a guess between 0 and 1 how interesting the result is for the user
    fn priority() -> f32;
    /// `run` implements the function
    fn run() -> Result<Output, Errors>;
}

pub(crate) trait OpOne {
    /// `name` gives the name of this function. Must match `[a-zA-Z0-9_-]`
    fn name() -> &'static str;
    /// `description` gives a single line string (try to limit it to 60 characters) describing the functionality
    fn description() -> &'static str;
    /// `priority` returns a guess between 0 and 1 how interesting the result is for the user
    fn priority(arg: &StrArg) -> f32;
    /// `run` implements the function
    fn run(arg: &StrArg) -> Result<Output, Errors>;
}

pub(crate) trait OpTwo {
    /// `name` gives the name of this function. Must match `[a-zA-Z0-9_-]`
    fn name() -> &'static str;
    /// `description` gives a single line string (try to limit it to 60 characters) describing the functionality
    fn description() -> &'static str;
    /// `priority` returns a guess between 0 and 1 how interesting the result is for the user
    fn priority(arg1: &StrArg, arg2: &StrArg) -> f32;
    /// `run` implements the function
    fn run(arg1: &StrArg, arg2: &StrArg) -> Result<Output, Errors>;
}

pub(crate) trait OpThree {
    /// `name` gives the name of this function. Must match `[a-zA-Z0-9_-]`
    fn name() -> &'static str;
    /// `description` gives a single line string (try to limit it to 60 characters) describing the functionality
    fn description() -> &'static str;
    /// `priority` returns a guess between 0 and 1 how interesting the result is for the user
    fn priority(arg1: &StrArg, arg2: &StrArg, arg3: &StrArg) -> f32;
    /// `run` implements the function
    fn run(arg1: &StrArg, arg2: &StrArg, arg3: &StrArg) -> Result<Output, Errors>;
}

pub(crate) trait OpMulti {
    /// `name` gives the name of this function. Must match `[a-zA-Z0-9_-]`
    fn name() -> &'static str;
    /// `description` gives a single line string (try to limit it to 60 characters) describing the functionality
    fn description() -> &'static str;
    /// `priority` returns a guess between 0 and 1 how interesting the result is for the user
    fn priority(args: &StrArgs) -> f32;
    /// `run` implements the function
    fn run(args: &StrArgs) -> Result<Output, Errors>;
}
