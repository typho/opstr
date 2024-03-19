use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct Repeat {}

impl Repeat {
    fn implementation(arg: &str, count: i64, count_arg_id: usize) -> Result<Output, Errors> {
        if count < 0 {
            return Err(Errors::ArgValueError(count_arg_id, format!("cannot repeat {} (negative) times - nonnegative integer required", count)));
        }
        if count > u16::MAX as i64 {
            return Err(Errors::ArgValueError(count_arg_id, "count argument is too large".to_owned()));
        }

        let limit = 65_535i64.saturating_div(arg.chars().count() as i64);
        if count > limit {
            return Err(Errors::ArgValueError(count_arg_id, format!("repeated string exceeds length limit {}", limit)))
        }

        Ok(arg.repeat(count as usize).into())
    }

    fn priority(arg: &str, count: i64) -> f32 {
        let resulting_len = count * (arg.len() as i64);
        if resulting_len > 65_536 {
            0.0
        } else if resulting_len > 1024 {
            0.1
        } else if resulting_len > 512 {
            0.3
        } else if resulting_len > 128 {
            0.43
        } else if resulting_len < 15 {
            0.47
        } else {
            0.8
        }
    }
}

impl traits::Op for Repeat {
    fn name() -> &'static str { "repeat" }
    fn usage() -> &'static str { "<#1 string to-repeat> <#2 int repetitions>" }
    fn description() -> &'static str { "repeat string #1 several (integer #2) times" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let argument1: &str = args.get(0)?.try_into()?;
        let argument2: &str = args.get(1)?.try_into()?;

        if let Ok(count) = argument2.parse::<i64>() {
            return Ok(Self::priority(argument1, count));
        }

        if let Ok(count) = argument1.parse::<i64>() {
            return Ok(Self::priority(argument2, count));
        }

        Ok(0.0)
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let argument1: &str = args.get(0)?.try_into()?;
        let argument2: &str = args.get(1)?.try_into()?;

        if let Ok(count) = argument2.parse::<i64>() {
            return Self::implementation(argument1, count, 1);
        }

        // Ok, maybe the user confused the order of arguments,
        // let us be lenient and accept the reversed order
        if let Ok(count) = argument1.parse::<i64>() {
            return Self::implementation(argument2, count, 0);
        }

        Err(Errors::ArgTypeError(1, "second argument must specify how often the first argument shall be repeated".to_owned()))
    }
}
