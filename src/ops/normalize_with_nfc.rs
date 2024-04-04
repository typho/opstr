use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

use unicode_normalization;
use unicode_normalization::UnicodeNormalization;

pub struct NormalizeWithNFC {}

impl NormalizeWithNFC {
    fn function_for_chars(string: &str) -> String {
        string.chars().nfd().to_string()
    }
}

impl traits::Op for NormalizeWithNFC {
    fn name() -> &'static str { "normalize-with-nfc" }
    fn usage() -> &'static str { "<#1 string to-normalize>" }
    // TODO add examples to description
    fn description() -> &'static str { "NFC-normalize Unicode string #1 which applies canonical decomposition followed by canonical composition (c.f. UAX #15)" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let text: &str = args.get(0)?.try_into()?;
        Ok(if unicode_normalization::is_nfc(text) {
            0.208
        } else {
            0.671
        })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let text: &str = args.get(0)?.try_into()?;
        Ok(Self::function_for_chars(text).into())
    }
}
