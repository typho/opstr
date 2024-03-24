use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct LengthMaximum {}

impl LengthMaximum {
    fn function_for_chars<'s>(args: &[&'s str]) -> &'s str {
        let mut max: &str = "";
        for arg in args {
            if arg.len() > max.len() {
                max = arg;
            }
        }
        max
    }
}

impl traits::Op for LengthMaximum {
    fn name() -> &'static str { "length-maximum" }
    fn usage() -> &'static str { "[<#1 string to-convert> 1 or more times]" }
    fn description() -> &'static str { "return the first string among the longest strings" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexOpen(1) }

    fn priority(args: &Args) -> Result<f32, LibError> {
        Ok(if args.len() >= 3 {
            0.72
        } else {
            0.34
        })
    }

    fn run(args: &Args) -> Result<Output, LibError> {
        let mut arguments = vec![];
        for arg in args.iter() {
            let s: &str = arg.try_into()?;
            arguments.push(s);
        }
        Ok(Self::function_for_chars(&arguments).into())
    }
}
