use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use unicode_segmentation::UnicodeSegmentation;

pub struct SentenceClusters {}

impl traits::OpOne for SentenceClusters {
    fn name() -> &'static str { "sentence-clusters" }
    fn description() -> &'static str { "return “Sentence clusters” according to Unicode Standard Annex #29 “Unicode Text Segmentation”" }

    fn priority(arg: &StrArg) -> f32 {
        let string: &str = arg.into();
        if string.len() >= 20 { 0.59 } else { 0.23 }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg.into();
        let data = string.split_word_bounds().map(OutputValue::from_str).collect::<Vec<OutputValue>>();
        Ok(Output::HomogeneousList { data, notes: vec![] })
    }
}
