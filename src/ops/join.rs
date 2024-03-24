use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct Join {}

impl traits::Op for Join {
    fn name() -> &'static str { "join" }
    fn usage() -> &'static str { "<#1 string separator> [<#2 string to-join> 0 or more times]" }
    fn description() -> &'static str { "join all following strings with string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexOpen(1) }

    fn priority(args: &Args) -> Result<f32, LibError> {
        Ok(match args.len() {
            0 => 0.0,
            1 => 0.09,
            2 => 0.11,
            _ => args.len() as f32 / (args.len() as f32 + 1.),
        })
    }

    fn run(args: &Args) -> Result<Output, LibError> {
        let sep = args.get(0)?.try_into()?;
        let mut result = String::new();
        if args.len() >= 2 {
            let s: &str = args.get(1)?.try_into()?;
            result.push_str(s);
        }
        for arg in args.iter().skip(2) {
            let s: &str = arg.try_into()?;
            result.push_str(sep);
            result.push_str(s);
        }

        Ok(result.into())
    }
}
