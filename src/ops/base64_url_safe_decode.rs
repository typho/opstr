use base64::{Engine as _, engine::general_purpose as base64_engine};

use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use crate::range;

pub struct Base64UrlSafeDecode {}

impl Base64UrlSafeDecode {
    fn function_for_chars(arg: &str) -> Result<Vec<u8>, base64::DecodeError> {
        Self::function_for_bitstring(arg.as_bytes())
    }

    fn function_for_bitstring(arg: &[u8]) -> Result<Vec<u8>, base64::DecodeError> {
        base64_engine::URL_SAFE_NO_PAD.decode(arg)
    }
}

impl traits::Op for Base64UrlSafeDecode {
    fn name() -> &'static str { "base64-url-safe-decode" }
    fn usage() -> &'static str { "<#1 string to-decode>" }
    fn description() -> &'static str { "base64 decoding of provided string #1 with URL-appropriate representation (c.f. RFC 3548)" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let base: &str = args.get(0)?.try_into()?;
        let length = base.chars().count();

        if length < 6 {
            return Ok(0.42);
        }

        Ok(match Self::function_for_chars(base) {
            Ok(_) => 0.487,
            Err(_) => 0.0,
        })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let base: &str = args.get(0)?.try_into()?;
        match Self::function_for_chars(base) {
            Ok(decoded) => {
                Ok(Output::HomogeneousList {
                    data: decoded.iter().map(|e| { OutputValue::Byte(*e) }).collect::<Vec<OutputValue>>(),
                    notes: vec![],
                })
            },
            Err(_) => {
                Err(Errors::ArgValueError(0, "provided argument is not a base64-encoded string".to_owned()))
            },
        }
    }
}
