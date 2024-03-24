#[cfg(feature = "digest")]
use hex::ToHex;
#[cfg(feature = "digest")]
use md5;

use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct DigestMd5 {}

impl DigestMd5 {
    fn function_for_chars(arg: &str) -> String {
        md5::compute(arg.as_bytes()).encode_hex()
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    fn function_for_bytes(arg: &[u8]) -> String {
        md5::compute(arg).encode_hex()
    }
}

impl traits::Op for DigestMd5 {
    fn name() -> &'static str { "digest-md5" }
    fn usage() -> &'static str { "<#1 string to-digest>" }
    fn description() -> &'static str { "generate the MD5 hexadecimal digest of the given UTF-8 string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(if s.len() <= 3 { 0.22 } else { 0.42 })
    }

    fn run(args: &Args) -> Result<Output, LibError> {
        let arg: &str = args.get(0)?.try_into()?;
        Ok(Self::function_for_chars(arg).into())
    }
}
