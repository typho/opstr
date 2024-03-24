use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct CountCodepoints {}

impl traits::Op for CountCodepoints {
    fn name() -> &'static str { "count-codepoints" }
    fn usage() -> &'static str { "<#1 string to-analyze>" }
    fn description() -> &'static str { "return the number of Unicode scalars in the Unicode string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }
    fn priority(_args: &Args) -> Result<f32, LibError> { Ok(0.67) }

    fn run(args: &Args) -> Result<Output, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(s.chars().count().into())
    }
}
