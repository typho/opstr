use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct LowercaseForAscii {}

impl LowercaseForAscii {
    fn function_for_chars(arg: &str) -> String {
        arg.to_lowercase().to_owned()
    }
}

impl traits::Op for LowercaseForAscii {
    fn name() -> &'static str { "lowercase-for-ascii" }
    fn usage() -> &'static str { "<#1 string to-convert>" }
    fn description() -> &'static str { "get locale-independent/ASCII lowercase version of string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(if s == Self::function_for_chars(s) {
            0.38
        } else {
            0.68
        })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let a: &str = args.get(0)?.try_into()?;
        Ok(Self::function_for_chars(a).into())
    }
}
