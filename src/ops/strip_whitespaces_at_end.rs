use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct StripWhitespacesAtEnd {}

impl StripWhitespacesAtEnd {
    fn function_for_str(arg: &str) -> &str {
        arg.trim_end()
    }
}

impl traits::OpOne for StripWhitespacesAtEnd {
    fn name() -> &'static str { "strip-whitespaces-at-end" }
    fn description() -> &'static str { "strip whitespaces from end of string" }

    fn priority(arg: &StrArg) -> f32 {
        let s: &str = arg.into();
        if s == s.trim_end() {
            0.24
        } else {
            0.78
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let a: &str = arg.into();
        Ok(Self::function_for_str(a).into())
    }
}
