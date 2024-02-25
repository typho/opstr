use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct StripWhitespacesAtStart {}

impl StripWhitespacesAtStart {
    fn function_for_str(arg: &str) -> &str {
        arg.trim_start()
    }
}

impl traits::OpOne for StripWhitespacesAtStart {
    fn name() -> &'static str { "strip-whitespaces-at-start" }
    fn description() -> &'static str { "strip whitespaces from start of string" }

    fn priority(arg: &StrArg) -> f32 {
        let s: &str = arg.into();
        if s == s.trim_start() {
            0.24
        } else {
            0.79
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let a: &str = arg.into();
        Ok(Self::function_for_str(a).into())
    }
}
