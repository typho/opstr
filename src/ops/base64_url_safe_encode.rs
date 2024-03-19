use base64;
use base64::{Engine as _, engine::general_purpose as base64_engine};

use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct Base64UrlSafeEncode {}

impl Base64UrlSafeEncode {
    fn function_for_chars(arg: &str) -> String {
        Self::function_for_bitstring(arg.as_bytes())
    }

    fn function_for_bitstring(arg: &[u8]) -> String {
        base64_engine::URL_SAFE_NO_PAD.encode(arg)
    }
}

impl traits::Op for Base64UrlSafeEncode {
    fn name() -> &'static str { "base64-url-safe-encode" }
    fn usage() -> &'static str { "<#1 string to-encode>" }
    fn description() -> &'static str { "base64 encoding of provided string #1 with URL-appropriate representation (c.f. RFC 3548)" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let base: &str = args.get(0)?.try_into()?;
        let length = base.chars().count();

        Ok(if 12 <= length && length <= 256 {
            0.6
        } else {
            0.24
        })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let base: &str = args.get(0)?.try_into()?;
        Ok(Self::function_for_chars(base).into())
    }
}
