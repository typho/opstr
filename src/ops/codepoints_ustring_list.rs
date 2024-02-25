use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::{Output,OutputValue};

pub struct CodepointsUstringList {}

impl CodepointsUstringList {
    fn function_for_str(s: &str) -> Vec<String> {
        s.chars().map(|x: char| format!("U+{:04X}", x as u32)).collect::<Vec<String>>()
    }
}

impl traits::OpOne for CodepointsUstringList {
    fn name() -> &'static str { "codepoints-ustring-list" }
    fn description() -> &'static str { "represent string #1 with Unicode codepoints, e.g. [“U+0048”, “U+0069”]" }

    fn priority(arg: &StrArg) -> f32 {
        let s: &str = arg.into();
        if s.len() / 10 > 50 { 0.2 } else { 0.5 }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let ustrings = Self::function_for_str(arg.into());
        let mut uvalues = vec![];
        for ustring in ustrings {
            uvalues.push(OutputValue::SingleLineText(ustring))
        }
        Ok(Output::HomogeneousList { data: uvalues, notes: vec![] })
    }
}