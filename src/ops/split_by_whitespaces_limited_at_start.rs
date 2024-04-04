use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use crate::range;

pub struct SplitByWhitespacesLimitedAtStart {}

impl traits::Op for SplitByWhitespacesLimitedAtStart {
    fn name() -> &'static str { "split-by-whitespaces-limited-at-start" }
    fn usage() -> &'static str { "<#1 string to-split> <#2 int times>" }
    fn description() -> &'static str { "split at most #2 times at the start of the string #1 by any character of Unicode category Whitespace" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let limit: Result<i64, LibError> = args.get(1)?.try_into();

        Ok(match limit {
            Ok(i) => if (1..=6).contains(&i) { 0.61 } else { 0.34 },
            _ => 0.0,
        })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let string: &str = args.get(0)?.try_into()?;
        let limit: Result<i64, LibError> = args.get(1)?.try_into();

        match limit {
            Ok(0) => Ok(Output::HomogeneousList { data: vec![OutputValue::from_str(string)], notes: vec![] }),
            Ok(i) => {
                let parts = string.splitn(i as usize, char::is_whitespace);
                let list = parts.map(OutputValue::from_str).collect::<Vec<OutputValue>>();
                Ok(Output::HomogeneousList { data: list, notes: vec![] })
            },
            Err(_) => Err(LibError::ArgValueError(2, "second argument must be number of splits".to_owned())),
        }
    }
}
