use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct CountUtf8Bytes {}

impl traits::Op for CountUtf8Bytes {
    fn name() -> &'static str { "count-utf8-bytes" }
    fn usage() -> &'static str { "<#1 string to-analyze>" }
    fn description() -> &'static str { "encode string #1 in UTF-8 and return its number of bytes" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }
    fn priority(_args: &Args, _conf: &Configuration) -> Result<f32, LibError> { Ok(0.56) }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(s.bytes().count().into())
    }
}
