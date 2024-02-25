#[cfg(feature = "digest")]
use hex::ToHex;
#[cfg(feature = "digest")]
use md5;

use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct DigestMd5 {}

impl DigestMd5 {
    fn function_for_str(arg: &str) -> String {
        md5::compute(arg.as_bytes()).encode_hex()
    }

    fn function_for_bitstring(arg: &[u8]) -> String {
        md5::compute(arg).encode_hex()
    }
}

impl traits::OpOne for DigestMd5 {
    fn name() -> &'static str { "digest-md5" }
    fn description() -> &'static str { "generate the MD5 hexadecimal digest of the given UTF-8 string" }

    fn priority(arg: &StrArg) -> f32 {
        let s: &str = arg.into();
        if s.len() <= 3 { 0.22 } else { 0.42 }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        Ok(Self::function_for_str(arg.into()).into())
    }
}
