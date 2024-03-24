use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct IsPrefix {}

impl IsPrefix {
    fn function_for_chars(arg1: &str, arg2: &str) -> bool {
        arg1.starts_with(arg2)
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    fn function_for_bytes(arg1: &[u8], arg2: &[u8]) -> bool {
        if arg2.len() > arg1.len() {
            false
        } else {
            &arg1[0..arg2.len()] == arg2
        }
    }
}

impl traits::Op for IsPrefix {
    fn name() -> &'static str { "is-prefix" }
    fn usage() -> &'static str { "<#1 string base> <#2 string prefix>" }
    fn description() -> &'static str { "does string #1 start with string #2?" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args) -> Result<f32, LibError> {
        let s1: &str = args.get(0)?.try_into()?;
        let s2: &str = args.get(1)?.try_into()?;
        if s1.len() < s2.len() {
            return Ok(0.0);
        }

        let p = if s1.starts_with(s2) { 1. } else { 0.6 };
        Ok(match usize::min(s1.len(), s2.len()) {
            0 => 0.0,
            1 => 0.1,
            2 => 0.3,
            _ => 0.4,
        } * p)
    }

    fn run(args: &Args) -> Result<Output, LibError> {
        let s1: &str = args.get(0)?.try_into()?;
        let s2: &str = args.get(1)?.try_into()?;
        Ok(Self::function_for_chars(s1, s2).into())
    }
}
