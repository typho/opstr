use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct Dedent {}

impl Dedent {
    fn identify_common_prefix_on_nonempty_lines(string: &str) -> &str {
        let mut result = "";
        for line in string.lines() {
            if line.is_empty() { continue; }
            if result.is_empty() {
                result = line;
            }
            
            let mut common_byte_length = 0;
            for i in (0..usize::min(line.len(), result.len())).rev() {
                if result.is_char_boundary(i) {
                    if line.get(0..i) == result.get(0..i) {
                        common_byte_length = i;
                    }
                }
            }

            result = match result.get(0..common_byte_length) {
                Some(sub) => sub,
                None => panic!("error in dedent: implementation error detecting code point boundaries properly"),
            };
        }
        result
    }
}

impl traits::OpOne for Dedent {
    fn name() -> &'static str { "dedent" }
    fn description() -> &'static str { "identify and remove common indentation among all non-empty lines" }

    fn priority(arg: &StrArg) -> f32 {
        let string: &str = arg.into();
        let common_prefix = Self::identify_common_prefix_on_nonempty_lines(string);

        if common_prefix.is_empty() {
            0.0
        } else if string.lines().count() > 4 {
            0.72
        } else {
            0.3
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg.into();
        let common_prefix = Self::identify_common_prefix_on_nonempty_lines(string);

        let terminator = if string.contains("\r\n") { "\r\n" } else { "\n" };
        let lines_count = string.lines().count();

        let mut out = String::new();
        for (i, line) in string.lines().enumerate() {
            if line.starts_with(common_prefix) {
                match line.get(common_prefix.len()..) {
                    Some(suffix) => out.push_str(suffix),
                    None => panic!("error in dedent: removing a UTF-8 suffix from a UTF-8 string must always be possible"),
                };
            }
            if i != lines_count - 1 {
                out.push_str(terminator);
            }
        }

        Ok(out.into())
    }
}