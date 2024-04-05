use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use crate::range;

pub struct SortLexicographically {}

impl traits::Op for SortLexicographically {
    fn name() -> &'static str { "sort-lexicographically" }
    fn usage() -> &'static str { "[<#1 string to-sort> one or more times]" }
    fn description() -> &'static str { "sort the strings provided lexicographically by their Unicode codepoints" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexOpen(1) }
    fn priority(_args: &Args, _conf: &Configuration) -> Result<f32, LibError> { Ok(0.5) }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        // fetch arguments as strings
        let mut strings: Vec<String> = vec![];
        for arg in args.iter() {
            strings.push(arg.str_or_panic().to_string());
        }

        // Sort strings lexicographically.
        // Sorting locale-dependent “is outside the scope of the `str` type”
        // https://doc.rust-lang.org/std/primitive.str.html#impl-Ord-for-str
        strings.sort();

        Ok(Output::HomogeneousList {
            data: strings.iter().map(|s| OutputValue::from_str(s)).collect::<Vec<OutputValue>>(),
            notes: vec![],
        })
    }
}
