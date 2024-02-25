use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::{Output,OutputValue};

pub struct Codepoints {}

impl traits::OpOne for Codepoints {
    fn name() -> &'static str { "codepoints" }
    fn description() -> &'static str { "represent string #1 with Unicode codepoints as integers, e.g. [72, 105, 10069]" }

    fn priority(arg: &StrArg) -> f32 {
        let s: &str = arg.into();
        if s.len() / 10 > 50 { 0.2 } else { 0.5 }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg.into();
        let ints = string.chars().map(|c| OutputValue::Int(c as i64)).collect::<Vec<OutputValue>>();
        Ok(Output::HomogeneousList { data: ints, notes: vec![] })
    }
}