// TODO review naming

use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct LinesShortened {}

impl LinesShortened {
    fn center(string: &str, center: usize, length: usize) -> (bool, &str, bool) {
        if length >= string.len() {
            return (false, string, false);
        }
    
        let mut before = center as i64 - ((length + 1) / 2) as i64;
        let mut after = center as i64 + (length / 2) as i64;
        let mut precut = true;
        let mut postcut = true;
        
        if before < 0 {
            after += before.abs();
            before = 0;
            precut = false;
        }
        if after > string.len() as i64 {
            before -= (after - string.len() as i64).abs();
            after = string.len() as i64;
            postcut = false;
        }
    
        (precut, &string[(before as usize)..(after as usize)], postcut)
    }
    
    fn shorten_line(line: &str, max_length: usize) -> (bool, &str, bool) {
        // APPROACH: maybe the midpoint between the first and last ANSI sequence can be used as center?
        let first_ansiseq = line.find(|chr| chr == '\u{001B}');
        let last_ansiseq = line.rfind(|chr| chr == '\u{001B}');
    
        if let Some(first) = first_ansiseq {
            if let Some(last) = last_ansiseq {
                let distance = last - first;
                if distance <= max_length {
                    return Self::center(line, first + ((last - first) / 2), max_length);
                }
            }
        }
    
        // APPROACH: maybe the midpoint between the first and second ANSI sequence can be used as center?
        if let Some(first) = first_ansiseq {
            let second_ansiseq = line[first..].find(|chr| chr == '\u{001B}');
    
            if let Some(sec) = second_ansiseq {
                let second = first + sec;
                let distance = second - first;
                if distance <= max_length {
                    return Self::center(line, first + ((second - first) / 2), max_length);
                }
            }
        }
    
        // APPROACH: fallback: take middle of string
        let mid = line.len() / 2;
        let before = mid - (max_length + 1) / 2;
        let after = mid + max_length / 2;
    
        (true, &line[before..after], true)
    }
}

impl traits::Op for LinesShortened {
    fn name() -> &'static str { "lines-shortened" }
    fn usage() -> &'static str { "<#1 string text> <#2 int width>" }
    fn description() -> &'static str { "shorten lines in string #1, if necessary, not to exceed width #2" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let text: &str = args.get(0)?.try_into()?;
        let w: Result<i64, Errors> = args.get(1)?.try_into();

        Ok(match w {
            Ok(width) => {
                let mut all_exceed = true;
                let mut any_exceed = false;
                for line in text.lines() {
                    if line.len() >= width as usize {
                        any_exceed = true;
                    } else {
                        all_exceed = false;
                    }
                }

                if all_exceed {
                    0.87
                } else if any_exceed {
                    0.69
                } else {
                    0.498
                }
            },
            Err(_) => 0.0,
        })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let text: &str = args.get(0)?.try_into()?;
        let w: Result<i64, Errors> = args.get(1)?.try_into();

        match w {
            Ok(width) => {
                let mut result = String::new();
                for line in text.lines() {
                    if line.len() > width as usize {
                        let (precut, slice, postcut) = Self::shorten_line(&line, width as usize);
                        result.push_str(&format!("{}{}{}\n",
                            if precut { "[…] " } else { "" },
                            slice,
                            if postcut { " […]" } else { "" }
                        ));
                    } else {
                        result.push_str(line);
                        result.push('\n');
                    }
                    // TODO is it okay to replace the line terminator with \n?
                }
                Ok(result.into())
            },
            Err(_) => Err(Errors::ArgValueError(1, "".to_owned())),
        }
    }
}
