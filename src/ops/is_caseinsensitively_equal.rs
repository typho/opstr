// use std::ascii::AsciiExt; // TODO would AsciiExt help us?

use crate::errors::Errors;
use crate::input::StrArgs;
use crate::ops::traits;
use crate::output::Output;

pub struct IsCaseinsensitivelyEqual {}

impl traits::OpMulti for IsCaseinsensitivelyEqual {
    fn name() -> &'static str { "is-caseinsensitively-equal" }
    fn description() -> &'static str { "do the Unicode strings have the same byte sequence after ASCII lowercasing?" }

    fn priority(args: &StrArgs) -> f32 {
        if args.len() <= 1 { return 0.0; }

        let mut eq = true;
        let str1: &str = (&args[0]).into();
        let s1 = str1.to_ascii_lowercase();

        for arg in args.iter() {
            let s: &str = arg.into();
            if s.to_ascii_lowercase() != s1 {
                eq = false;
            }
        }

        if eq { 0.64 } else { 0.52 }
    }

    fn run(args: &StrArgs) -> Result<Output, Errors> {
        if args.len() <= 1 { return Err(Errors::ArgumentCountError((2..).into(), args.len())); }

        let mut eq = true;
        let str1: &str = (&args[0]).into();
        let s1 = str1.to_ascii_lowercase();

        for arg in args.iter() {
            let s: &str = arg.into();
            if s.to_ascii_lowercase() != s1 {
                eq = false;
            }
        }

        Ok(eq.into())
    }
}
