use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct IsAscii {}

impl traits::OpOne for IsAscii {
    fn name() -> &'static str { "is-ascii" }
    fn description() -> &'static str { "does this string only contain ASCII characters?" }

    fn priority(arg: &StrArg) -> f32 {
        if <&StrArg as Into<&str>>::into(arg).is_ascii() {
            0.476
        } else {
            0.312
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let s: &str = arg.into();
        Ok(s.is_ascii().into())
    }
}
