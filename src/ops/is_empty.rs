use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct IsEmpty {}

impl traits::OpOne for IsEmpty {
    fn name() -> &'static str { "is-empty" }
    fn description() -> &'static str { "does this string have length zero?" }
    fn priority(_arg: &StrArg) -> f32 { 0.382 }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let s: &str = arg.into();
        Ok(s.is_empty().into())
    }
}
