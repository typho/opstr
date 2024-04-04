use crate::auxiliary;
use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output,OutputValue};
use crate::range;

pub struct CodepointNames {}

impl traits::Op for CodepointNames {
    fn name() -> &'static str { "codepoint-names" }
    fn usage() -> &'static str { "<#1 string to-decompose-and-represent>" }
    fn description() -> &'static str { "look up the Unicode name (or 'unknown-name' if unknown) of each codepoint of string #1, e.g. [“LATIN SMALL LETTER H”, “LATIN SMALL LETTER DOTLESS ”]" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }
    fn priority(_args: &Args, _conf: &Configuration) -> Result<f32, LibError> { Ok(0.39) }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let string: &str = args.get(0)?.try_into()?;
        let mut unknowns = 0;
        let mut data = vec![];

        for opt_name in auxiliary::unicode_codepoint_names_lookup(&string.chars().collect::<Vec<char>>()) {
            data.push(OutputValue::SingleLineText(match opt_name {
                Some(name) => name,
                None => {
                    unknowns += 1;
                    "unknown-name"
                },
            }.to_owned()));
        }

        Ok(Output::HomogeneousList {
            data,
            notes: if unknowns > 0 {
                vec![ format!("{} codepoints have not been found in UnicodeData.txt of Version 15.0", unknowns) ]
            } else {
                vec![]
            }
        })
    }
}