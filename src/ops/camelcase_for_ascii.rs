use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct CamelcaseForAscii {}

impl traits::OpOne for CamelcaseForAscii {
    fn name() -> &'static str { "ascii-camelcase" }
    fn description() -> &'static str { "replace the ASCII character after ' ' or '_' sequences with an uppercase letter" }

    fn priority(arg: &StrArg) -> f32 {
        let s: &str = arg.into();
        if s.matches(' ').count() > 0 || s.matches('_').count() > 0 {
            0.58
        } else {
            0.0
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let name: &str = arg.into();

        let mut just_skipped = false;
        let mut result = String::new();
        for letter in name.chars() {
            if letter == ' ' || letter == '_' {
                just_skipped = true;
            } else if just_skipped {
                result.push(letter.to_ascii_uppercase());
                just_skipped = false;
            } else {
                result.push(letter);
                just_skipped = false;
            }
        }

        Ok(result.into())
    }
}
