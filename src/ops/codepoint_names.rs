use crate::auxiliary;
use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::{Output,OutputValue};

pub struct CodepointNames {}

impl traits::OpOne for CodepointNames {
    fn name() -> &'static str { "codepoint-names" }
    fn description() -> &'static str { "look up the Unicode name (or 'unknown-name' if unknown) of each codepoint of string #1, e.g. [“LATIN SMALL LETTER H”, “LATIN SMALL LETTER DOTLESS ”]" }
    fn priority(_arg: &StrArg) -> f32 { 0.39 }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg.into();
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