use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct StripWhitespacesAtEnd {}

impl StripWhitespacesAtEnd {
    fn function_for_chars(arg: &str) -> &str {
        arg.trim_end()
    }
}

impl traits::Op for StripWhitespacesAtEnd {
    fn name() -> &'static str { "strip-whitespaces-at-end" }
    fn usage() -> &'static str { "<#1 string text>" }
    fn description() -> &'static str { "strip whitespaces from end of string" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(if s == s.trim_end() {
            0.24
        } else {
            0.78
        })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let a: &str = args.get(0)?.try_into()?;
        Ok(Self::function_for_chars(a).into())
    }
}
