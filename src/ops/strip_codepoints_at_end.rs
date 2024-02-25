use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct StripCodepointsAtEnd {}

impl traits::OpTwo for StripCodepointsAtEnd {
    fn name() -> &'static str { "strip-codepoints-at-end" }
    fn description() -> &'static str { "strip codepoints found in string #2 from end of string #1" }

    fn priority(_arg1: &StrArg, arg2: &StrArg) -> f32 {
        let set: &str = arg2.into();
        if !set.is_empty() && set.len() <= 3 {
            0.41
        } else {
            0.17
        }
    }

    fn run(arg1: &StrArg, arg2: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg1.into();
        let set: &str = arg2.into();

        let result = string.trim_end_matches(|c: char| set.contains(c));
        Ok(result.into())
    }
}
