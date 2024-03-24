use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct IsContained {}

impl traits::Op for IsContained {
    fn name() -> &'static str { "is-contained" }
    fn usage() -> &'static str { "<#1 string base> <#2 string contained>" }
    fn description() -> &'static str { "does string #1 contain string #2?" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(_args: &Args) -> Result<f32, LibError> {
        Ok(0.564)
    }

    fn run(args: &Args) -> Result<Output, LibError> {
        let s1: &str = args.get(0)?.try_into()?;
        let s2: &str = args.get(1)?.try_into()?;
        Ok(s1.contains(s2).into())
    }
}
