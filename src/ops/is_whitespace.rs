use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct IsWhitespace {}

impl traits::Op for IsWhitespace {
    fn name() -> &'static str { "is-whitespace" }
    fn usage() -> &'static str { "<#1 string to-analyze>" }
    fn description() -> &'static str { "does the provided string #1 only contain codepoints in the Unicode Whitespace category?" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(_args: &Args, _conf: &Configuration) -> Result<f32, LibError> { Ok(0.382) }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        for chr in s.chars() {
            if !chr.is_whitespace() {
                return Ok(false.into());
            }
        }
        Ok(true.into())
    }
}
