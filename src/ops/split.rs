use crate::errors::Errors;
use crate::input::StrArgs;
use crate::ops::traits;
use crate::output::{Output, OutputValue};

pub struct Split {}

impl traits::OpMulti for Split {
    fn name() -> &'static str { "split" }
    fn description() -> &'static str { "split string #1 by any of the provided substrings #2, or #3, or …" }

    fn priority(args: &StrArgs) -> f32 {
        if args.len() < 2 {
            0.0
        } else if 2 <= args.len() && args.len() <= 5 {
            0.45
        } else {
            0.37
        }
    }

    fn run(args: &StrArgs) -> Result<Output, Errors> {
        if args.len() < 2 {
            return Err(Errors::ArgumentCountError((2..).into(), args.len()));
        }

        let string: &str = (&args[0]).into();
        let seps = args[1..].iter().map(|s| s.into()).collect::<Vec<&str>>();

        if seps.iter().all(|sep| sep.chars().count() == 1) {
            // all arguments are one code point? Common and simple to implement.
            let sep_chars: Vec<char> = seps.iter().map(|s| s.chars().next().unwrap()).collect();
            let slices = string.split(&sep_chars[..]);
            Ok(Output::HomogeneousList {
                data: slices.map(OutputValue::from_str).collect::<Vec<OutputValue>>(),
                notes: vec![],
            })

        } else {
            // Various arguments of different length? More complex implementation.
            let mut current_index = 0;
            let mut slices = vec![];

            while current_index < string.len() {
                let mut next_sep_indices = None;

                // apply str.find(substr) and determine the smallest non-None index → found_index
                for arg in args.iter().skip(1) {
                    let substring: &str = arg.into();
                    match string[current_index..].find(substring) {
                        Some(index) => {
                            let candidate_pre_index = current_index + index;
                            if let Some((pre_index, _)) = next_sep_indices {
                                if candidate_pre_index < pre_index {
                                    // better candidate? take it!
                                    next_sep_indices = Some((candidate_pre_index, candidate_pre_index + substring.len()));
                                }
                            } else {
                                // first candidate? take it!
                                next_sep_indices = Some((candidate_pre_index, candidate_pre_index + substring.len()));
                            }
                        },
                        None => continue,
                    }
                }

                if let Some((pre, post)) = next_sep_indices {
                    // Found the soonest separator? push it
                    slices.push(&string[current_index..pre]);
                    current_index = post;
                } else {
                    // No separator found? then we are done
                    break;
                }
            }

            // push final slice
            slices.push(&string[current_index..]);

            Ok(Output::HomogeneousList {
                data: slices.iter().map(|s| OutputValue::from_str(s)).collect::<Vec<OutputValue>>(),
                notes: vec![],
            })
        }
    }
}
