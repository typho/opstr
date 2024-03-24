use crate::auxiliary;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct Similarity {}

impl traits::Op for Similarity {
    fn name() -> &'static str { "similarity" }
    fn usage() -> &'static str { "<#1 string base> <#2 string comparison>" }
    fn description() -> &'static str { "indicate similarity (0 = not, 100 = equal) of two strings with a number between 0 and 100" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args) -> Result<f32, LibError> {
        let s1 = args.get(0)?.try_into()?;
        let s2 = args.get(1)?.try_into()?;
        let sim = auxiliary::string_similarity(s1, s2);
        let similarity = f32::round(sim * 100.) as u8;

        Ok(if 40 <= similarity {
            0.632
        } else {
            0.327
        })
    }

    fn run(args: &Args) -> Result<Output, LibError> {
        let s1 = args.get(0)?.try_into()?;
        let s2 = args.get(1)?.try_into()?;
        let sim = auxiliary::string_similarity(s1, s2);
        let similarity = f32::round(sim * 100.) as i64;

        Ok(similarity.into())
    }
}
