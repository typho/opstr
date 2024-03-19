use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct CountSubstring {}

impl traits::Op for CountSubstring {
    fn name() -> &'static str { "count-substring" }
    fn usage() -> &'static str { "<#1 string base> <#2 string search>" }
    fn description() -> &'static str { "how often does string #2 non-overlappingly occur in string #1?" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let string: &str = args.get(0)?.try_into()?;
        let substring: &str = args.get(1)?.try_into()?;
        Ok(if string.find(substring).is_some() {
            0.61
        } else {
            0.51
        })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let string: &str = args.get(0)?.try_into()?;
        let substring: &str = args.get(1)?.try_into()?;
        Ok(string.matches(substring).count().into())
    }
}
