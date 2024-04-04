use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct IsAscii {}

impl traits::Op for IsAscii {
    fn name() -> &'static str { "is-ascii" }
    fn usage() -> &'static str { "<#1 string to-analyze>" }
    fn description() -> &'static str { "does this string #1 only contain ASCII characters?" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(if s.is_ascii() {
            0.476
        } else {
            0.312
        })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(s.is_ascii().into())
    }
}
