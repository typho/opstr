use crate::auxiliary;
use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct Similarity {}

impl traits::OpTwo for Similarity {
    fn name() -> &'static str { "similarity" }
    fn description() -> &'static str { "indicate similarity (0 = not, 100 = equal) of two strings with a number between 0 and 100" }

    fn priority(arg1: &StrArg, arg2: &StrArg) -> f32 {
        let s1 = arg1.into();
        let s2 = arg2.into();
        let sim = auxiliary::string_similarity(s1, s2);
        let similarity = f32::round(sim * 100.) as u8;

        if 40 <= similarity {
            0.632
        } else {
            0.327
        }
    }

    fn run(arg1: &StrArg, arg2: &StrArg) -> Result<Output, Errors> {
        let s1 = arg1.into();
        let s2 = arg2.into();
        let sim = auxiliary::string_similarity(s1, s2);
        let similarity = f32::round(sim * 100.) as i64;

        Ok(similarity.into())
    }
}
