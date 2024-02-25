use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct GuaranteeSuffix {}

impl traits::OpTwo for GuaranteeSuffix {
    fn name() -> &'static str { "guarantee-suffix" }
    fn description() -> &'static str { "if string #1 does not end with string #2, append it" }

    fn priority(arg1: &StrArg, arg2: &StrArg) -> f32 {
        let string: &str = arg1.into();
        let suffix: &str = arg2.into();
        if string.len() > suffix.len() {
            0.53
        } else {
            0.32
        }
    }

    fn run(arg1: &StrArg, arg2: &StrArg) -> Result<Output, Errors> {
        let mut string: String = arg1.into();
        let suffix: &str = arg2.into();
        if string.ends_with(suffix) {
            Ok(string.into())
        } else {
            string.push_str(suffix);
            Ok(string.into())
        }
    }
}
