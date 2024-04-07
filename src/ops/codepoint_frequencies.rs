use crate::auxiliary;
use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use crate::range;

use std::cmp::Ordering;

pub struct CodepointFrequencies {}

impl traits::Op for CodepointFrequencies {
    fn name() -> &'static str { "codepoint-frequencies" }
    fn usage() -> &'static str { "<#1 string to-analyze-statistically>" }
    fn description() -> &'static str { "return the frequency analysis per codepoint of string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let string: &str = args.get(0)?.try_into()?;
        Ok(if 10 <= string.len() && string.len() <= 50 {
            0.67
        } else {
            0.51
        })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let string: &str = args.get(0)?.try_into()?;
        let codepoint_names = auxiliary::unicode_codepoint_names_lookup(&string.chars().collect::<Vec<char>>());
        let total_count = string.chars().count();
        let mut frequency: Vec<(char, (usize, &'static str))> = vec![];

        for (chr, codepoint_name) in string.chars().zip(codepoint_names) {
            // determine index within frequency
            let mut opt_index = None;
            for (i, e) in frequency.iter().enumerate() {
                if e.0 == chr {
                    opt_index = Some(i);
                }
            }

            if let Some(idx) = opt_index {
                let (_, (chr_freq, name)) = frequency.swap_remove(idx);
                frequency.push((chr, (chr_freq + 1, name)));
            } else {
                frequency.push((chr, match codepoint_name {
                    Some(name) => (1, name),
                    None => (1, auxiliary::UNICODEPOINT_UNKNOWN),
                }));
            }
        }

        frequency.sort_by_key(|e| e.0);
        frequency.sort_by_key(|e| e.1.0);

        let mut data: Vec<Vec<OutputValue>> = vec![];
        for (chr, (count, codepoint_name)) in frequency.iter() {
            data.push(vec![
                OutputValue::Int(*count as i64),
                OutputValue::Int(((100 * *count) / total_count) as i64),
                OutputValue::SingleLineText(chr.to_string()),
                OutputValue::SingleLineText((*codepoint_name).to_owned()),
            ]);
        }

        let cmp = |row1: &Vec<OutputValue>, row2: &Vec<OutputValue>| {
            if let OutputValue::Int(a) = row1[1] {
                if let OutputValue::Int(b) = row2[1] {
                    return a.cmp(&b).reverse();
                }
            }
            Ordering::Equal
        };

        data.sort_by(cmp);

        Ok(Output::Table{
            data,
            column_headers: vec!["frequency".to_owned(), "percentage".to_owned(), "codepoint".to_owned(), "codepoint-name".to_owned()],
            notes: vec![]
        })
    }
}
