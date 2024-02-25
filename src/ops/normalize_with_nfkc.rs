use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

use unicode_normalization;
use unicode_normalization::UnicodeNormalization;

pub struct NormalizeWithNFKC {}

impl NormalizeWithNFKC {
    fn function_for_str(string: &str) -> String {
        string.chars().nfkc().to_string()
    }
}

impl traits::OpOne for NormalizeWithNFKC {
    fn name() -> &'static str { "normalize-with-nfkc" }
    // TODO add examples to description
    fn description() -> &'static str { "NFKC-normalize Unicode string #1 which applies compatibility decomposition followed by canonical composition (c.f. UAX #15)" }

    fn priority(arg: &StrArg) -> f32 {
        if unicode_normalization::is_nfkc(arg.into()) {
            0.207
        } else {
            0.669
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let a: &str = arg.into();
        Ok(Self::function_for_str(a).into())
    }
}
