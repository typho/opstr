use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

use unicode_normalization;
use unicode_normalization::UnicodeNormalization;

pub struct NormalizeWithNFKD {}

impl NormalizeWithNFKD {
    fn function_for_str(string: &str) -> String {
        string.chars().nfkd().to_string()
    }
}

impl traits::OpOne for NormalizeWithNFKD {
    fn name() -> &'static str { "normalize-with-nfkd" }
    // TODO add examples to description
    fn description() -> &'static str { "NFKD-normalize Unicode string #1 which applies compatibility decomposition followed by canonical composition (c.f. UAX #15)" }

    fn priority(arg: &StrArg) -> f32 {
        if unicode_normalization::is_nfkd(arg.into()) {
            0.208
        } else {
            0.670
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let a: &str = arg.into();
        Ok(Self::function_for_str(a).into())
    }
}
