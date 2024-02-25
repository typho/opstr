use base64;
use base64::{Engine as _, engine::general_purpose as base64_engine};

use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct Base64UrlSafeEncode {}

impl Base64UrlSafeEncode {
    fn function_for_str(arg: &str) -> String {
        Self::function_for_bitstring(arg.as_bytes())
    }

    fn function_for_bitstring(arg: &[u8]) -> String {
        base64_engine::URL_SAFE_NO_PAD.encode(arg)
    }
}

impl traits::OpOne for Base64UrlSafeEncode {
    fn name() -> &'static str { "base64-url-safe-encode" }
    fn description() -> &'static str { "base64 encoding of provided string #1 with URL-appropriate representation (c.f. RFC 3548)" }

    fn priority(arg: &StrArg) -> f32 {
        let s: &str = arg.into();
        let length = s.chars().count();

        if 12 <= length && length <= 256 {
            0.6
        } else {
            0.24
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        Ok(Self::function_for_str(arg.into()).into())
    }
}
