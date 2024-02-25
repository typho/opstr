use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct StripCodepointsAtStart {}

impl traits::OpTwo for StripCodepointsAtStart {
    fn name() -> &'static str { "strip-codepoints-at-start" }
    fn description() -> &'static str { "strip codepoints found in string #2 from start of string #1" }

    fn priority(_arg1: &StrArg, arg2: &StrArg) -> f32 {
        let set: &str = arg2.into();
        if !set.is_empty() && set.len() <= 3 {
            0.48
        } else {
            0.178
        }
    }

    fn run(arg1: &StrArg, arg2: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg1.into();
        let set: &str = arg2.into();

        let result = string.trim_start_matches(|c: char| set.contains(c));
        Ok(result.into())
    }
}
