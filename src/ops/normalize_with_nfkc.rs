use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

use unicode_normalization;
use unicode_normalization::UnicodeNormalization;

pub struct NormalizeWithNFKC {}

impl NormalizeWithNFKC {
    fn function_for_chars(string: &str) -> String {
        string.chars().nfkc().to_string()
    }
}

impl traits::Op for NormalizeWithNFKC {
    fn name() -> &'static str { "normalize-with-nfkc" }
    fn usage() -> &'static str { "<#1 string to-normalize>" }
    // TODO add examples to description
    fn description() -> &'static str { "NFKC-normalize Unicode string #1 which applies compatibility decomposition followed by canonical composition (c.f. UAX #15)" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let text: &str = args.get(0)?.try_into()?;
        Ok(if unicode_normalization::is_nfkc(text) {
            0.207
        } else {
            0.669
        })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let text: &str = args.get(0)?.try_into()?;
        Ok(Self::function_for_chars(text).into())
    }
}
