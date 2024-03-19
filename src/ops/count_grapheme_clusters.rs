use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use crate::range;

use unicode_segmentation::UnicodeSegmentation;

pub struct CountGraphemeClusters {}

impl traits::Op for CountGraphemeClusters {
    fn name() -> &'static str { "count-grapheme-clusters" }
    fn usage() -> &'static str { "<#1 string to-analyze>" }
    fn description() -> &'static str { "return number of “Grapheme clusters” in string #1 according to Unicode Standard Annex 29 “Unicode Text Segmentation”" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let string: &str = args.get(0)?.try_into()?;
        Ok(if string.len() >= 20 { 0.512 } else { 0.08 })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let string: &str = args.get(0)?.try_into()?;
        let count = UnicodeSegmentation::graphemes(string, true).map(OutputValue::from_str).count();
        Ok(count.into())
    }
}
