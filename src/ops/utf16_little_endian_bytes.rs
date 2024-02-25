use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::{Output,OutputValue};

pub struct Utf16LittleEndianBytes {}

impl traits::OpOne for Utf16LittleEndianBytes {
    fn name() -> &'static str { "utf16-little-endian-bytes" }
    fn description() -> &'static str { "encode string #1 in UTF-16 and return its bytes in little endian order" }

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
        let mut list = vec![];
        for two_bytes in s.encode_utf16() {
            for byte in two_bytes.to_le_bytes() {
                list.push(OutputValue::Byte(byte));
            }
        }
        Ok(Output::HomogeneousList { data: list, notes: vec![] })

    }
}
