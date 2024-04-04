use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use crate::range;

use unicode_segmentation::UnicodeSegmentation;

pub struct SentenceClusters {}

impl traits::Op for SentenceClusters {
    fn name() -> &'static str { "sentence-clusters" }
    fn usage() -> &'static str { "<#1 string to-analyze>" }
    fn description() -> &'static str { "return “Sentence clusters” according to Unicode Standard Annex #29 “Unicode Text Segmentation”" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let string: &str = args.get(0)?.try_into()?;
        Ok(if string.len() >= 20 { 0.59 } else { 0.23 })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let string: &str = args.get(0)?.try_into()?;
        let data = string.split_word_bounds().map(OutputValue::from_str).collect::<Vec<OutputValue>>();
        Ok(Output::HomogeneousList { data, notes: vec![] })
    }
}
