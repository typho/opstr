use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output,OutputValue};
use crate::range;

pub struct Utf16LittleEndianBytes {}

impl traits::Op for Utf16LittleEndianBytes {
    fn name() -> &'static str { "utf16-little-endian-bytes" }
    fn usage() -> &'static str { "<#1 string to-encode>" }
    fn description() -> &'static str { "encode string #1 in UTF-16 and return its bytes in little endian order" }
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
        let mut list = vec![];
        for two_bytes in s.encode_utf16() {
            for byte in two_bytes.to_le_bytes() {
                list.push(OutputValue::Byte(byte));
            }
        }
        Ok(Output::HomogeneousList { data: list, notes: vec![] })
    }
}
