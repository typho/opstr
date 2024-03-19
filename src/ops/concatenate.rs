use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct Concatenate {}

impl Concatenate {
    fn function_for_chars(args: &[&str]) -> String {
        let mut result = String::new();
        for arg in args {
            result.push_str(arg);
        }
        result
    }

    #[doc(hidden)]
    #[warn(dead_code)]
    fn function_for_bytes(args: &[&[u8]]) -> Vec<u8> {
        let mut result = vec![];
        for arg in args {
            result.extend_from_slice(arg);
        }
        result
    }
}

impl traits::Op for Concatenate {
    fn name() -> &'static str { "concatenate" }
    fn usage() -> &'static str { "<#1 string to-concatenate> 0 to … times" }
    fn description() -> &'static str { "concatenate all provided strings" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexOpen(0) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        Ok(match args.len() {
            0 | 1 => 0.0,
            2 => 0.34,
            _ => 0.724,
        })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let mut arguments = vec![];
        for arg in args.iter() {
            let s: &str = arg.try_into()?;
            arguments.push(s);
        }
        Ok(Self::function_for_chars(&arguments).into())
    }
}
