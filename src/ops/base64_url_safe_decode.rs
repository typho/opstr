use base64::{Engine as _, engine::general_purpose as base64_engine};

use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::{Output, OutputValue};

pub struct Base64UrlSafeDecode {}

impl Base64UrlSafeDecode {
    fn function_for_str(arg: &str) -> Result<Vec<u8>, base64::DecodeError> {
        Self::function_for_bitstring(arg.as_bytes())
    }

    fn function_for_bitstring(arg: &[u8]) -> Result<Vec<u8>, base64::DecodeError> {
        base64_engine::URL_SAFE_NO_PAD.decode(arg)
    }
}

impl traits::OpOne for Base64UrlSafeDecode {
    fn name() -> &'static str { "base64-url-safe-decode" }
    fn description() -> &'static str { "base64 decoding of provided string #1 with URL-appropriate representation (c.f. RFC 3548)" }

    fn priority(arg: &StrArg) -> f32 {
        let s: &str = arg.into();
        let length = s.chars().count();

        if length < 6 {
            return 0.42;
        }

        match Self::function_for_str(arg.into()) {
            Ok(_) => 0.487,
            Err(_) => 0.0,
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        match Self::function_for_str(arg.into()) {
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
