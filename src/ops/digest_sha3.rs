#[cfg(feature = "digest")]
use hex::ToHex;
#[cfg(feature = "digest")]
use sha3::{Digest, Sha3_256};

use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct DigestSha3256 {}

impl DigestSha3256 {
    fn function_for_str(arg: &str) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(arg.as_bytes());
        hasher.finalize().encode_hex()
    }

    fn function_for_bitstring(arg: &[u8]) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(arg);
        hasher.finalize().encode_hex()
    }
}

impl traits::OpOne for DigestSha3256 {
    fn name() -> &'static str { "digest-sha3-256" }
    fn description() -> &'static str { "generate the SHA3-256 hexadecimal digest of the given UTF-8 string" }

    fn priority(arg: &StrArg) -> f32 {
        let s: &str = arg.into();
        if s.len() <= 3 { 0.23 } else { 0.43 }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        Ok(Self::function_for_str(arg.into()).into())
    }
}
