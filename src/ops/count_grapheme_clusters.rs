use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use unicode_segmentation::UnicodeSegmentation;

pub struct CountGraphemeClusters {}

impl traits::OpOne for CountGraphemeClusters {
    fn name() -> &'static str { "count-grapheme-clusters" }
    fn description() -> &'static str { "return number of “Grapheme clusters” according to Unicode Standard Annex #29 “Unicode Text Segmentation”" }

    fn priority(arg: &StrArg) -> f32 {
        let string: &str = arg.into();
        if string.len() >= 20 { 0.512 } else { 0.08 }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg.into();
        let count = UnicodeSegmentation::graphemes(string, true).map(OutputValue::from_str).count();
        Ok(count.into())
    }
}
