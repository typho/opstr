use crate::errors::Errors;
use crate::input::StrArgs;
use crate::ops::traits;
use crate::output::Output;

pub struct Join {}

impl traits::OpMulti for Join {
    fn name() -> &'static str { "join" }
    fn description() -> &'static str { "join all following strings with string #1" }

    fn priority(args: &StrArgs) -> f32 {
        match args.len() {
            0 => 0.0,
            1 => 0.09,
            2 => 0.11,
            _ => args.len() as f32 / (args.len() as f32 + 1.),
        }
    }

    fn run(args: &StrArgs) -> Result<Output, Errors> {
        if args.is_empty() {
            return Err(Errors::ArgumentCountError((3..).into(), args.len()))
        }

        let sep = (&args[0]).into();
        let mut result = String::new();
        if args.len() >= 2 {
            let s: &str = (&args[1]).into();
            result.push_str(s);
        }
        for arg in args.iter().skip(2) {
            let s: &str = arg.into();
            result.push_str(sep);
            result.push_str(s);
        }

        Ok(result.into())
    }
}
