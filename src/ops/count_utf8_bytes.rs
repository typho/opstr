use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct CountUtf8Bytes {}

impl traits::OpOne for CountUtf8Bytes {
    fn name() -> &'static str { "count-utf8-bytes" }
    fn description() -> &'static str { "encode string #1 in UTF-8 and return its number of bytes" }
    fn priority(arg: &StrArg) -> f32 { 0.56 }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let s: &str = arg.into();
        Ok(s.bytes().count().into())
    }
}
