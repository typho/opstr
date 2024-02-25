use crate::errors::Errors;
use crate::input::StrArgs;
use crate::ops::traits;
use crate::output::Output;

pub struct LengthMaximum {}

impl LengthMaximum {
    fn function_for_str<'s>(args: &[&'s str]) -> &'s str {
        let mut max: &str = "";
        for arg in args {
            if arg.len() > max.len() {
                max = arg;
            }
        }
        max
    }
}

impl traits::OpMulti for LengthMaximum {
    fn name() -> &'static str { "length-maximum" }
    fn description() -> &'static str { "return the first string with the longest string" }

    fn priority(args: &StrArgs) -> f32 {
        if args.len() >= 3 {
            0.72
        } else {
            0.34
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
