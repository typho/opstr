use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output,OutputValue};
use crate::range;

use std::vec;

pub struct Utf8Bytes {}

impl traits::Op for Utf8Bytes {
    fn name() -> &'static str { "utf8-bytes" }
    fn usage() -> &'static str { "<#1 string to-encode>" }
    fn description() -> &'static str { "encode string #1 in UTF-8 and return its bytes" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(if s.len() > 3 {
            0.86
        } else {
            0.67
        })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        let list = Output::HomogeneousList {
            data: s.bytes().map(|e| { OutputValue::Byte(e) }).collect::<Vec<OutputValue>>(),
            notes: vec![],
        };
        Ok(list)
    }
}
