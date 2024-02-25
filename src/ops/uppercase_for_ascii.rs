use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct UppercaseForAscii {}

impl UppercaseForAscii {
    fn function_for_str(arg: &str) -> String {
        arg.to_uppercase().to_owned()
    }
}

impl traits::OpOne for UppercaseForAscii {
    fn name() -> &'static str { "ascii-uppercase" }
    fn description() -> &'static str { "get locale-independent/ASCII uppercase version of string #1" }

    fn priority(arg: &StrArg) -> f32 {
        let s: &str = arg.into();
        if s == Self::function_for_str(s) {
            0.383
        } else {
            0.683
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let a: &str = arg.into();
        Ok(Self::function_for_str(a).into())
    }
}
