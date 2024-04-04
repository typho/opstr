use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

use base64;
use base64::{Engine as _, engine::general_purpose as base64_engine};

pub struct Base64Encode {}

impl Base64Encode {
    fn function_for_chars(arg: &str) -> String {
        Self::function_for_bitstring(arg.as_bytes())
    }

    fn function_for_bitstring(arg: &[u8]) -> String {
        base64_engine::STANDARD_NO_PAD.encode(arg)
    }
}

impl traits::Op for Base64Encode {
    fn name() -> &'static str { "base64-encode" }
    fn usage() -> &'static str { "<#1 string to-encode>" }
    fn description() -> &'static str { "base64 encoding of provided string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let base: &str = args.get(0)?.try_into()?;
        let length = base.chars().count();

        Ok(if 12 <= length && length <= 256 {
            0.63
        } else {
            0.242
        })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let base: &str = args.get(0)?.try_into()?;
        Ok(Self::function_for_chars(base).into())
    }
}
