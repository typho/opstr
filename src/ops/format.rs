use std::fmt;

use crate::errors::Errors;
use crate::input::StrArgs;
use crate::ops::traits;
use crate::output::Output;

use rt_format::ParsedFormat;
use rt_format::Specifier;
use rt_format::argument::FormatArgument;

pub struct Format {}

impl Format {
    fn function_for_str(args: &[&str]) -> Result<String, Errors> {
        match args.len() {
            0 => Err(Errors::ArgumentCountError((2..).into(), 0)),
            1 => Ok(args[0].into()),
            _ => {
                let template: &str = args[0];
                if !template.contains("{") {
                    return Ok(template.to_owned());
                }

                // prepare arguments
                let mut fmt_args = vec![];
                for arg in args.iter().skip(1) {
                    fmt_args.push(FmtStrArg(arg));
                }

                let repr = match ParsedFormat::parse(template, &fmt_args, &rt_format::NoNamedArguments) {
                    Ok(val) => format!("{}", val),
                    Err(err) => return Err(Errors::ArgValueError(0, err.to_string())),
                };

                Ok(repr)
            }
        }
    }
}

impl traits::OpMulti for Format {
    fn name() -> &'static str { "format" }
    fn description() -> &'static str { "replace {placeholders} in string #1 with consecutive arguments" }

    fn priority(args: &StrArgs) -> f32 {
        if args.is_empty() { return 0.0; }

        let template: &str = (&args[0]).into();
        let occurences_start = template.matches('{').count().max(5);
        let occurences_end = template.matches('}').count().max(5);

        if occurences_start < (args.len() - 1) {
            // NOTE: there are not sufficient placeholders for the arguments
            return 0.0;
        }

        let mut score = 0.75 + (0.05 * occurences_start as f32);
        if occurences_start != occurences_end {
            score *= 0.5;
        }
        score
    }

    fn run(args: &StrArgs) -> Result<Output, Errors> {
        if args.is_empty() {
            return Ok("".into());
        }

        let arguments = args.iter().map(|e| -> &str { e.into() }).collect::<Vec<&str>>();
        if args.len() == 1 {
            return Ok(arguments[0].into());
        }

        Ok(Self::function_for_str(&arguments)?.into())
    }
}

pub struct FmtStrArg<'s>(&'s str);

impl<'s> FormatArgument for FmtStrArg<'s> {
    // TODO I don't think specifier is respected in fmt_*

    fn supports_format(&self, _specifier: &Specifier) -> bool { true }

    fn fmt_display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }

    fn fmt_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }

    fn fmt_octal(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.parse::<u64>() {
            Ok(int) => write!(f, "{:o}", int),
            Err(_) => Err(std::fmt::Error),
        }
    }

    fn fmt_lower_hex(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.parse::<u64>() {
            Ok(int) => write!(f, "{:x}", int),
            Err(_) => Err(std::fmt::Error),
        }
    }

    fn fmt_upper_hex(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.parse::<u64>() {
            Ok(int) => write!(f, "{:X}", int),
            Err(_) => Err(std::fmt::Error),
        }
    }

    fn fmt_binary(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.parse::<u64>() {
            Ok(int) => write!(f, "{:b}", int),
            Err(_) => Err(std::fmt::Error),
        }
    }

    fn fmt_lower_exp(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.parse::<u64>() {
            Ok(int) => write!(f, "{:e}", int),
            Err(_) => Err(std::fmt::Error),
        }
    }

    fn fmt_upper_exp(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.parse::<u64>() {
            Ok(int) => write!(f, "{:E}", int),
            Err(_) => Err(std::fmt::Error),
        }
    }
}