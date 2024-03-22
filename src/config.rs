use crate::errors::Errors;

use std::default;
use std::env;
use std::fmt;
use std::io;

use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

/// Global application settings (mainly to configure representation)
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Configuration {
    /// Radix used to represent bytes and integers in the output.
    /// Only radices 2, 10, and 16 are supported. Default: 10.
    pub radix: usize,
    /// Shall we use an uppercase letter (e.g. ``0xAB3D``) to print a hexadecimal number
    /// (or more generically any number with alphabetic characters)?
    /// If false, lowercase letters are used. Default: true.
    pub alpha_upper: bool,
    /// The color scheme to be used to print the output.
    /// Desired support: NONE and several color schemes for various colot blindnesses.
    /// Default: NONE.
    pub color_scheme: ColorScheme,
    /// If the result is a list, this option allows to represent only the zero-based {item}-th element.
    /// If the result is a table, this option allows to represent only the zero-based {item}-th row.
    pub item: Option<isize>,
    /// If the result is an association, this option allows to represent only the element with key {column}.
    /// If the result is a table, this option allows to represent only the {column}-th column.
    pub column: Option<String>,
    /// The locale to be used for locale-dependent operations.
    /// Default: 'en-US'.
    pub locale: String,
    /// We can make opstr represent results for a specific syntax
    /// (mainly programming languages, like 'Go' or 'python'),
    /// but the default is a human-readable version.
    /// Default: ``Syntax::Human``.
    pub output_syntax: Syntax,
}

impl default::Default for Configuration {
    fn default() -> Self {
        Self {
            radix: 10,
            alpha_upper: true,
            color_scheme: ColorScheme::default(),
            item: None,
            column: None,
            locale: String::from("en-US"),
            output_syntax: Syntax::Human,
        }
    }
}

impl Configuration {
    pub fn overwrite_with_clap(&mut self, out_radix: Option<u8>, out_item: Option<isize>, out_column: Option<String>, out_alpha_upper: Option<bool>, out_color_scheme: Option<String>, out_locale: Option<String>, out_syntax: Option<String>) -> Result<(), Errors> {
        if let Some(radix) = out_radix {
            if !(radix == 2 || radix == 10 || radix == 16) {
                return Err(Errors::CLIValueError("radix", "Only radices 2, 10, and 16 are supported".to_string()));
            }
            self.radix = out_radix.unwrap_or(10) as usize;
        }

        self.item = out_item;
        self.column = out_column.map(|s| s.to_owned());

        // ignore "" as selection column name
        if let Some(column_name) = &self.column {
            if column_name.is_empty() {
                self.column = None;
            }
        }

        if let Some(alpha_upper) = out_alpha_upper {
            self.alpha_upper = alpha_upper;
        }

        if let Some(color_scheme) = out_color_scheme {
            match ColorScheme::by_name(&color_scheme) {
                Some(cs) => self.color_scheme = cs,
                None => return Err(Errors::CLIValueError("color-scheme", "Unknown color scheme".to_string())),
            }
        }

        if let Some(locale) = out_locale {
            // NOTE: there is no trivial locale verification algorithm
            // I expect some failure during the operation later, if the locale is invalid, but must be provided.
            self.locale = locale;
        }

        if let Some(syntax) = out_syntax {
            self.output_syntax = match syntax.to_ascii_lowercase().as_str() {
                "c" => Syntax::C,
                "c++" | "cpp" => Syntax::Cpp,
                "golang" | "go" => Syntax::Golang,
                "human" | "default" => Syntax::Human,
                "java" => Syntax::Java,
                "kotlin" => Syntax::Kotlin,
                "python" | "py" => Syntax::Python,
                "rust" | "rustlang" => Syntax::Rust,
                _ => return Err(Errors::CLIValueError("syntax", "Sorry, no support yet".to_string())),
            };
        }

        Ok(())
    }

