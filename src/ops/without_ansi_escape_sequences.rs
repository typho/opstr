use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct WithoutAnsiEscapeSequences {}

impl traits::OpOne for WithoutAnsiEscapeSequences {
    fn name() -> &'static str { "without-ansi-escape-sequences" }
    fn description() -> &'static str { "remove any ANSI X3.64 (also found in ECMA-48/ISO 6429) sequences starting with U+001B ESCAPE" }

    fn priority(_arg: &StrArg) -> f32 {

    }

    // \x1B 

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
