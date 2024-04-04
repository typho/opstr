use crate::config::Configuration;
use crate::input::{Arg,Args};
use crate::errors::LibError;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct Center {}

impl traits::Op for Center {
    fn name() -> &'static str { "center" }
    fn usage() -> &'static str { "<#1 string centered-text> [optional <#2 int width> [optional <#3 string repetition-char>]]" }
    fn description() -> &'static str { "put string #1 in the middle of string of width #2 (default 80) repeating char #3 (default #) on both sides" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 3) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let text: &str = args.get(0)?.try_into()?;
        let width: i64 = match args.get(1)?.try_into() {
            Ok(w) => w,
            Err(_) => return Ok(0.0),
        };
        let rep: &str = args.get(2)?.try_into()?;

        let mut score = 0.5;
        score *= if 2 <= text.len() && text.len() <= 30 { 1.2 } else { 0.73 };
        score *= if 10 <= width && width <= 200 { 1.2 } else { 0.53 };
        score *= if rep.chars().count() == 1 { 1.2 } else { 0.46 };

        Ok(score)
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let default_w = Arg::Chars("80".to_owned(), 1);
        let default_rep = Arg::Chars("#".to_owned(), 2);

        let text: &str = args.get(0)?.try_into()?;
        let w: i64 = args.get_or_default(1, &default_w).try_into()?;
        let rep: &str = args.get_or_default(2, &default_rep).try_into()?;

        let width = w as usize;

        if rep.chars().count() != 1 {
            return Err(LibError::ArgTypeError(2, format!("argument needs to be one character / Unicode scalar, but there are {} characters", rep.chars().count())));
        }

        if width <= text.len() + 2 {
            return Ok(text.into());
        }

        let before = (width - text.len() - 2) / 2;
        let after = (width - text.len() - 1) / 2; // NOTE: (-1) is (-2+1) where (+1) computes ceil unlike `before`
        Ok(format!("{} {} {}", rep.repeat(before), text, rep.repeat(after)).into())
    }
}