    pub fn overwrite_with_env(&mut self) -> Result<(), Errors> {
        if let Ok(val) = env::var("OPSTR_RADIX") {
            match val.parse::<u8>() {
                Ok(r) => {
                    self.radix = r as usize;
                    if !(r == 2 || r == 10 || r == 16) {
                        return Err(Errors::CLIValueError("radix", "Only radices 2, 10, and 16 are supported".to_string()));
                    }            
                },
                Err(_) => return Err(Errors::CLIValueError("OPSTR_RADIX", "env value is not an integer".to_string())),
            }
        }

        if let Ok(val) = env::var("OPSTR_ALPHA_UPPER") {
            self.alpha_upper = match val.to_lowercase().as_str() {
                "yes" | "1" | "y" | "true" => true,
                _ => false,
            };
        }

        if let Ok(val) = env::var("OPSTR_COLOR_SCHEME") {
            match ColorScheme::by_name(&val) {
                Some(cs) => self.color_scheme = cs,
                None => return Err(Errors::CLIValueError("color-scheme", "Unknown color scheme".to_string())),
            }
        }

        if let Ok(val) = env::var("OPSTR_LOCALE") {
            // NOTE: there is no trivial locale verification algorithm
            // I expect some failure during the operation later, if the locale is invalid, but must be provided.
            self.locale = val;
        }

        if let Ok(val) = env::var("OPSTR_SYNTAX") {
            self.output_syntax = match val.to_ascii_lowercase().as_str() {
                "c" => Syntax::C,
                "c++" | "cpp" => Syntax::Cpp,
                "golang" | "go" => Syntax::Golang,
                "human" => Syntax::Human,
                "java" => Syntax::Java,
                "kotlin" => Syntax::Kotlin,
                "python" | "py" => Syntax::Python,
                "rust" | "rustlang" => Syntax::Rust,
                _ => return Err(Errors::CLIValueError("syntax", "Sorry, no support yet".to_string())),
            };
        }

        Ok(())
    }
}

/// Enum `ColorScheme` in use to represent the diagnostic message.
/// 
/// The default one might be a good choice, but for visually impaired
/// people, alternatives must be provided. Furthermore personal preferences
/// shall be addressed.
/// 
/// For the library, it shall be pointed out that ColorScheme::NONE
/// might be handled by different program logic. This is because
/// we use method ``uses_colors`` to determine whether colored output
/// is actually used. And we invoke different methods depending on the answer.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default, Hash, PartialEq)]
pub enum ColorScheme {
    NoColors,
    #[default]
    Default,
    RegularAndBold,
    Red,
    Green,
    Blue,
    White,
    // TODO more schemes
}

impl Eq for ColorScheme {}

impl ColorScheme {
    /// Take a name and return the corresponding ColorScheme instance (or None, if unknown)
    pub fn by_name(scheme: &str) -> Option<Self> {
        use ColorScheme::*;
        match scheme.to_ascii_lowercase().as_str() {
            "none" => Some(NoColors),
            "default" => Some(Default),
            "regularandbold" => Some(RegularAndBold),
            "red" => Some(Red),
            "green" => Some(Green),
            "blue" => Some(Blue),
            "white" => Some(White),
            _ => None,
        }
    }

    /// Return the ColorChoice for the configured color scheme
    pub fn color_choice(&self) -> ColorChoice {
        match self {
            ColorScheme::NoColors | ColorScheme::RegularAndBold => ColorChoice::Never,
            _ => ColorChoice::AlwaysAnsi,
        }
    }

