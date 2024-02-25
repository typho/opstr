use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct IsWhitespace {}

impl traits::OpOne for IsWhitespace {
    fn name() -> &'static str { "is-whitespace" }
    fn description() -> &'static str { "does the provided string only contain codepoints in the Unicode Whitespace category?" }

    fn priority(_arg: &StrArg) -> f32 { 0.382 }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let s: &str = arg.into();
        for chr in s.chars() {
            if !chr.is_whitespace() {
                return Ok(false.into());
            }
        }
        Ok(true.into())
    }
}
