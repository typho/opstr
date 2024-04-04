use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use crate::range;

pub struct SplitByWhitespaces {}

impl traits::Op for SplitByWhitespaces {
    fn name() -> &'static str { "split-by-whitespaces" }
    fn usage() -> &'static str { "<#1 string to-split>" }
    fn description() -> &'static str { "split string #1 by any character of Unicode category Whitespace" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let string: &str = args.get(0)?.try_into()?;
        let length = string.chars().count();
        let ws_count = string.split_whitespace().count();

        Ok(ws_count as f32 / length as f32)
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let string: &str = args.get(0)?.try_into()?;
        let result = string.split_whitespace();
        Ok(Output::HomogeneousList {
            data: result.map(OutputValue::from_str).collect::<Vec<OutputValue>>(),
            notes: vec![],
        })
    }
}
