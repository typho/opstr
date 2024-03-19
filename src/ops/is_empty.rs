use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct IsEmpty {}

impl traits::Op for IsEmpty {
    fn name() -> &'static str { "is-empty" }
    fn usage() -> &'static str { "<#1 string to-analyze>" }
    fn description() -> &'static str { "does this string #1 have length zero?" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }
    fn priority(_args: &Args) -> Result<f32, Errors> { Ok(0.382) }

    fn run(args: &Args) -> Result<Output, Errors> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(s.is_empty().into())
    }
}
