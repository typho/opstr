use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct CountSubstring {}

impl traits::OpTwo for CountSubstring {
    fn name() -> &'static str { "count-substring" }
    fn description() -> &'static str { "how often does string #2 non-overlappingly occur in string #1?" }

    fn priority(arg1: &StrArg, arg2: &StrArg) -> f32 {
        let string: &str = arg1.into();
        let substring: &str = arg2.into();
        if string.find(substring).is_some() {
            0.61
        } else {
            0.51
        }
    }

    fn run(arg1: &StrArg, arg2: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg1.into();
        let substring: &str = arg2.into();
        Ok(string.matches(substring).count().into())
    }
}
