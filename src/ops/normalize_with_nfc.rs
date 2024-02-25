use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

use unicode_normalization;
use unicode_normalization::UnicodeNormalization;

pub struct NormalizeWithNFC {}

impl NormalizeWithNFC {
    fn function_for_str(string: &str) -> String {
        string.chars().nfd().to_string()
    }
}

impl traits::OpOne for NormalizeWithNFC {
    fn name() -> &'static str { "normalize-with-nfc" }
    // TODO add examples to description
    fn description() -> &'static str { "NFC-normalize Unicode string #1 which applies canonical decomposition followed by canonical composition (c.f. UAX #15)" }

    fn priority(arg: &StrArg) -> f32 {
        if unicode_normalization::is_nfc(arg.into()) {
            0.208
        } else {
            0.671
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let a: &str = arg.into();
        Ok(Self::function_for_str(a).into())
    }
}
