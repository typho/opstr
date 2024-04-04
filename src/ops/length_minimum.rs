use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct LengthMinimum {}

impl LengthMinimum {
    fn function_for_chars<'s>(args: &[&'s str]) -> &'s str {
        if args.is_empty() { return ""; }

        let mut min: &str = args[0];
        for arg in args {
            if arg.len() < min.len() {
                min = arg;
            }
        }
        min
    }
}

impl traits::Op for LengthMinimum {
    fn name() -> &'static str { "length-minimum" }
    fn usage() -> &'static str { "[<#1 string to-convert> 1 or more times]" }
    fn description() -> &'static str { "return the first string among the shortest strings" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexOpen(1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        Ok(if args.len() >= 3 {
            0.71
        } else {
            0.33
        })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let mut arguments = vec![];
        for arg in args.iter() {
            let s: &str = arg.try_into()?;
            arguments.push(s);
        }
        Ok(Self::function_for_chars(&arguments).into())
    }
}
