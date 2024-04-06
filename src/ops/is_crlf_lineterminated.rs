use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct IsCRLFLineTerminated {}

impl traits::Op for IsCRLFLineTerminated {
    fn name() -> &'static str { "is-crlf-lineterminated" }
    fn usage() -> &'static str { "<#1 string lines>" }
    fn description() -> &'static str { "is (U+000D CARRIAGE RETURN)(U+000A LINE FEED) the only sequence causing line breaks in string #1?" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let string: &str = args.get(0)?.try_into()?;
        Ok(if string.lines().count() > 1 {
            0.11
        } else {
            0.07
        })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        // c.f. UAX #14 and is-lf-lineterminated

        let string: &str = args.get(0)?.try_into()?;
        for chr in "\u{000B}\u{000C}\u{2028}\u{2029}\u{0085}".chars() {
            if string.find(chr).is_some() {
                return Ok(false.into());
            }
        }

        // after U+000A, U+000D must occur
        // before U+000D, U+000A must occur
        let mut last_is_000a = false;
        for (prev, next) in string.chars().zip(string.chars()) {
            if prev == '\u{000A}' && next != '\u{000D}' {
                return Ok(false.into());
            } else if prev != '\u{000A}' && next == '\u{000D}' {
                return Ok(false.into());
            }
            last_is_000a = next == '\u{000A}';
        }

        // zip in this configuration skips the last character
        if last_is_000a {
            return Ok(false.into());
        }

        Ok(true.into())
    }
}
