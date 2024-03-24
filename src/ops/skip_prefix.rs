use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct SkipPrefix {}

impl traits::Op for SkipPrefix {
    fn name() -> &'static str { "skip-prefix" }
    fn usage() -> &'static str { "<#1 string base> <#2 string prefix>" }
    fn description() -> &'static str { "remove string #2 from the beginning of string #1 if it exists" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args) -> Result<f32, LibError> {
        let s1: &str = args.get(0)?.try_into()?;
        let s2: &str = args.get(1)?.try_into()?;
        Ok(if s1.starts_with(s2) {
            0.637
        } else {
            0.547
        })
    }

    fn run(args: &Args) -> Result<Output, LibError> {
        let s1: &str = args.get(0)?.try_into()?;
        let s2: &str = args.get(1)?.try_into()?;
        if s1.starts_with(s2) {
            match s1.get(s2.len()..) {
                Some(sub) => Ok(sub.into()),
                None => Err(LibError::ArgTypeError(0, format!("Removing the UTF-8 prefix {:?} from a UTF-8 string {:?} should always result in a UTF-8 string - but it did not!", s2, s1))),
            }
        } else {
            Ok(s1.into())
        }
    }
}
