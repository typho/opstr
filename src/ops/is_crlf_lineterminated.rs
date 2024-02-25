use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct IsCRLFLineTerminated {}

impl traits::OpOne for IsCRLFLineTerminated {
    fn name() -> &'static str { "is-lf-lineterminated" }
    fn description() -> &'static str { "is (U+000D CARRIAGE RETURN)(U+000A LINE FEED) the only sequence causing line breaks in string #1?" }

    fn priority(arg: &StrArg) -> f32 {
        let string: &str = arg.into();
        if string.lines().count() > 1 {
            0.11
        } else {
            0.07
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        // c.f. UAX #14 and is-lf-lineterminated

        let string: &str = arg.into();
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
