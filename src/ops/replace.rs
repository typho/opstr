use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct Replace {}

impl Replace {
    fn function_for_chars(arg1: &str, arg2: &str, arg3: &str) -> String {
        arg1.replace(arg2, arg3)
    }
}

impl traits::Op for Replace {
    fn name() -> &'static str { "replace" }
    fn usage() -> &'static str { "<#1 string base> <#2 string to-search> <#3 string replacement>" }
    fn description() -> &'static str { "replace string #2 with string #3 in string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(3, 3) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let base_arg: &str = args.get(0)?.try_into()?;
        let search_arg: &str = args.get(1)?.try_into()?;

        let factor = match base_arg.matches(search_arg).count() {
            0 => 0.03,
            1..=6 => 1.0,
            _ => 0.7,
        };

        Ok(if search_arg.len() > base_arg.len() {
            0.1 * factor
        } else if search_arg.len() * 2 > base_arg.len() {
            0.3 * factor
        } else {
            let diff = base_arg.len().abs_diff(search_arg.len()) as f32;
            0.8 * (diff / (diff + 1.0)) * factor
        })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let s1: &str = args.get(0)?.try_into()?;
        let s2: &str = args.get(1)?.try_into()?;
        let s3: &str = args.get(2)?.try_into()?;

        Ok(Self::function_for_chars(s1, s2, s3).into())
    }
}
