use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output,OutputValue};
use crate::range;

pub struct CodepointsUstringList {}

impl CodepointsUstringList {
    fn function_for_chars(s: &str) -> Vec<String> {
        s.chars().map(|x: char| format!("U+{:04X}", x as u32)).collect::<Vec<String>>()
    }
}

impl traits::Op for CodepointsUstringList {
    fn name() -> &'static str { "codepoints-ustring-list" }
    fn usage() -> &'static str { "<#1 string to-decompose-and-represent>" }
    fn description() -> &'static str { "represent string #1 with Unicode codepoints, e.g. [“U+0048”, “U+0069”]" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(if s.len() / 10 > 50 { 0.2 } else { 0.5 })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let arg: &str = args.get(0)?.try_into()?;
        let ustrings = Self::function_for_chars(arg.into());
        let mut uvalues = vec![];
        for ustring in ustrings {
            uvalues.push(OutputValue::SingleLineText(ustring))
        }
        Ok(Output::HomogeneousList { data: uvalues, notes: vec![] })
    }
}