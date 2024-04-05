use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct Camelcase {}

impl traits::Op for Camelcase {
    fn name() -> &'static str { "camelcase" }
    fn usage() -> &'static str { "<#1 string to-camelcase>" }
    fn description() -> &'static str { "turn #1 to lowercase and replace the ASCII character after ' ' or '_' sequences with an uppercase letter" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(if s.matches(' ').count() > 0 || s.matches('_').count() > 0 {
            0.58
        } else {
            0.0
        })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let name: &str = args.get(0)?.try_into()?;

        // NOTE: this algorithm does NOT use the Unicode casing algorithms.
        //       My argument is that this op only has the usecase that a programmer
        //       wants to convert a variable from one format to another.
        //       Identifiers in programs rarely have non-ASCII characters.
        //
        //       But the description clearly describes the ASCII-only situation
        //       and we could change that in the future, if desired.
        let mut just_skipped = false;
        let mut result = String::new();
        for letter in name.chars() {
            if letter == ' ' || letter == '_' {
                just_skipped = true;
            } else if just_skipped {
                result.push(letter.to_ascii_uppercase());
                just_skipped = false;
            } else {
                result.push(letter.to_ascii_lowercase());
                just_skipped = false;
            }
        }

        Ok(result.into())
    }
}
