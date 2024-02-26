use crate::errors::Errors;

use std::default;
use std::env;
use std::fmt;

use termcolor::StandardStreamLock;

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
            color_scheme: ColorScheme::NoColors,
            item: None,
            column: None,
            locale: String::from("en-US"),
            output_syntax: Syntax::Human,
        }
    }
}

impl Configuration {
    pub fn overwrite_with_clap(&mut self, out_radix: Option<u8>, out_item: Option<isize>, out_column: Option<String>, out_alpha_upper: Option<bool>, out_color_scheme: Option<String>, out_locale: Option<String>, out_syntax: Option<String>) -> Option<Errors> {
        if let Some(radix) = out_radix {
            if !(radix == 2 || radix == 10 || radix == 16) {
                return Some(Errors::CLIValueError("out-radix", "Only radices 2, 10, and 16 are supported".to_string()));
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
                None => return Some(Errors::CLIValueError("out-color-scheme", "Unknown color scheme".to_string())),
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
                "human" => Syntax::Human,
                "java" => Syntax::Java,
                "kotlin" => Syntax::Kotlin,
                "python" | "py" => Syntax::Python,
                "rust" | "rustlang" => Syntax::Rust,
                _ => return Some(Errors::CLIValueError("out-syntax", "Sorry, no support yet".to_string())),
            };
        }

        None
    }

    pub fn overwrite_with_env(&mut self) -> Option<Errors> {
        if let Ok(val) = env::var("OPSTR_RADIX") {
            match val.parse::<u8>() {
                Ok(r) => {
                    self.radix = r as usize;
                    if !(r == 2 || r == 10 || r == 16) {
                        return Some(Errors::CLIValueError("out-radix", "Only radices 2, 10, and 16 are supported".to_string()));
                    }            
                },
                Err(_) => return Some(Errors::CLIValueError("OPSTR_RADIX", "env value is not an integer".to_string())),
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
                None => return Some(Errors::CLIValueError("out-color-scheme", "Unknown color scheme".to_string())),
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
                _ => return Some(Errors::CLIValueError("out-syntax", "Sorry, no support yet".to_string())),
            };
        }

        None
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
    YellowBlue,
    // TODO more schemes
}

impl Eq for ColorScheme {}

// TODO: use termcolor instead
impl ColorScheme {
    /// Take a name and return the corresponding ColorScheme instance (or None, if unknown)
    pub fn by_name(scheme: &str) -> Option<Self> {
        use ColorScheme::*;
        match scheme.to_ascii_lowercase().as_str() {
            "none" => Some(NoColors),
            "default" => Some(Default),
            "yellowblue" => Some(YellowBlue),
            _ => None,
        }
    }

    /// Does this color scheme use any colors?
    pub fn uses_colors(&self) -> bool {
        *self != ColorScheme::NoColors
    }

    // TODO: use termcolor

    pub fn start_note(&self) {} // NOTE: must send to stderr, not stdout
    pub fn end_note(&self) {} // NOTE: must send to stderr, not stdout
    pub fn start_scalar(&self) {}
    pub fn end_scalar(&self) {}
    pub fn start_list(&self) {}
    pub fn end_list(&self) {}
    pub fn start_list_item(&self) {
        match self {
            ColorScheme::Default => print!("\x1B[38;2;240;200;50m"),
            ColorScheme::NoColors => {},
            ColorScheme::YellowBlue => print!("\x1B[38;2;196;70;1m"),
        };
    }
    pub fn end_list_item(&self) {
        match self {
            ColorScheme::Default => print!("\x1B[39;49m"),
            ColorScheme::NoColors => {},
            ColorScheme::YellowBlue => print!("\x1B[39;49m"),
        };
    }
    pub fn start_assoc(&self) {}
    pub fn end_assoc(&self) {}
    pub fn start_assoc_key(&self) {}
    pub fn end_assoc_key(&self) {}
    pub fn start_assoc_value(&self) {}
    pub fn end_assoc_value(&self) {}
    pub fn start_table(&self) {}
    pub fn end_table(&self) {}
    pub fn start_table_header(&self) {}
    pub fn end_table_header(&self) {}
    pub fn start_table_header_item(&self) {}
    pub fn end_table_header_item(&self) {}
    pub fn start_table_cell(&self) {}// TODO: this cannot be called if a table row is represented through Output::HeterogeneousList.print(…)
    pub fn end_table_cell(&self) {}
    pub fn start_op_item(&self, name: &str) {
        match self {
            ColorScheme::Default => println!("\x1b[0;33;49m{} \x1b[1;39;49m{} \x1b[0;33;49m{}\x1b[0;39;49m", "-".repeat(5), name, "-".repeat(80 - 2 - 5 - name.len())),
            ColorScheme::NoColors => {},
            ColorScheme::YellowBlue => println!("\x1b[0;33;49m{} \x1b[1;34;49m{} \x1b[0;33;49m{}\x1b[0;39;49m", "-".repeat(5), name, "-".repeat(80 - 2 - 5 - name.len())),
        };
    }
    pub fn end_op_item(&self, _name: &str) {}
}

impl fmt::Display for ColorScheme {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ColorScheme::*;
        match self {
            NoColors => f.write_str("None"),
            Default => f.write_str("Default"),
            YellowBlue => f.write_str("YellowBlue"),
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
    // TODO desired support: rust, bash.
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
