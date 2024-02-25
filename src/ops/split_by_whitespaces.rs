use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::{Output, OutputValue};

pub struct SplitByWhitespaces {}

impl traits::OpOne for SplitByWhitespaces {
    fn name() -> &'static str { "split-by-whitespaces" }
    fn description() -> &'static str { "split string #1 by any character of Unicode category Whitespace" }

    fn priority(arg: &StrArg) -> f32 {
        let string: &str = arg.into();
        let length = string.chars().count();
        let ws_count = string.split_whitespace().count();

        ws_count as f32 / length as f32
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg.into();
        let result = string.split_whitespace();
        Ok(Output::HomogeneousList {
            data: result.map(OutputValue::from_str).collect::<Vec<OutputValue>>(),
            notes: vec![],
        })
    }
}
