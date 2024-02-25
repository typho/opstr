use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct CountCodepoints {}

impl traits::OpOne for CountCodepoints {
    fn name() -> &'static str { "count-codepoints" }
    fn description() -> &'static str { "return the number of Unicode scalars in this Unicode string" }
    fn priority(_arg: &StrArg) -> f32 { 0.67 }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let s: &str = arg.into();
        Ok(s.chars().count().into())
    }
}
