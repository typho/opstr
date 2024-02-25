use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct StripCodepoints {}

impl traits::OpTwo for StripCodepoints {
    fn name() -> &'static str { "strip-codepoints" }
    fn description() -> &'static str { "strip codepoints found in string #2 from start or end of string #1" }

    fn priority(_arg1: &StrArg, arg2: &StrArg) -> f32 {
        let set: &str = arg2.into();
        if !set.is_empty() && set.len() <= 3 {
            0.493
        } else {
            0.179
        }
    }

    fn run(arg1: &StrArg, arg2: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg1.into();
        let set: &str = arg2.into();

        let result = string.trim_matches(|c: char| set.contains(c));
        Ok(result.into())
    }
}
