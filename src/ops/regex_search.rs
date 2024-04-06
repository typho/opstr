use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::output::OutputValue;
use crate::range;

use regex;

pub struct RegexSearch {}

impl traits::Op for RegexSearch {
    fn name() -> &'static str { "regex-search" }
    fn usage() -> &'static str { "<#1 string pattern> <#2 string to-match>" }
    fn description() -> &'static str { "does regex pattern #1 occur anywhere inside #2? if so, return matching substring, otherwise empty string" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let pattern: &str = args.get(0)?.try_into()?;
        let mut prio = 0.6;

        if pattern.match_indices(".").count() == 0 {
            prio *= 0.73;
        }
        if pattern.match_indices("*").count() == 0 {
            prio *= 0.56;
        }
        if pattern.match_indices("+").count() == 0 {
            prio *= 0.56;
        }
        if pattern.match_indices("|").count() == 0 {
            prio *= 0.67;
        }
        if pattern.match_indices("(").count() == 0 {
            prio *= 0.33;
        }

        Ok(prio)
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let pattern: &str = args.get(0)?.try_into()?;
        let haystack: &str = args.get(1)?.try_into()?;
        let re = match regex::Regex::new(pattern) {
            Ok(re) => re,
            Err(_) => return Err(LibError::ArgValueError(0, format!("the string '{}' is not a valid regular expression for rust's regex library", pattern))),
        };

        let mut result = OutputValue::from_str("");
        if let Some(captures) = re.captures(haystack) {
            if let Some(grp) = captures.get(0) {
                result = OutputValue::from_str(grp.as_str());
            }
        };

        Ok(Output::Scalar { data: result, notes: vec![] })
    }
}
