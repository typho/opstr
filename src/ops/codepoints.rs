use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output,OutputValue};
use crate::range;

pub struct Codepoints {}

impl traits::Op for Codepoints {
    fn name() -> &'static str { "codepoints" }
    fn usage() -> &'static str { "<#1 string to-decompose-and-represent>" }
    fn description() -> &'static str { "represent string #1 with Unicode codepoints as integers, e.g. [72, 105, 10069]" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(if s.len() / 10 > 50 { 0.2 } else { 0.5 })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let string: &str = args.get(0)?.try_into()?;
        let ints = string.chars().map(|c| OutputValue::Int(c as i64)).collect::<Vec<OutputValue>>();
        Ok(Output::HomogeneousList { data: ints, notes: vec![] })
    }
}