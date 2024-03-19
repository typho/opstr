use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct IsWhitespaceAgnosticallyEqual {}

impl IsWhitespaceAgnosticallyEqual {
    fn without_whitespaces(s: &str) -> String {
        let mut out = String::new();
        for chr in s.chars().filter(|c| !c.is_whitespace()) {
            out.push(chr);
        }
        out
    }

    fn function_for_args(args: &Args) -> Result<bool, Errors> {
        let mut eq = true;
        let s1: String = Self::without_whitespaces(args.get(0)?.try_into()?).into();

        for arg in args.iter() {
            let s: String = Self::without_whitespaces(arg.try_into()?);
            if s != s1 {
                eq = false;
            }
        }

        Ok(eq)
    }
}

impl traits::Op for IsWhitespaceAgnosticallyEqual {
    fn name() -> &'static str { "is-whitespace-agnostically-equal" }
    fn usage() -> &'static str { "<#1 string base> [<#2 string compare> 1 or more times]" }
    fn description() -> &'static str { "are all strings equal if we ignore any whitespace characters?" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexOpen(2) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        if args.len() <= 1 { return Ok(0.0); }
        Ok(if Self::function_for_args(args)? { 0.41 } else { 0.25 })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        Ok(Self::function_for_args(args)?.into())
    }
}
