use crate::auxiliary;
use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use crate::range;

use std::cmp::Ordering;
use std::collections;

pub struct CodepointFrequency {}

impl traits::Op for CodepointFrequency {
    fn name() -> &'static str { "codepoint-frequency" }
    fn usage() -> &'static str { "<#1 string to-analyze-statistically>" }
    fn description() -> &'static str { "return the frequency analysis per codepoint of string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let string: &str = args.get(0)?.try_into()?;
        Ok(if 10 <= string.len() && string.len() <= 50 {
            0.67
        } else {
            0.51
        })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let string: &str = args.get(0)?.try_into()?;
        let codepoint_names = auxiliary::unicode_codepoint_names_lookup(&string.chars().collect::<Vec<char>>());
        let total_count = string.chars().count();
        let mut frequency: collections::HashMap<char, (usize, &'static str)> = collections::HashMap::new();

        for (chr, codepoint_name) in string.chars().zip(codepoint_names) {
            if frequency.contains_key(&chr) {
                let (chr_freq, name) = frequency[&chr];
                frequency.insert(chr, (chr_freq + 1, name));
            } else {
                frequency.insert(chr, match codepoint_name {
                    Some(name) => (1, name),
                    None => (1, auxiliary::UNICODEPOINT_UNKNOWN),
                });
            }
        }

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
