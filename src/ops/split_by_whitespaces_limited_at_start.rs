use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::{Output, OutputValue};

pub struct SplitByWhitespacesLimitedAtStart {}

impl traits::OpTwo for SplitByWhitespacesLimitedAtStart {
    fn name() -> &'static str { "split-by-whitespaces-limited-at-start" }
    fn description() -> &'static str { "split at most #2 times at the start of the string #1 by any character of Unicode category Whitespace" }

    fn priority(_arg1: &StrArg, arg2: &StrArg) -> f32 {
        let limit: Result<i64, Errors> = arg2.try_into();

        match limit {
            Ok(i) => if (1..=6).contains(&i) { 0.61 } else { 0.34 },
            _ => 0.0,
        }
    }

    fn run(arg1: &StrArg, arg2: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg1.into();
        let limit: Result<i64, Errors> = arg2.try_into();

        match limit {
            Ok(0) => Ok(Output::HomogeneousList { data: vec![OutputValue::from_str(string)], notes: vec![] }),
            Ok(i) => {
                let parts = string.splitn(i as usize, char::is_whitespace);
                let list = parts.map(OutputValue::from_str).collect::<Vec<OutputValue>>();
                Ok(Output::HomogeneousList { data: list, notes: vec![] })
            },
            Err(_) => Err(Errors::ArgValueError(2, "second argument must be number of splits".to_owned())),
        }
    }
}
