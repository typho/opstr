use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct SkipPrefix {}

impl traits::OpTwo for SkipPrefix {
    fn name() -> &'static str { "skip-prefix" }
    fn description() -> &'static str { "remove string #2 from the beginning of string #1 if it exists" }

    fn priority(arg1: &StrArg, arg2: &StrArg) -> f32 {
        let s1: &str = arg1.into();
        let s2: &str = arg2.into();
        if s1.starts_with(s2) {
            0.637
        } else {
            0.547
        }
    }

    fn run(arg1: &StrArg, arg2: &StrArg) -> Result<Output, Errors> {
        let s1: &str = arg1.into();
        let s2: &str = arg2.into();
        if s1.starts_with(s2) {
            match s1.get(s2.len()..) {
                Some(sub) => Ok(sub.into()),
                None => Err(Errors::ArgTypeError(0, format!("Removing the UTF-8 prefix {:?} from a UTF-8 string {:?} should always result in a UTF-8 string - but it did not!", s2, s1))),
            }
        } else {
            Ok(s1.into())
        }
    }
}
