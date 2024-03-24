use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct StripWhitespaces {}

impl StripWhitespaces {
    fn function_for_chars(arg: &str) -> &str {
        arg.trim()
    }
}

impl traits::Op for StripWhitespaces {
    fn name() -> &'static str { "strip-whitespaces" }
    fn usage() -> &'static str { "<#1 string text>" }
    fn description() -> &'static str { "strip whitespaces from start and end of string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(if s == s.trim() {
            0.24
        } else {
            0.76
        })
    }

    fn run(args: &Args) -> Result<Output, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(Self::function_for_chars(s).into())
    }
}
