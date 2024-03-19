use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct RemoveAnsiEscapeSequences {}

impl traits::Op for RemoveAnsiEscapeSequences {
    fn name() -> &'static str { "remove-ansi-escape-sequences" }
    fn usage() -> &'static str { "<#1 string to-simplify>" }
    fn description() -> &'static str { "remove any ANSI X3.64 (also found in ECMA-48/ISO 6429) sequences in string #1 starting with U+001B ESCAPE" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    // TODO implementation
    fn priority(arg: &Args) -> Result<f32, Errors> {
        Ok(0.0)
    }

    // TODO implementation
    fn run(args: &Args) -> Result<Output, Errors> {
        let s: &str = args.get(0)?.try_into()?;
        // \x1B 
        for chr in s.chars() {
            if !chr.is_whitespace() {
                return Ok(false.into());
            }
        }
        Ok(true.into())
    }
}
