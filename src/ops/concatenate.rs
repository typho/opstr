use crate::errors::Errors;
use crate::input::StrArgs;
use crate::ops::traits;
use crate::output::Output;

pub struct Concatenate {}

impl Concatenate {
    fn function_for_str(args: &[&str]) -> String {
        let mut result = String::new();
        for arg in args {
            result.push_str(arg);
        }
        result
    }

    fn function_for_bitstring(args: &[&[u8]]) -> Vec<u8> {
        let mut result = vec![];
        for arg in args {
            result.extend_from_slice(arg);
        }
        result
    }
}

impl traits::OpMulti for Concatenate {
    fn name() -> &'static str { "concatenate" }
    fn description() -> &'static str { "concatenate all provided strings" }

    fn priority(args: &StrArgs) -> f32 {
        match args.len() {
            0 | 1 => 0.0,
            2 => 0.34,
            _ => 0.724,
        }
    }

    fn run(args: &StrArgs) -> Result<Output, Errors> {
        let arguments = args.iter().map(|e| -> &str { e.into() }).collect::<Vec<&str>>();
        Ok(Self::function_for_str(&arguments).into())
    }
}
