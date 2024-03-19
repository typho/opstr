use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use crate::range;

pub struct SplitByWhitespacesLimitedAtEnd {}

impl traits::Op for SplitByWhitespacesLimitedAtEnd {
    fn name() -> &'static str { "split-by-whitespaces-limited-at-end" }
    fn usage() -> &'static str { "<#1 string to-split> <#2 int times>" }
    fn description() -> &'static str { "split at most #2 times from the end of the string #1 by any character of Unicode category Whitespace" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let limit: Result<i64, Errors> = args.get(1)?.try_into();

        Ok(match limit {
            Ok(i) => if (1..=6).contains(&i) { 0.60 } else { 0.33 },
            _ => 0.0,
        })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let string: &str = args.get(0)?.try_into()?;
        let limit: Result<i64, Errors> = args.get(1)?.try_into();

        match limit {
            Ok(0) => Ok(Output::HomogeneousList { data: vec![OutputValue::from_str(string)], notes: vec![] }),
            Ok(i) => {
                let mut parts: Vec<&str> = vec![];
                let mut end = string.len();

                for _ in 0..i as usize {
                    match string.rfind(char::is_whitespace) {
                        Some(byte_index) => {
                            parts.push(&string[byte_index..end]);
                            end = byte_index;
                        },
                        None => {
                            parts.push(string);
                            end = 0;
                            break;
                        },
                    }
                }
                if parts.len() == i as usize && end != 0 {
                    parts.push(&string[..end]);
                }
                parts.reverse();

                let list = parts.iter().map(|s| OutputValue::from_str(s)).collect::<Vec<OutputValue>>();
                Ok(Output::HomogeneousList { data: list, notes: vec![] })
            },
            Err(_) => Err(Errors::ArgValueError(2, "second argument must be number of splits".to_owned())),
        }
    }
}
