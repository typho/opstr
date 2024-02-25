use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

use unicode_normalization;
use unicode_normalization::UnicodeNormalization;

pub struct NormalizeWithNFD {}

impl NormalizeWithNFD {
    fn function_for_str(string: &str) -> String {
        string.chars().nfd().to_string()
    }
}

impl traits::OpOne for NormalizeWithNFD {
    fn name() -> &'static str { "normalize-with-nfd" }
    // TODO add examples to description
    fn description() -> &'static str { "NFD-normalize Unicode string #1 which applies canonical decomposition (c.f. UAX #15)" }

    fn priority(arg: &StrArg) -> f32 {
        if unicode_normalization::is_nfd(arg.into()) {
            0.21
        } else {
            0.672
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let a: &str = arg.into();
        Ok(Self::function_for_str(a).into())
    }
}