    /// represents an operation like ``----- hello-world ---------``
    pub fn op_section(&self, op_name: &str) -> io::Result<()> {
        if self == &ColorScheme::NoColors {
            println!("----- {} {}", op_name, "-".repeat((65 - op_name.len()).max(0)));
            return Ok(());
        }

        let mut stdout = StandardStream::stdout(self.color_choice());
        match self {
            ColorScheme::NoColors => {},
            ColorScheme::Default => {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
                write!(&mut stdout, "----- ")?;
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
                write!(&mut stdout, "{}", op_name)?;
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
                writeln!(&mut stdout, " {}", "-".repeat((65 - op_name.len()).max(0)))?;
            },
            ColorScheme::RegularAndBold => {
                write!(&mut stdout, "----- ")?;
                stdout.set_color(ColorSpec::new().set_bold(true))?;
                write!(&mut stdout, "{}", op_name)?;
                stdout.reset()?;
                writeln!(&mut stdout, " {}", "-".repeat((65 - op_name.len()).max(0)))?;
            },
            ColorScheme::Red => {
                write!(&mut stdout, "----- ")?;
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(1))))?;
                write!(&mut stdout, "{}", op_name)?;
                stdout.reset()?;
                writeln!(&mut stdout, " {}", "-".repeat((65 - op_name.len()).max(0)))?;
            },
            ColorScheme::Green => {
                write!(&mut stdout, "----- ")?;
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(34))))?;
                write!(&mut stdout, "{}", op_name)?;
                stdout.reset()?;
                writeln!(&mut stdout, " {}", "-".repeat((65 - op_name.len()).max(0)))?;
            },
            ColorScheme::Blue => {
                write!(&mut stdout, "----- ")?;
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(61))))?;
                write!(&mut stdout, "{}", op_name)?;
                stdout.reset()?;
                writeln!(&mut stdout, " {}", "-".repeat((65 - op_name.len()).max(0)))?;
            },
            ColorScheme::White => {
                write!(&mut stdout, "----- ")?;
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Ansi256(236))))?;
                write!(&mut stdout, "{}", op_name)?;
                stdout.reset()?;
                writeln!(&mut stdout, " {}", "-".repeat((65 - op_name.len()).max(0)))?;
            },
        };
        stdout.reset()?;
        Ok(())
    }

    /// represents a note label like ``NOTE: `` in ``NOTE: recognize this Unicode codepoint``
    pub fn note_label(&self, label: &str) -> io::Result<()> {
        if self == &ColorScheme::NoColors {
            print!("{}", label);
            return Ok(());
        }

        let mut stderr = StandardStream::stderr(self.color_choice());
        let mut cs = ColorSpec::new();
        stderr.set_color(match self {
            ColorScheme::NoColors => &cs,
            ColorScheme::Default => cs.set_fg(Some(Color::Magenta)),
            ColorScheme::RegularAndBold => cs.set_bold(true),
            ColorScheme::Red => cs.set_fg(Some(Color::Ansi256(218))),
            ColorScheme::Green => cs.set_fg(Some(Color::Ansi256(70))),
            ColorScheme::Blue => cs.set_fg(Some(Color::Ansi256(26))),
            ColorScheme::White => cs.set_fg(Some(Color::Ansi256(243))),
        })?;
        write!(&mut stderr, "{}", label)?;
        stderr.reset()?;
        Ok(())
    }

    /// represents a error label like ``ERROR: `` in ``ERROR: argument invalid``
    pub fn error_label(&self, label: &str) -> io::Result<()> {
        if self == &ColorScheme::NoColors {
            eprint!("{}", label);
            return Ok(());
        }

        let mut stderr = StandardStream::stderr(self.color_choice());
        let mut cs = ColorSpec::new();
        stderr.set_color(match self {
            ColorScheme::NoColors => &cs,
            ColorScheme::Default => cs.set_fg(Some(Color::Red)),
            ColorScheme::RegularAndBold => cs.set_bold(true),
            ColorScheme::Red => cs.set_fg(Some(Color::Ansi256(9))),
            ColorScheme::Green => cs.set_fg(Some(Color::Ansi256(46))),
            ColorScheme::Blue => cs.set_fg(Some(Color::Ansi256(27))),
            ColorScheme::White => cs.set_fg(Some(Color::Ansi256(232))),
        })?;
        write!(&mut stderr, "{}", label)?;
        stderr.flush()?;
        stderr.reset()?;
        Ok(())
    }

    /// represents a keyword like a column header or type name
    pub fn keyword(&self, word: &str) -> io::Result<()> {
        if self == &ColorScheme::NoColors {
            print!("{}", word);
            return Ok(());
        }

        let mut stdout = StandardStream::stdout(self.color_choice());
        let mut cs = ColorSpec::new();
        stdout.set_color(match self {
            ColorScheme::NoColors => &cs,
            ColorScheme::Default => cs.set_fg(Some(Color::White)),
            ColorScheme::RegularAndBold => cs.set_bold(true),
            ColorScheme::Red => cs.set_fg(Some(Color::Ansi256(88))),
            ColorScheme::Green => cs.set_fg(Some(Color::Ansi256(36))),
            ColorScheme::Blue => cs.set_fg(Some(Color::Ansi256(147))),
            ColorScheme::White => cs.set_fg(Some(Color::Ansi256(240))),
        })?;
        write!(&mut stdout, "{}", word)?;
        stdout.reset()?;
        Ok(())
    }

    pub fn outer_wrapper(&self, wrapper: &str) -> io::Result<()> {
        if self == &ColorScheme::NoColors {
            print!("{}", wrapper);
            return Ok(());
        }

        let mut stdout = StandardStream::stdout(self.color_choice());
        let mut cs = ColorSpec::new();
        stdout.set_color(match self {
            ColorScheme::NoColors => &cs,
            ColorScheme::Default => cs.set_bold(true),
            ColorScheme::RegularAndBold => cs.set_bold(true),
            ColorScheme::Red => cs.set_fg(Some(Color::Ansi256(216))),
            ColorScheme::Green => cs.set_fg(Some(Color::Ansi256(190))),
            ColorScheme::Blue => cs.set_fg(Some(Color::Ansi256(45))),
            ColorScheme::White => cs.set_fg(Some(Color::Ansi256(246))),
        })?;
        write!(&mut stdout, "{}", wrapper)?;
        stdout.reset()?;
        Ok(())
    }

    pub fn outer_separator(&self, sep: &str) -> io::Result<()> {
        if self == &ColorScheme::NoColors {
            print!("{}", sep);
            return Ok(());
        }

        let mut stdout = StandardStream::stdout(self.color_choice());
        let mut cs = ColorSpec::new();
        stdout.set_color(match self {
            ColorScheme::NoColors => &cs,
            ColorScheme::Default => cs.set_bold(true),
            ColorScheme::RegularAndBold => cs.set_bold(true),
            ColorScheme::Red => cs.set_fg(Some(Color::Ansi256(209))),
            ColorScheme::Green => cs.set_fg(Some(Color::Ansi256(192))),
            ColorScheme::Blue => cs.set_fg(Some(Color::Ansi256(117))),
            ColorScheme::White => cs.set_fg(Some(Color::Ansi256(243))),
        })?;
        write!(&mut stdout, "{}", sep)?;
        stdout.reset()?;
        Ok(())
    }


    pub fn inner_wrapper(&self, wrapper: &str) -> io::Result<()> {
        if self == &ColorScheme::NoColors {
            print!("{}", wrapper);
            return Ok(());
        }

        let mut stdout = StandardStream::stdout(self.color_choice());
        let mut cs = ColorSpec::new();
        stdout.set_color(match self {
            ColorScheme::NoColors => &cs,
            ColorScheme::Default => cs.set_bold(true).set_fg(Some(Color::Blue)),
            ColorScheme::RegularAndBold => cs.set_bold(true),
            ColorScheme::Red => cs.set_fg(Some(Color::Ansi256(131))),
            ColorScheme::Green => cs.set_fg(Some(Color::Ansi256(118))),
            ColorScheme::Blue => cs.set_fg(Some(Color::Ansi256(33))),
            ColorScheme::White => cs.set_fg(Some(Color::Ansi256(255))),
        })?;
        write!(&mut stdout, "{}", wrapper)?;
        stdout.reset()?;
        Ok(())
    }

    pub fn inner_separator(&self, sep: &str) -> io::Result<()> {
        if self == &ColorScheme::NoColors {
            print!("{}", sep);
            return Ok(());
        }

        let mut stdout = StandardStream::stdout(self.color_choice());
        let mut cs = ColorSpec::new();
        stdout.set_color(match self {
            ColorScheme::NoColors => &cs,
            ColorScheme::Default => cs.set_bold(true).set_fg(Some(Color::Green)),
            ColorScheme::RegularAndBold => cs.set_bold(true),
            ColorScheme::Red => cs.set_fg(Some(Color::Ansi256(160))),
            ColorScheme::Green => cs.set_fg(Some(Color::Ansi256(120))),
            ColorScheme::Blue => cs.set_fg(Some(Color::Ansi256(105))),
            ColorScheme::White => cs.set_fg(Some(Color::Ansi256(252))),
        })?;
        write!(&mut stdout, "{}", sep)?;
        stdout.reset()?;
        Ok(())
    }
}

impl fmt::Display for ColorScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ColorScheme::*;
        match self {
            NoColors => f.write_str("None"),
            Default => f.write_str("Default"),
            RegularAndBold => f.write_str("RegularAndBold"),
            Red => f.write_str("Red"),
            Green => f.write_str("Green"),
            Blue => f.write_str("Blue"),
            White => f.write_str("White"),
        }
    }
}


#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default, Hash, PartialEq)]
pub enum Syntax {
    #[default]
    Human,
    Golang,
    Python,
    C,
    Cpp,
    Java,
    Kotlin,
    Rust,
}

impl Syntax {
    /// A human-readable representation of this syntax name
    pub fn represent(&self) -> &'static str {
        match self {
            Syntax::C => "C",
            Syntax::Cpp => "C++",
            Syntax::Golang => "golang",
            Syntax::Human => "human",
            Syntax::Java => "Java",
            Syntax::Kotlin => "Kotlin",
            Syntax::Python => "python",
            Syntax::Rust => "rust",
        }
    }
}
