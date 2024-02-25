use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::{Output, OutputValue};

pub struct SubstringByteIndices {}

impl traits::OpTwo for SubstringByteIndices {
    fn name() -> &'static str { "substring-byte-indices" }
    fn description() -> &'static str { "return the byte indices where string #2 can be found in string #1" }
    fn priority(_arg1: &StrArg, _arg2: &StrArg) -> f32 { 0.3 }

    fn run(arg1: &StrArg, arg2: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg1.into();
        let sub: &str = arg2.into();

        let string_bytes = string.as_bytes();

        let mut data = vec![];
        let byte_offset = 0;
        
        loop {
            let base = byte_offset + sub.len();
            //let opt_idx = (&string_bytes[base..]).match_indices(sub.as_bytes());
            let opt_idx = string_bytes[base..].windows(sub.as_bytes().len()).position(|window| window == sub.as_bytes());
            if let Some(idx) = opt_idx {
                data.push(OutputValue::Int((base as i64) + (idx as i64)));
            } else {
                break;
            }
        }

        Ok(Output::HeterogeneousList { data, notes: vec![] })
    }
}
