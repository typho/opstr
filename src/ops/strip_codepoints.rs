use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct StripCodepoints {}

impl traits::Op for StripCodepoints {
    fn name() -> &'static str { "strip-codepoints" }
    fn usage() -> &'static str { "<#1 string text> <#2 string codepoints>" }
    fn description() -> &'static str { "strip codepoints found in string #2 from start or end of string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args) -> Result<f32, LibError> {
        let set: &str = args.get(1)?.try_into()?;
        Ok(if !set.is_empty() && set.len() <= 3 {
            0.493
        } else {
            0.179
        })
    }

    fn run(args: &Args) -> Result<Output, LibError> {
        let string: &str = args.get(0)?.try_into()?;
        let set: &str = args.get(1)?.try_into()?;

        let result = string.trim_matches(|c: char| set.contains(c));
        Ok(result.into())
    }
}
