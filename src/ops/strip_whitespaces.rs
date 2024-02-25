use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct StripWhitespaces {}

impl StripWhitespaces {
    fn function_for_str(arg: &str) -> &str {
        arg.trim()
    }
}

impl traits::OpOne for StripWhitespaces {
    fn name() -> &'static str { "strip-whitespaces" }
    fn description() -> &'static str { "strip whitespaces from start and end of string" }

    fn priority(arg: &StrArg) -> f32 {
        let s: &str = arg.into();
        if s == s.trim() {
            0.24
        } else {
            0.76
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let a: &str = arg.into();
        Ok(Self::function_for_str(a).into())
    }
}
