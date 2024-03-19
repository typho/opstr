use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct StripWhitespacesAtStart {}

impl StripWhitespacesAtStart {
    fn function_for_chars(arg: &str) -> &str {
        arg.trim_start()
    }
}

impl traits::Op for StripWhitespacesAtStart {
    fn name() -> &'static str { "strip-whitespaces-at-start" }
    fn usage() -> &'static str { "<#1 string text>" }
    fn description() -> &'static str { "strip whitespaces from start of string" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(if s == s.trim_start() {
            0.24
        } else {
            0.79
        })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(Self::function_for_chars(s).into())
    }
}
