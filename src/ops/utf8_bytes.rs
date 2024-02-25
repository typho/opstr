use std::vec;

use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::{Output,OutputValue};

pub struct Utf8Bytes {}

impl traits::OpOne for Utf8Bytes {
    fn name() -> &'static str { "utf8-bytes" }
    fn description() -> &'static str { "encode string #1 in UTF-8 and return its bytes" }

    fn priority(arg: &StrArg) -> f32 {
        let s: &str = arg.into();
        if s.len() > 3 {
            0.86
        } else {
            0.67
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let s: &str = arg.into();
        let list = Output::HomogeneousList {
            data: s.bytes().map(|e| { OutputValue::Byte(e) }).collect::<Vec<OutputValue>>(),
            notes: vec![],
        };
        Ok(list)
    }
}
