use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct IsLFLineTerminated {}

impl traits::Op for IsLFLineTerminated {
    fn name() -> &'static str { "is-lf-lineterminated" }
    fn usage() -> &'static str { "<#1 string lines>" }
    fn description() -> &'static str { "is U+000A LINE FEED the only character causing line breaks in string #1?" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let string: &str = args.get(0)?.try_into()?;
        Ok(if string.lines().count() > 1 {
            0.12
        } else {
            0.08
        })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        // line break causing characters can be found in categories c.f. UAX #14 …
        //   BK    Mandatory Break   Cause a line break (after)
        //   CR    Carriage Return   Cause a line break (after), except between CR and LF
        //   LF    Line Feed         Cause a line break (after)
        //   NL    Next Line         Cause a line break (after)
        // the following characters are included in these categories excluding U+000A LINE FEED

        let string: &str = args.get(0)?.try_into()?;
        for chr in "\u{000B}\u{000C}\u{2028}\u{2029}\u{000D}\u{0085}".chars() {
            if let Some(_) = string.find(chr) {
                return Ok(false.into());
            }
        }

        Ok(true.into())
    }
}
