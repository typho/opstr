use crate::errors::Errors;
use crate::input::StrArgs;
use crate::ops::traits;
use crate::output::Output;

pub struct IsWhitespaceAgnosticallyEqual {}

impl IsWhitespaceAgnosticallyEqual {
    fn without_whitespaces(s: &str) -> String {
        let mut out = String::new();
        for chr in s.chars().filter(|c| !c.is_whitespace()) {
            out.push(chr);
        }
        out
    }

    fn function_for_strargs(args: &StrArgs) -> bool {
        let mut eq = true;
        let s1: String = Self::without_whitespaces((&args[0]).into());

        for arg in args.iter() {
            let s: String = Self::without_whitespaces(arg.into());
            if s != s1 {
                eq = false;
            }
        }

        eq
    }
}

impl traits::OpMulti for IsWhitespaceAgnosticallyEqual {
    fn name() -> &'static str { "is-whitespace-agnostically-equal" }
    fn description() -> &'static str { "are the strings equal if we ignore any whitespace characters?" }

    fn priority(args: &StrArgs) -> f32 {
        if args.len() <= 1 { return 0.0; }
        if Self::function_for_strargs(args) { 0.41 } else { 0.25 }
    }

    fn run(args: &StrArgs) -> Result<Output, Errors> {
        if args.len() <= 1 { return Err(Errors::ArgumentCountError((2..).into(), args.len())); }
        Ok(Self::function_for_strargs(args).into())
    }
}
