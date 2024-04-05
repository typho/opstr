use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct UppercaseForAscii {}

impl UppercaseForAscii {
    fn function_for_chars(arg: &str) -> String {
        arg.to_uppercase().to_owned()
    }
}

impl traits::Op for UppercaseForAscii {
    fn name() -> &'static str { "uppercase-for-ascii" }
    fn usage() -> &'static str { "<#1 string to-uppercase>" }
    fn description() -> &'static str { "get locale-independent/ASCII uppercase version of string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(if s == Self::function_for_chars(s) {
            0.383
        } else {
            0.683
        })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let a: &str = args.get(0)?.try_into()?;
        Ok(Self::function_for_chars(a).into())
    }
}
