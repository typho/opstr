use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct LowercaseForAscii {}

impl LowercaseForAscii {
    fn function_for_str(arg: &str) -> String {
        arg.to_lowercase().to_owned()
    }
}

impl traits::OpOne for LowercaseForAscii {
    fn name() -> &'static str { "ascii-lowercase" }
    fn description() -> &'static str { "get locale-independent/ASCII lowercase version of string #1" }

    fn priority(arg: &StrArg) -> f32 {
        let s: &str = arg.into();
        if s == Self::function_for_str(s) {
            0.38
        } else {
            0.68
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let a: &str = arg.into();
        Ok(Self::function_for_str(a).into())
    }
}
