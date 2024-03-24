use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use crate::range;

pub struct SubstringByteIndices {}

impl traits::Op for SubstringByteIndices {
    fn name() -> &'static str { "substring-byte-indices" }
    fn usage() -> &'static str { "<#1 string base> <#2 string search>" }
    fn description() -> &'static str { "return the byte indices where string #2 can be found in string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args) -> Result<f32, LibError> {
        let base: &str = args.get(0)?.try_into()?;
        let search: &str = args.get(1)?.try_into()?;

        Ok(if base.matches(search).count() > 0 {
            0.3
        } else {
            0.0
        })
    }

    fn run(args: &Args) -> Result<Output, LibError> {
        let base: &str = args.get(0)?.try_into()?;
        let search: &str = args.get(1)?.try_into()?;

        let string_bytes = base.as_bytes();

        let mut data = vec![];
        let byte_offset = 0;
        
        loop {
            let base = byte_offset + search.len();
            //let opt_idx = (&string_bytes[base..]).match_indices(search.as_bytes());
            let opt_idx = string_bytes[base..].windows(search.as_bytes().len()).position(|window| window == search.as_bytes());
            if let Some(idx) = opt_idx {
                data.push(OutputValue::Int((base as i64) + (idx as i64)));
            } else {
                break;
            }
        }

        Ok(Output::HeterogeneousList { data, notes: vec![] })
    }
}
