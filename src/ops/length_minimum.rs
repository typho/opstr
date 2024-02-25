use crate::errors::Errors;
use crate::input::StrArgs;
use crate::ops::traits;
use crate::output::Output;

pub struct LengthMinimum {}

impl LengthMinimum {
    fn function_for_str<'s>(args: &[&'s str]) -> &'s str {
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

impl traits::OpMulti for LengthMinimum {
    fn name() -> &'static str { "length-minimum" }
    fn description() -> &'static str { "return the first string with the shortest string" }

    fn priority(args: &StrArgs) -> f32 {
        if args.len() >= 3 {
            0.71
        } else {
            0.33
        }
    }

    fn run(args: &StrArgs) -> Result<Output, Errors> {
        if args.is_empty() {
            return Err(Errors::ArgumentCountError((1..).into(), 0));
        }

        let arguments = args.iter().map(|e| -> &str { e.into() }).collect::<Vec<&str>>();
        Ok(Self::function_for_str(&arguments).into())
    }
}
