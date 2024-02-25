use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

use std::cmp;

pub struct LevenstheinDistance {}

// via https://en.wikibooks.org/wiki/Algorithm_Implementation/Strings/Levenshtein_distance#Rust
// CC BY-SA: https://creativecommons.org/licenses/by-sa/4.0/
fn levenshtein_d1stance<T>(s1: &T, s2: &T) -> usize where T: ToString {
    let v1: Vec<char> = s1.to_string().chars().collect();
    let v2: Vec<char> = s2.to_string().chars().collect();
    let v1len = v1.len();
    let v2len = v2.len();
    
    // Early exit if one of the strings is empty
    if v1len == 0 { return v2len; }
    if v2len == 0 { return v1len; }

    fn min3<T: Ord>(v1: T, v2: T, v3: T) -> T{
        cmp::min(v1, cmp::min(v2, v3))
    }
    fn delta(x: char, y: char) -> usize {
        if x == y { 0 } else { 1 }
    }
    
    let mut column: Vec<usize> = (0..v1len+1).collect();
    for x in 1..v2len+1 {
        column[0] = x;
        let mut lastdiag = x-1;
        for y in 1..v1len+1 {
            let olddiag = column[y];
            column[y] = min3(column[y] + 1,
                             column[y-1] + 1,
                             lastdiag + delta(v1[y-1], v2[x-1]));
            lastdiag = olddiag;
        }
    }
    column[v1len]
}

impl traits::OpTwo for LevenstheinDistance {
    fn name() -> &'static str { "levensthein-distance" }
    fn description() -> &'static str { "levensthein distance between #1 and #2" }

    fn priority(arg1: &StrArg, arg2: &StrArg) -> f32 {
        0.473
    }

    fn run(arg1: &StrArg, arg2: &StrArg) -> Result<Output, Errors> {
        let s1: &str = arg1.into();
        let s2: &str = arg2.into();

        let dist: usize = levenshtein_d1stance(&s1, &s2);
        Ok((dist as i64).into())
    }
}
