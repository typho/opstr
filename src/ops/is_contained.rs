use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct IsContained {}

impl traits::OpTwo for IsContained {
    fn name() -> &'static str { "is-contained" }
    fn description() -> &'static str { "does string #1 contain string #2?" }

    fn priority(_arg1: &StrArg, _arg2: &StrArg) -> f32 {
        0.564
    }

    fn run(arg1: &StrArg, arg2: &StrArg) -> Result<Output, Errors> {
        let s1: &str = arg1.into();
        let s2: &str = arg2.into();
        Ok(s1.contains(s2).into())
    }
}
