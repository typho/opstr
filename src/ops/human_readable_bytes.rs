use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct HumanReadableBytes {}

impl HumanReadableBytes {
    const SUFFIXES: [&'static str; 9] = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

    fn function_for_i64(bytes_count: i64) -> String {
        if bytes_count == 0 {
            return "0 B".to_owned();
        }
        let sign = if bytes_count < 0 { "-" } else { "" };
        let delimiter = 1024_f64;
        let bytes_count_magnitude = ((bytes_count as f64).ln() / delimiter.ln()).floor() as i32;
        let exponent = i32::min(bytes_count_magnitude, (Self::SUFFIXES.len() - 1) as i32);
        let exponent_unit = Self::SUFFIXES[exponent as usize];
        let bytes_repr = format!("{:.2}", (bytes_count as f64) / delimiter.powi(exponent)).parse::<f64>().unwrap() * 1_f64;
        format!("{}{} {}", sign, bytes_repr, exponent_unit)
    }
}

impl traits::Op for HumanReadableBytes {
    fn name() -> &'static str { "human-readable-bytes" }
    fn usage() -> &'static str { "<#1 string bytes-count>" }
    fn description() -> &'static str { "represent integer #1 (as 1024-based count of bytes) in a human-readable manner likely with two decimal points" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, LibError> {
        let res: Result<i64, LibError> = args.get(0)?.try_into();
        Ok(match res {
            Ok(bytes_count) => {
                if 1024 <= bytes_count && bytes_count <= i32::MAX as i64 {
                    0.875
                } else {
                    0.223
                }
            },
            Err(_) => 0.0
        })
    }

    fn run(args: &Args) -> Result<Output, LibError> {
        let bytes_count: i64 = args.get(0)?.try_into()?;
        Ok(Self::function_for_i64(bytes_count).into())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_human_readable_bytes() {
        assert_eq!(HumanReadableBytes::function_for_i64(21387), "20.89 kB");
        assert_eq!(HumanReadableBytes::function_for_i64(10240), "10 kB");
    }
}