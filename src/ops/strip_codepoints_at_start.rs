use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct StripCodepointsAtStart {}

impl traits::Op for StripCodepointsAtStart {
    fn name() -> &'static str { "strip-codepoints-at-start" }
    fn usage() -> &'static str { "<#1 string text> <#2 string codepoints>" }
    fn description() -> &'static str { "strip codepoints found in string #2 from start of string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let set: &str = args.get(1)?.try_into()?;
        Ok(if !set.is_empty() && set.len() <= 3 {
            0.48
        } else {
            0.178
        })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let string: &str = args.get(0)?.try_into()?;
        let set: &str = args.get(1)?.try_into()?;

        let result = string.trim_start_matches(|c: char| set.contains(c));
        Ok(result.into())
    }
}
