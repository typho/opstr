use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

#[cfg(feature = "digest")]
use hex::ToHex;
#[cfg(feature = "digest")]
use sha1::{Digest, Sha1};


pub struct DigestSha1 {}

impl DigestSha1 {
    fn function_for_chars(arg: &str) -> String {
        let mut hasher = Sha1::new();
        hasher.update(arg.as_bytes());
        hasher.finalize().encode_hex()
    }

    #[doc(hidden)]
    #[allow(dead_code)]
    fn function_for_bytes(arg: &[u8]) -> String {
        let mut hasher = Sha1::new();
        hasher.update(arg);
        hasher.finalize().encode_hex()
    }
}

impl traits::Op for DigestSha1 {
    fn name() -> &'static str { "digest-sha1" }
    fn usage() -> &'static str { "<#1 string to-digest>" }
    fn description() -> &'static str { "generate the SHA1 hexadecimal digest of the given UTF-8 string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(if s.len() <= 3 { 0.21 } else { 0.41 })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let arg: &str = args.get(0)?.try_into()?;
        Ok(Self::function_for_chars(arg).into())
    }
}
