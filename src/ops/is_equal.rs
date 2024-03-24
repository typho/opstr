use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct IsEqual {}

impl traits::Op for IsEqual {
    fn name() -> &'static str { "is-equal" }
    fn usage() -> &'static str { "<#1 string base> [<#2 string compare> 1 or more times]" }
    fn description() -> &'static str { "do all Unicode strings have the same byte sequence?" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexOpen(2) }

    fn priority(args: &Args) -> Result<f32, LibError> {
        let mut eq = true;
        let s1: &str = args.get(0)?.try_into()?;

        for arg in args.iter() {
            let s: &str = arg.try_into()?;
            if s != s1 {
                eq = false;
            }
        }

        Ok(if eq { 0.64 } else { 0.52 })
    }

    fn run(args: &Args) -> Result<Output, LibError> {
        let mut eq = true;
        let s1: &str = args.get(0)?.try_into()?;

        for arg in args.iter() {
            let s: &str = arg.try_into()?;
            if s != s1 {
                eq = false;
            }
        }

        Ok(eq.into())
    }
}
