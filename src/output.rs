//! Output module responsible for generating helpful representation
//! for the answers to the query

use crate::config::Configuration;
use crate::config::Syntax;
use crate::errors::LibError;

use std::collections;

/// A scalar value in the result of the operation
#[non_exhaustive]
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum OutputValue {
    Bool(bool),
    Byte(u8),
    Int(i64),
    SingleLineText(String),
    MultiLineText(String),
}

impl Eq for OutputValue {}

impl OutputValue {
    /// Provide a string representation of the type in the syntax specified in `Configuration`
    pub(crate) fn typename(&self, conf: &Configuration) -> &'static str {
        self.represent_value_type(conf.syntax)
    }

    /// Create an `OutputValue` element from the provided string
    pub fn from_str(s: &str) -> OutputValue {
        if s.lines().count() > 1 {
            OutputValue::MultiLineText(s.to_owned())
        } else {
            OutputValue::SingleLineText(s.to_owned())
        }
    }

    pub(crate) fn represent_value_type(&self, syntax: Syntax) -> &'static str {
        // NOTE: Syntax::Human must satisfy that every type must have a different string representation
        match self {
            OutputValue::Bool(_) => {
                match syntax {
                    Syntax::Cpp | Syntax::Golang | Syntax::Human | Syntax::Java |
                    Syntax::Kotlin | Syntax::Python | Syntax::Rust => "bool",
                    Syntax::C => "uint8_t",
                    Syntax::Perl => "int",
                }
            },
            OutputValue::Byte(_) => {
                match syntax {
                    Syntax::C | Syntax::Cpp => "char",
                    Syntax::Golang | Syntax::Human => "byte",
                    Syntax::Java => "int",
                    Syntax::Kotlin => "UByte",
                    Syntax::Perl => "int",
                    Syntax::Python => "bytes",
                    Syntax::Rust => "u8",
                }
            },
            OutputValue::Int(_) => {
                match syntax {
                    Syntax::C | Syntax::Cpp => "uint64_t",
                    Syntax::Golang => "int64",
                    Syntax::Human | Syntax::Python => "int",
                    Syntax::Java => "long",
                    Syntax::Kotlin => "Long",
                    Syntax::Perl => "int",
                    Syntax::Rust => "int64",
                }
            },
            OutputValue::SingleLineText(_) => {
                match syntax {
                    Syntax::C => "char*",
                    Syntax::Cpp => "std::string",
                    Syntax::Golang => "string",
                    Syntax::Human => "single-line-text",
                    Syntax::Java | Syntax::Kotlin => "String",
                    Syntax::Perl => "q//",
                    Syntax::Python => "str",
                    Syntax::Rust => "&str",
                }
            },
            OutputValue::MultiLineText(_) => {
                match syntax {
                    Syntax::C => "char*",
                    Syntax::Cpp => "std::string",
                    Syntax::Golang => "string",
                    Syntax::Human => "multi-line-text",
                    Syntax::Java | Syntax::Kotlin => "String",
                    Syntax::Perl => "q//",
                    Syntax::Python => "str",
                    Syntax::Rust => "&str",
                }
            },
        }
    }

    #[allow(dead_code)]
    pub(crate) fn represent(&self, conf: &Configuration) -> String {
        match conf.syntax {
            Syntax::C | Syntax::Cpp => self.represent_c_cpp(conf),
            Syntax::Golang => self.represent_golang(conf),
            Syntax::Human => self.represent_human(conf),
            Syntax::Java => self.represent_java(conf),
            Syntax::Kotlin => self.represent_kotlin(conf),
            Syntax::Perl => self.represent_perl(conf),
            Syntax::Python => self.represent_python(conf),
            Syntax::Rust => self.represent_rust(conf),
        }
    }

    pub fn represent_c_cpp(&self, conf: &Configuration) -> String {
        match self {
            OutputValue::Bool(b) => {
                match conf.syntax {
                    Syntax::C => String::from(if *b { "1" } else { "0" }),
                    _ => String::from(if *b { "true" } else { "false" })
                }
            },
            OutputValue::Byte(b) => match conf.radix {
                2 => format!("0b{:08b}", *b),
                8 => format!("0{:03o}", *b),
                10 => format!("{}", *b),
                16 => if conf.alpha_upper { format!("0x{:02X}", *b) } else { format!("0x{:02x}", *b) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::Int(i) => match conf.radix {
                2 => format!("0b{:b}", *i),
                8 => format!("0{:o}", *i),
                10 => format!("{}", *i),
                16 => if conf.alpha_upper { format!("0x{:X}", *i) } else { format!("0x{:x}", *i) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::SingleLineText(t) |
            OutputValue::MultiLineText(t) => {
                let a = t.replace('\\', "\\\\").replace('\n', "\\n").replace('\r', "\\r").replace('\t', "\\t");
                let b = a.replace('\x07', "\\a").replace('\x08', "\\b").replace('\x0C', "\\f");
                let c = b.replace('\x0B', "\\v").replace('\'', "\\'").replace('"', "\\\"");
                format!("\"{}\"", c)
            },
        }
    }

    pub fn represent_golang(&self, conf: &Configuration) -> String {
        match self {
            OutputValue::Bool(b) => String::from(if *b { "true" } else { "false" }),
            OutputValue::Byte(b) => match conf.radix {
                2 => format!("0b{:08b}", *b),
                10 => format!("{}", *b),
                16 => if conf.alpha_upper { format!("0x{:02X}", *b) } else { format!("0x{:02x}", *b) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::Int(i) => match conf.radix {
                2 => format!("0b{:b}", *i),
                10 => format!("{}", *i),
                16 => if conf.alpha_upper { format!("0x{:X}", *i) } else { format!("0x{:x}", *i) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::SingleLineText(t) => {
                format!("\"{}\"", t.replace('"', "\\\""))
            },
            OutputValue::MultiLineText(t) => {
                format!("\"{}\"", t.replace('"', "\\\"").replace('\n', "\\n"))
            },
        }
    }

    pub(crate) fn represent_human(&self, conf: &Configuration) -> String {
        match self {
            OutputValue::Bool(b) => String::from(if *b { "true" } else { "false" }),
            OutputValue::Byte(b) => match conf.radix {
                2 => format!("{:08b}", *b),
                10 => format!("{}", *b),
                16 => if conf.alpha_upper { format!("{:02X}", *b) } else { format!("{:02x}", *b) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::Int(i) => match conf.radix {
                2 => format!("{:08b}", *i),
                10 => format!("{}", *i),
                16 => if conf.alpha_upper { format!("{:02X}", *i) } else { format!("{:02x}", *i) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::SingleLineText(t) => String::from(t),
            OutputValue::MultiLineText(t) => String::from(t),
        }
    }

    pub fn represent_java(&self, conf: &Configuration) -> String {
        match self {
            OutputValue::Bool(b) => String::from(if *b { "true" } else { "false" }),
            OutputValue::Byte(b) => match conf.radix {
                2 => format!("0b{:08b}", *b),
                10 => format!("{}", *b),
                16 => if conf.alpha_upper { format!("0x{:02X}", *b) } else { format!("0x{:02x}", *b) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::Int(i) => match conf.radix {
                2 => format!("0b{:b}", *i),
                8 => format!("0o{:o}", *i),
                10 => format!("{}", *i),
                16 => if conf.alpha_upper { format!("0x{:X}", *i) } else { format!("0x{:x}", *i) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::SingleLineText(t) | OutputValue::MultiLineText(t) => {
                t.replace('\\', "\\\\").replace('\t', "\\t")
                 .replace('\x08', "\\b").replace('\n', "\\n")
                 .replace('\r', "\\r").replace('\x0C', "\\f")
                 .replace('\'', "\\'").replace('"', "\\\"")
            },
        }
    }

    pub fn represent_kotlin(&self, conf: &Configuration) -> String {
        let escape_string = |t: &str| {
            t.replace('\\', "\\\\").replace('\t', "\\t").replace('\x08', "\\b")
             .replace('\n', "\\n").replace('\r', "\\r").replace('\'', "\\'")
             .replace('"', "\\\"").replace('$', "\\$")
        };
        match self {
            OutputValue::Bool(b) => String::from(if *b { "true" } else { "false" }),
            OutputValue::Byte(b) => match conf.radix {
                2 => format!("0b{:08b}u", *b),
                10 => format!("{}u", *b),
                16 => if conf.alpha_upper { format!("0x{:02X}u", *b) } else { format!("0x{:02x}u", *b) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::Int(i) => match conf.radix {
                2 => format!("0b{:b}uL", *i),
                10 => format!("{}uL", *i),
                16 => if conf.alpha_upper { format!("0x{:X}uL", *i) } else { format!("0x{:x}uL", *i) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::SingleLineText(t) => escape_string(t),
            OutputValue::MultiLineText(t) => {
                if !t.contains("\"\"\"") {
                    escape_string(t)
                } else {
                    format!("\"\"\"{}\"\"\"", t.replace('$', "${'$'}"))
                }
            },
        }
    }

    pub fn represent_perl(&self, conf: &Configuration) -> String {
        match self {
            OutputValue::Bool(b) => String::from(if *b { "1" } else { "0" }),
            OutputValue::Byte(b) => match conf.radix {
                2 => format!("0b{:08b}", *b),
                8 => format!("0o{:03o}", *b),
                10 => format!("{}", *b),
                16 => if conf.alpha_upper { format!("0x{:02X}", *b) } else { format!("0x{:02x}", *b) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::Int(i) => match conf.radix {
                2 => format!("0b{:b}", *i),
                8 => format!("0o{:o}", *i),
                10 => format!("{}", *i),
                16 => if conf.alpha_upper { format!("0x{:X}", *i) } else { format!("0x{:x}", *i) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::SingleLineText(t) => {
                format!("q/{}/", t.replace("/", "\\/"))
            },
            OutputValue::MultiLineText(t) => {
                let esc1 = t.replace("\\", "\\\\");
                let esc2 = esc1.replace("\0", "\\0").replace("\n", "\\n").replace("\r", "\\r");
                let esc3 = esc2.replace("\t", "\\t").replace("\x0B", "\\v").replace("\x0C", "\\f");
                let esc4 = esc3.replace("\x1b", "\\e").replace("\x07", "\\a").replace("\x08", "\\b");
                format!("\"{}\"", esc4)
            },
        }
    }

    pub fn represent_python(&self, conf: &Configuration) -> String {
        match self {
            OutputValue::Bool(b) => String::from(if *b { "True" } else { "False" }),
            OutputValue::Byte(b) => match conf.radix {
                2 => format!("0b{:08b}", *b),
                8 => format!("0o{:03o}", *b),
                10 => format!("{}", *b),
                16 => if conf.alpha_upper { format!("0x{:02X}", *b) } else { format!("0x{:02x}", *b) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::Int(i) => match conf.radix {
                2 => format!("0b{:b}", *i),
                8 => format!("0o{:o}", *i),
                10 => format!("{}", *i),
                16 => if conf.alpha_upper { format!("0x{:X}", *i) } else { format!("0x{:x}", *i) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::SingleLineText(t) => {
                if !t.contains('"') {
                    format!("\"{}\"", t)
                } else if !t.contains('\'') {
                    format!("'{}'", t)
                } else {
                    format!("'''{}'''", t.replace('\'', "\\'"))
                }
            },
            OutputValue::MultiLineText(t) => {
                if !t.contains('"') {
                    format!("\"\"\"{}\"\"\"", t)
                } else if !t.contains('\'') {
                    format!("'''{}'''", t)
                } else {
                    format!("'''{}'''", t.replace('\'', "\\'"))
                }
            },
        }
    }

    pub fn represent_rust(&self, conf: &Configuration) -> String {
        match self {
            OutputValue::Bool(b) => String::from(if *b { "true" } else { "false" }),
            OutputValue::Byte(b) => match conf.radix {
                2 => format!("0b{:08b}", *b),
                8 => format!("0o{:03o}", *b),
                10 => format!("{}", *b),
                16 => if conf.alpha_upper { format!("0x{:02X}", *b) } else { format!("0x{:02x}", *b) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::Int(i) => match conf.radix {
                2 => format!("0b{:b}", *i),
                8 => format!("0o{:o}", *i),
                10 => format!("{}", *i),
                16 => if conf.alpha_upper { format!("0x{:X}", *i) } else { format!("0x{:x}", *i) },
                _ => panic!("unsupported radix: {}", conf.radix),
            },
            OutputValue::SingleLineText(t) => {
                if !t.contains('"') {
                    format!("\"{}\"", t.replace('\\', "\\\\"))
                } else if !t.contains('\'') {
                    format!("'{}'", t.replace('\\', "\\\\"))
                } else {
                    let mut count = 1;
                    loop {
                        if !t.contains(&"#".repeat(count)) {
                            break;
                        }
                        count += 1;
                    }
                    // string representation like: r###"sample string"###;
                    format!("r{delim}\"{text}\"{delim}", delim="#".repeat(count), text=t)
                }
            },
            OutputValue::MultiLineText(t) => {
                if !t.contains('"') {
                    format!("\"{}\"", t.replace('\\', "\\\\").replace('\n', "\\n"))
                } else if !t.contains('\'') {
                    format!("'{}'", t.replace('\\', "\\\\").replace('\n', "\\n"))
                } else {
                    let mut count = 1;
                    loop {
                        if !t.contains(&"#".repeat(count)) {
                            break;
                        }
                        count += 1;
                    }
                    // string representation like: r###"sample string"###;
                    format!("r{delim}\"{text}\"{delim}", delim="#".repeat(count), text=t)
                }
            },
        }
    }
}

/// Output is the result of an operation ready for user-friendly representation through the `print` method
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq)]
pub enum Output {
    Scalar{ data: OutputValue, notes: Vec<String> },
    /// A list where all items have the same OutputValue type (e.g. Int)
    HomogeneousList{ data: Vec<OutputValue>, notes: Vec<String> },
    /// A list where the items likely have different OutputValue types (e.g. Int and Bool)
    HeterogeneousList{ data: Vec<OutputValue>, notes: Vec<String> },
    /// Associates a key to a value
    /// TODO: redefine as Vec<(OutputValue, OutputValue)> to allow ordered entries
    Association{ data: collections::HashMap<OutputValue, OutputValue>, notes: Vec<String> },
    /// Creates a table.
    /// ASSUME: for every row in data { assert!(len(row) == len(column_headers)); }
    Table{ data: Vec<Vec<OutputValue>>, column_headers: Vec<String>, notes: Vec<String> },
}

impl Eq for Output {}

type Err = Result<(), LibError>;

impl Output {
    /// Create an `Output` element from the provided list of `OutputValue` items
    pub fn from_value_list(value_list: &[OutputValue], notes: &[String]) -> Output {
        let mut is_homogeneous = true;
        let mut typename = "";

        for value in value_list.iter() {
            let this_typename = value.represent_value_type(Syntax::Human);
            if typename.is_empty() {
                typename = this_typename;
            } else if typename != this_typename {
                is_homogeneous = false;
                break;
            }
        }

        if is_homogeneous {
            Output::HomogeneousList { data: value_list.to_vec(), notes: notes.to_vec() }
        } else {
            Output::HeterogeneousList { data: value_list.to_vec(), notes: notes.to_vec() }
        }
    }

    /// Attach an additional note to this output
    /// (warning/imprecision note/configuration insufficiencies/…)
    #[allow(dead_code)]
    pub fn add_note(&mut self, note: &str) {
        match self {
            Output::Scalar { notes, .. } |
            Output::HomogeneousList { notes, .. } |
            Output::HeterogeneousList { notes, .. } |
            Output::Association { notes, .. } |
            Output::Table { notes, .. } => {
                notes.push(note.to_owned());
            },
        }
    }

    fn is_homogeneous(&self) -> bool {
        match self {
            Output::Scalar { .. } => true,
            Output::HomogeneousList { .. } => true,
            Output::HeterogeneousList { .. } => false,
            Output::Association { data, .. } => {
                let mut typename = "";
                for (key, value) in data.iter() {
                    if typename.is_empty() {
                        typename = key.represent_value_type(Syntax::Human);
                    } else if typename != key.represent_value_type(Syntax::Human)
                           || typename != value.represent_value_type(Syntax::Human)
                    {
                        return false;
                    }
                }
                true
            },
            Output::Table { data, .. } => {
                let mut typename = "";
                for row in data.iter() {
                    for cell in row {
                        if typename.is_empty() {
                            typename = cell.represent_value_type(Syntax::Human);
                        } else if typename != cell.represent_value_type(Syntax::Human) {
                            return false;
                        }
                    }
                }
                true
            },
        }
    }

    /// Return the `OutputValue` representing "nothing". If some `type_value` is provided,
    /// then returned `OutputValue` matches the variant of the given `OutputValue`.
    /// Otherwise, the type is a `OutputValue::SingleLineText`.
    fn zero_value(type_value: Option<&OutputValue>) -> OutputValue {
        match type_value {
            None => OutputValue::SingleLineText("".to_owned()),
            Some(discriminator) => {
                match discriminator {
                    OutputValue::Bool(_) => OutputValue::Bool(false),
                    OutputValue::Byte(_) => OutputValue::Byte(0),
                    OutputValue::Int(_) => OutputValue::Int(0),
                    OutputValue::SingleLineText(_) => OutputValue::SingleLineText("".to_owned()),
                    OutputValue::MultiLineText(_) => OutputValue::MultiLineText("".to_owned()),
                }
            }
        }
    }

    /// Represent this `Output` in stdout & stderr.
    ///
    /// NOTE: Sadly, I could not implement this as part of some `fmt` trait, because
    /// library `termcolor` needs to write to stdout/stderr directly.
    pub fn print(&self, conf: &Configuration) -> Err {
        if let Some(idx) = conf.item {
            if let Some(col) = &conf.column {
                self.reduce_by_column(conf, col).reduce_by_index(conf, idx).print_internally(conf)
            } else {
                self.reduce_by_index(conf, idx).print_internally(conf)
            }
        } else {
            if let Some(col) = &conf.column {
                self.reduce_by_column(conf, col).print_internally(conf)
            } else {
                self.print_internally(conf)
            }
        }
    }

    fn reduce_by_index(&self, _conf: &Configuration, index: isize) -> Output {
        // `item` selects the `item`-th item of a list
        if let Output::HomogeneousList{ data, .. } | Output::HeterogeneousList{ data, .. } = self {
            if index < 0 {
                let normalized_index = index.rem_euclid(data.len() as isize) as usize;
                let index_item = data[normalized_index].clone();
                Output::Scalar { data: index_item, notes: vec![] }

            } else if index >= data.len() as isize {
                // NOTE: if we are out of bounds, we want to return "nothing".
                //       "nothing" does not exist in our data model, so we want to align it with the first item.
                let nothing_value = Self::zero_value(data.get(0));
                Output::Scalar {
                    data: nothing_value,
                    notes: vec![format!("item argument {} is out of bounds {}", index, data.len())]
                }

            } else {
                let index_item = data[index as usize].clone();
                Output::Scalar { data: index_item, notes: vec![] }
            }

        // `item` selects the `item`-th row of a table
        } else if let Output::Table{ data, column_headers, notes } = self {
        
            let mut row_index = index as usize;
            if index >= data.len() as isize {
                Output::Table {
                    data: vec![],
                    column_headers: column_headers.clone(),
                    notes: vec![format!("item argument {} is out of bounds {}", row_index, data.len())]
                }

            } else {
                if index < 0 {
                    row_index = index.rem_euclid(data.len() as isize) as usize;
                }

                Output::Table {
                    data: vec![data[row_index].clone()],
                    column_headers: column_headers.clone(),
                    notes: notes.clone()
                }
            }
        } else {
            self.clone()
        }
    }

    fn reduce_by_column(&self, _conf: &Configuration, column_name: &str) -> Output {
        // `column` selects the value where `column` equals key in an association
        if let Output::Association { data, .. } = self {
            let mut selected_value = None;
            for (key, value) in data {
                let comparison_key = match key {
                    OutputValue::Bool(b) => (if *b { "true" } else { "false" }).to_owned(),
                    OutputValue::Byte(b) => format!("{}", b),
                    OutputValue::Int(i) => format!("{}", i),
                    OutputValue::SingleLineText(s) => s.to_owned(),
                    OutputValue::MultiLineText(s) => s.to_owned(),
                };
                if &comparison_key == column_name {
                    selected_value = Some(value.clone());
                }
            }
            match selected_value {
                Some(val) => Output::Scalar { data: val, notes: vec![] },
                None => Output::Scalar {
                    data: Self::zero_value(None),
                    notes: vec![format!("column argument '{}' is unknown", column_name)]
                },
            }

        // `column` selects the column with this name in a table
        } else if let Output::Table{ data, column_headers, notes } = self {
            let mut index = None;
            for (i, header) in column_headers.iter().enumerate() {
                if header == column_name {
                    index = Some(i);
                }
            }

            match index {
                Some(column_id) => {
                    let selected_data = data.iter().map(|row| { row[column_id].clone() }).collect::<Vec<OutputValue>>();
                    Output::from_value_list(&selected_data, &notes)
                },
                None => {
                    Output::Table {
                        data: vec![],
                        column_headers: vec![column_name.to_owned()],
                        notes: vec![format!("column argument '{}' is unknown", column_name)]
                    }
                }
            }
        } else {
            self.clone()
        }
    }

    fn print_internally(&self, conf: &Configuration) -> Err {
        let col = conf.color_scheme;

        match self {
            Output::Scalar { notes, .. } |
            Output::HomogeneousList { notes, .. } |
            Output::HeterogeneousList { notes, .. } |
            Output::Association { notes, .. } |
            Output::Table { notes, .. } => {
                for note in notes {
                    col.note_label("NOTE")?;
                    eprintln!(": {}", note);
                }
            }
        }

        // TODO provide variable name argument to `print_` functions
        match conf.syntax {
            Syntax::C | Syntax::Cpp => self.print_c_cpp(conf),
            Syntax::Golang => self.print_golang(conf),
            Syntax::Human => self.print_human(conf),
            Syntax::Java => self.print_java(conf),
            Syntax::Kotlin => self.print_kotlin(conf),
            Syntax::Perl => self.print_perl(conf),
            Syntax::Python => self.print_python(conf),
            Syntax::Rust => self.print_rust(conf),
        }?;

        Ok(())
    }

    fn print_c_cpp(&self, conf: &Configuration) -> Err {
        let col = conf.color_scheme;

        match self {
            Output::Scalar { data, .. } => {
                println!("{}", data.represent_c_cpp(conf));
            },
            Output::HomogeneousList { data,  .. } => {
                if data.is_empty() {
                    print!("int ");
                    col.keyword("list")?;
                    print!("[0] = ");
                    col.inner_wrapper("{}")?;
                    println!(";");
                } else {
                    print!("{} ", data[0].typename(conf));
                    col.keyword("list")?;
                    print!("[{}] = ", data.len());
                    col.inner_wrapper("{")?;
                    for (i, elem) in data.iter().enumerate() {
                        print!("{}", elem.represent_c_cpp(conf));

                        if i != data.len() - 1 {
                            col.inner_separator(", ")?;
                        }
                    }
                    col.inner_wrapper("}")?;
                    println!(";");
                }
            },
            Output::HeterogeneousList { data, .. } => {
                col.note_label("NOTE")?;
                eprintln!(": heterogeneous lists cannot be handled in this syntax!");

                if data.is_empty() {
                    print!("int ");
                    col.keyword("list")?;
                    print!("[0] = ");
                    col.inner_wrapper("{}")?;
                    println!(";");
                    return Ok(());
                }

                print!("void* ");
                col.keyword("list")?;
                print!("[{}] = ", data.len());
                col.inner_wrapper("{")?;
                for (i, elem) in data.iter().enumerate() {
                    print!("{}", elem.represent_c_cpp(conf));

                    if i != data.len() - 1 {
                        col.inner_separator(", ")?;
                    }
                }
                col.inner_wrapper("}")?;
                println!(";");
            },
            Output::Association { data, .. } => {
                if data.is_empty() {
                    print!("int ");
                    col.keyword("keys")?;
                    print!("[0] = ");
                    col.inner_wrapper("{}")?;
                    println!(";");
                    print!("int ");
                    col.keyword("values")?;
                    print!("[0] = ");
                    col.inner_wrapper("{}")?;
                    println!(";");
                    return Ok(());
                }

                let keys = &Vec::from_iter(data.keys().cloned());
                let values = &Vec::from_iter(data.values().cloned());

                let key_list = Output::from_value_list(keys, &[]);
                let value_list = Output::from_value_list(values, &[]);

                println!("// {} keys and values", data.len());
                // NOTE: no proper variable name for two lists, no improved coloring, …
                //       I did not put a lot of effort into this.
                key_list.print_c_cpp(conf)?;
                value_list.print_c_cpp(conf)?;
            },
            Output::Table { data, column_headers, .. } => {
                // generate representation
                print!("const char* ");
                col.keyword("headers")?;
                print!("[{}] = ", column_headers.len());
                col.inner_wrapper("{")?;
                for (i, description) in column_headers.iter().enumerate() {
                    let header = OutputValue::SingleLineText(description.to_owned());
                    print!("{}", header.represent_c_cpp(conf));
                    if i != column_headers.len() - 1 {
                        col.inner_separator(", ")?;
                    }
                }
                col.inner_wrapper("}")?;
                println!(";");

                if data.is_empty() {
                    print!("int ");
                    col.keyword("table")?;
                    print!("[0][0] = ");
                    col.inner_wrapper("{}")?;
                    println!(";");
                    return Ok(());
                }

                // NOTE: no special coloring, no two-dimensional array, …
                //       I did not put a lot of effort into this.
                for row in data.iter() {
                    let list = Output::from_value_list(row, &[]);
                    list.print_c_cpp(conf)?;
                }
            },
        }

        Ok(())
    }

    fn print_golang(&self, conf: &Configuration) -> Err {
        let col = conf.color_scheme;

        match self {
            Output::Scalar { data, .. } => {
                println!("{}", data.represent_golang(conf));
            },
            Output::HomogeneousList{ data, .. } => {
                if data.is_empty() {
                    print!("[]");
                    col.keyword("int64")?;
                    col.inner_wrapper("{}")?;
                    println!(";");
                } else {
                    print!("[]");
                    col.keyword(data[0].typename(conf))?;
                    col.inner_wrapper("{")?;
                    for (i, elem) in data.iter().enumerate() {
                        print!("{}", elem.represent_golang(conf));

                        if i != data.len() - 1 {
                            col.inner_separator(", ")?;
                        }
                    }
                    col.inner_wrapper("}")?;
                    println!("");
                }
            },
            Output::HeterogeneousList{ data, .. } => {
                if data.is_empty() {
                    print!("[]");
                    col.keyword("any")?;
                    col.inner_wrapper("{}")?;
                    println!(";");
                } else {
                    print!("[]");
                    col.keyword("any")?;
                    col.inner_wrapper("{")?;
                    for (i, elem) in data.iter().enumerate() {
                        print!("{}", elem.represent_golang(conf));

                        if i != data.len() - 1 {
                            col.inner_separator(", ")?;
                        }
                    }
                    col.inner_wrapper("}")?;
                    println!("");
                }
            },
            Output::Association { data, .. } => {
                // TODO introduce special case if data.is_homogeneous()?
                print!("map[");
                col.keyword("any")?;
                print!("]");
                col.keyword("any")?;
                col.outer_wrapper("{")?;
                for (i, (key, value)) in data.iter().enumerate() {
                    print!("{}", key.represent_golang(conf));

                    col.inner_separator(": ")?;

                    print!("{}", value.represent_golang(conf));

                    if i != data.len() - 1 {
                        col.outer_separator(", ")?;
                    }
                }
                col.outer_wrapper("}")?;
                println!("");
            },
            Output::Table { data, column_headers, .. } => {
                // generate representation
                print!("header := []");
                col.keyword("string")?;
                col.inner_wrapper("{")?;
                for (i, description) in column_headers.iter().enumerate() {
                    let header = OutputValue::SingleLineText(description.to_owned());
                    print!("{}", header.represent_golang(conf));
                    if i != column_headers.len() - 1 {
                        col.inner_separator(", ")?;
                    }
                }
                col.inner_wrapper("}")?;
                println!("");

                // are all types the same?
                let mut typename = "";
                if self.is_homogeneous() {
                    for row in data.iter() {
                        if !row.is_empty() {
                            typename = row[0].typename(conf);
                        }
                    }
                }
                if typename.is_empty() {
                    typename = "any";
                }

                print!("[][]");
                col.keyword(typename)?;
                col.outer_wrapper("{")?;
                for (i, row) in data.iter().enumerate() {
                    let list = Output::from_value_list(row, &[]);
                    list.print_golang(conf)?;
                    if i != row.len() - 1 {
                        col.outer_separator(", ")?;
                    }
                }
                col.outer_wrapper("}")?;
                println!("");
            },
        }

        Ok(())
    }

    fn print_human(&self, conf: &Configuration) -> Err {
        let col = conf.color_scheme;

        match self {
            Output::Scalar{ data: scalar, .. } => {
                println!("{}", scalar.represent_human(conf));
            },
            Output::HomogeneousList{ data: list, .. } |
            Output::HeterogeneousList{ data: list, .. } => {
                //let mut any_is_multiline = false;
                col.inner_wrapper("[ ")?;
                for (i, elem) in list.iter().enumerate() {
                    println!("{}", elem.represent_human(conf));

                    if i != list.len() - 1 {
                        col.inner_separator("| ")?;
                    }
                }
                col.inner_wrapper("]")?;
                println!("");
            },
            Output::Association{ data: assoc, .. } => {
                // NOTE: humans prefer it sorted
                //       for other syntaxes I won't sort though, because the operation is sufficiently expensive
                let mut key_order = Vec::from_iter(assoc.keys());
                let mut max_key_width = 0;
                key_order.sort_by_key(|key| {
                    let repr = key.represent_human(conf);
                    max_key_width = max_key_width.max(repr.chars().count());
                    repr
                });

                col.outer_wrapper("{ ")?;
                for (i, key) in key_order.iter().enumerate() {
                    let value = assoc.get(key).unwrap();
                    if i != 0 {
                        println!();
                        col.outer_separator("| ")?;
                    }

                    if max_key_width > 0 && max_key_width < 80 {
                        let repr_key = key.represent_human(conf);
                        print!("{}", repr_key);
                        col.inner_separator("::  ")?;
                        let already_shown = repr_key.chars().count() + 4;
                        let to_show = 8 - (already_shown % 8);
                        print!("{}", " ".repeat(to_show));
                    } else {
                        print!("{}", key.represent_human(conf));
                        col.inner_separator(":: ")?;
                    }
                    print!("{}", value.represent_human(conf));
                }
                println!("");
                col.outer_wrapper("}")?;
                println!("");
            },
            Output::Table{ column_headers, data: table_data, .. } => {
                // compute column widths
                let mut column_widths = Vec::new();
                for column_description in column_headers.iter() {
                    column_widths.push(column_description.chars().count());
                }
                for row in table_data.iter() {
                    for (i, col) in row.iter().enumerate() {
                        column_widths[i] = column_widths[i].max(col.represent_human(conf).chars().count());
                    }
                }

                // generate representation
                for (description, width) in column_headers.iter().zip(&column_widths) {
                    col.keyword(&format!("{0: <width$}", description, width=width))?;
                    print!(" ");
                }
                println!();
                for width in column_widths.iter() {
                    col.inner_separator(&format!("{}", "─".repeat(*width)))?;
                    print!(" ");
                }
                println!();

                // TODO continuous line below table header?

                for row in table_data.iter() {
                    for (column, width) in row.iter().zip(&column_widths) {
                        print!("{: <width$}", column.represent_human(conf), width=width);
                        print!(" ");
                    }
                    println!();
                }
            },
        }

        Ok(())
    }

    fn print_java(&self, conf: &Configuration) -> Err {
        let col = conf.color_scheme;

        match self {
            Output::Scalar { data, .. } => {
                println!("{}", data.represent_java(conf));
            },
            Output::HomogeneousList { data,  .. } => {
                if data.is_empty() {
                    print!("new ");
                    col.keyword("int")?;
                    print!("[]");
                    col.outer_wrapper("{}")?;
                    println!(";");
                } else {
                    print!("new ");
                    col.keyword(data[0].typename(conf))?;
                    print!("[] = ");
                    col.inner_wrapper("{")?;
                    for (i, elem) in data.iter().enumerate() {
                        print!("{}", elem.represent_java(conf));

                        if i != data.len() - 1 {
                            col.inner_separator(", ")?;
                        }
                    }
                    col.inner_wrapper("}")?;
                    println!(";");
                }
            },
            Output::HeterogeneousList { data, .. } => {
                if data.is_empty() {
                    println!("List<");
                    col.keyword("Object")?;
                    print!("> items = new ");
                    col.keyword("ArrayList")?;
                    println!("();");
                    return Ok(());
                }

                print!("List<Object> items = ");
                println!("List<Object> ");
                col.keyword("items")?;
                print!(" = Arrays.asList");
                col.inner_wrapper("(")?;
                for (i, elem) in data.iter().enumerate() {
                    print!("{}", elem.represent_java(conf));

                    if i != data.len() - 1 {
                        col.inner_separator(", ")?;
                    }
                }
                col.inner_wrapper(")")?;
                println!(";");
            },
            Output::Association { data, .. } => {
                if data.is_empty() {
                    print!("Map<String, String> ");
                    col.keyword("map")?;
                    println!(" = new HashMap();");
                } else if data.len() == 1 {
                    let key = data.values().next().unwrap();
                    let value = data.values().next().unwrap();
                    print!("Collections.singletonMap");
                    col.inner_wrapper("(")?;
                    print!("{}", key.represent_java(conf));
                    col.inner_separator(", ")?;
                    print!("{}", value.represent_java(conf));
                    col.inner_wrapper(")")?;
                    println!("");
                } else {
                    print!("Map<Object, Object> ");
                    col.keyword("map")?;
                    print!(" = Map.ofEntries");
                    col.outer_wrapper("(")?;
                    println!("");
                    for (key, value) in data.iter() {
                        print!("  entry");
                        col.inner_wrapper("(")?;
                        print!("{}", key.represent_java(conf));
                        col.inner_separator(", ")?;
                        print!("{}", value.represent_java(conf));
                        col.inner_wrapper(")")?;
                        println!(",");
                    }
                    col.outer_wrapper(")")?;
                    println!(";");
                }
            },
            Output::Table { data, column_headers, .. } => {
                // generate representation
                print!("List<String> ");
                col.keyword("headers")?;
                print!(" = Arrays.asList");
                col.inner_wrapper("(")?;
                for (i, description) in column_headers.iter().enumerate() {
                    let header = OutputValue::SingleLineText(description.to_owned());
                    print!("{}", header.represent_java(conf));
                    if i != column_headers.len() - 1 {
                        col.inner_wrapper(", ")?;
                    }
                }
                col.inner_wrapper(")")?;
                println!(";");

                if data.is_empty() {
                    print!("int[][] ");
                    col.keyword("emptyTable")?;
                    println!(";");
                    return Ok(());
                }

                print!("Object[][] ");
                col.keyword("table")?;
                print!(" = ");
                col.outer_wrapper("{")?;
                println!("");
                for (row_id, row) in data.iter().enumerate() {
                    print!("  ");
                    col.inner_wrapper("{")?;
                    print!(" ");
                    for (cell_id, cell) in row.iter().enumerate() {
                        print!("{}", cell.represent_java(conf));
                        if cell_id < row.len() - 1 {
                            col.inner_separator(", ")?;
                        }
                    }
                    if row_id < data.len() - 1 {
                        print!(" ");
                        col.inner_wrapper("}")?;
                        col.outer_separator(",")?;
                    } else {
                        print!(" ");
                        col.inner_wrapper("}")?;
                    }
                    println!("");
                }
                col.outer_wrapper("}")?;
                println!(";");
            },
        }

        Ok(())
    }

    fn print_kotlin(&self, conf: &Configuration) -> Err {
        let col = conf.color_scheme;

        match self {
            Output::Scalar { data, .. } => {
                println!("{}", data.represent_kotlin(conf));
            },
            Output::HomogeneousList { data,  .. } => {
                if data.is_empty() {
                    col.keyword("emptyArray")?;
                    println!("()");
                } else {
                    col.keyword("arrayOf")?;
                    col.inner_wrapper("(")?;
                    for (i, elem) in data.iter().enumerate() {
                        print!("{}", elem.represent_kotlin(conf));

                        if i != data.len() - 1 {
                            col.inner_separator(", ")?;
                        }
                    }
                    col.inner_wrapper(")")?;
                    println!("");
                }
            },
            Output::HeterogeneousList { data, .. } => {
                if data.is_empty() {
                    col.keyword("emptyList")?;
                    println!("()");
                    return Ok(());
                }

                col.keyword("listOf")?;
                col.inner_wrapper("(")?;
                for (i, elem) in data.iter().enumerate() {
                    print!("{}", elem.represent_kotlin(conf));

                    if i != data.len() - 1 {
                        col.inner_separator(", ")?;
                    }
                }
                col.inner_wrapper(")")?;
                println!("");
            },
            Output::Association { data, .. } => {
                if data.is_empty() {
                    col.keyword("emptyMap")?;
                    println!("()");
                    return Ok(());
                }

                col.keyword("mapOf")?;
                col.inner_wrapper("(")?;
                for (key, value) in data.iter() {
                    print!("{}", key.represent_kotlin(conf));
                    col.inner_separator(" to ")?;
                    print!("{}", value.represent_kotlin(conf));
                    col.outer_separator(",")?;
                }
                col.inner_wrapper(")")?;
                println!("");
            },
            Output::Table { data, column_headers, .. } => {
                // generate representation
                print!("val ");
                col.keyword("headers")?;
                print!(" = listOf");
                col.inner_wrapper("(")?;
                for (i, description) in column_headers.iter().enumerate() {
                    let header = OutputValue::SingleLineText(description.to_owned());
                    print!("{}", header.represent_kotlin(conf));
                    if i != column_headers.len() - 1 {
                        col.inner_separator(", ")?;
                    }
                }
                col.inner_wrapper(")")?;
                println!("");

                if data.is_empty() {
                    print!("val ");
                    col.keyword("table")?;
                    println!(" = emptyList()");
                    return Ok(());
                }

                print!("val ");
                col.keyword("table")?;
                print!(" = listOf");
                col.outer_wrapper("(")?;
                println!("");
                for (row_id, row) in data.iter().enumerate() {
                    print!("  listOf");
                    col.inner_wrapper("(")?;
                    for (cell_id, cell) in row.iter().enumerate() {
                        print!("{}", cell.represent_kotlin(conf));
                        if cell_id < row.len() - 1 {
                            col.inner_separator(", ")?;
                        }
                    }
                    if row_id < data.len() - 1 {
                        col.inner_wrapper(")")?;
                        println!(",");
                    } else {
                        col.inner_wrapper(")")?;
                        println!("");
                    }
                }
                col.outer_wrapper(")")?;
                println!(";");
            },
        }

        Ok(())
    }

    fn print_perl(&self, conf: &Configuration) -> Err {
        let col = conf.color_scheme;

        match self {
            Output::Scalar { data, .. } => {
                println!("{}", data.represent_perl(conf));
            },
            Output::HomogeneousList { data, .. } | Output::HeterogeneousList { data, .. } => {
                println!("({})", data.iter().map(|v| v.represent_perl(conf)).collect::<Vec<String>>().join(", ") );
            },
            Output::Association { data, .. } => {
                // NOTE: perl only accepts strings as keys in hashes

                // convert keys to strings
                let mut data_key_to_str_key = collections::HashMap::new();
                let mut all_keys_are_strings = true;
                for key in data.keys() {
                    if let OutputValue::MultiLineText(_) | OutputValue::SingleLineText(_) = key {
                        data_key_to_str_key.insert(key, key.to_owned());
                    } else {
                        all_keys_are_strings = false;
                        let str_forced = key.represent_perl(conf);
                        let str_repr = OutputValue::SingleLineText(str_forced);
                        data_key_to_str_key.insert(key, str_repr);
                    }
                }

                if !all_keys_are_strings {
                    col.note_label("NOTE")?;
                    println!(": perl hashes only accept strings as keys - keys have been converted");
                }

                let text_of_output_value = |v: &OutputValue| {
                    if let OutputValue::MultiLineText(t) | OutputValue::SingleLineText(t) = v {
                        t.to_owned()
                    } else {
                        // NOTE: must not happen because all keys must be OutputValue::*Text
                        String::new()
                    }
                };

                let new_keys = data_key_to_str_key.values().map(text_of_output_value).collect::<Vec<String>>();
                let mut new_keys_are_unique = true;
                for (i, v) in new_keys.iter().enumerate() {
                    if let Some(idx) = new_keys.iter().position(|e| { e == v }) {
                        if i != idx {
                            new_keys_are_unique = false;
                        }
                    }
                }

                // NOTE: since we generate perl string representations, it might happen that elements are not unique.
                //       In this case, perl will throw an error. Since we cannot prevent it, I want to inform the user about it.
                if !new_keys_are_unique {
                    col.note_label("NOTE")?;
                    println!(": perl hash keys must be unique - sadly these keys are not unique but I couldn't prevent it");
                }

                // write content to stdout
                print!("( ");
                let count = data.len();
                let mut i = 0;
                for (key, value) in data.iter() {
                    let new_key = match data_key_to_str_key.get(key) {
                        Some(k) => k,
                        None => continue,
                    };

                    i += 1;
                    if i == count {
                        print!("{} => {}", new_key.represent_perl(conf), value.represent_perl(conf));
                    } else {
                        println!("{} => {},", new_key.represent_perl(conf), value.represent_perl(conf));
                    }
                }
                println!(");");
            },
            Output::Table { data, column_headers, .. } => {
                let col_header_values = column_headers.iter().map(|v| { OutputValue::from_str(&v) }).collect::<Vec<OutputValue>>();
                print!("my @headers = ");
                println!("({});", col_header_values.iter().map(|v| v.represent_perl(conf)).collect::<Vec<String>>().join(", ") );
                println!("@table = (");
                for row in data.iter() {
                    print!("\t[ ");
                    for cell in row.iter() {
                        print!("{}, ", cell.represent_perl(conf));
                    }
                    print!("],");
                }
                println!(");");
            },
        }

        Ok(())
    }

    fn print_python(&self, conf: &Configuration) -> Err {
        let col = conf.color_scheme;

        match self {
            Output::Scalar { data, .. } => {
                println!("{}", data.represent_python(conf));
            },
            Output::HomogeneousList{ data: list, .. } |
            Output::HeterogeneousList{ data: list, .. } => {
                col.inner_wrapper("[")?;
                for (i, elem) in list.iter().enumerate() {
                    print!("{}", elem.represent_python(conf));

                    if i != list.len() - 1 {
                        col.inner_separator(", ")?;
                    }
                }
                col.inner_wrapper("]")?;
                println!("");
            },
            Output::Association { data, .. } => {
                col.outer_wrapper("{")?;
                for (i, (key, value)) in data.iter().enumerate() {
                    print!("{}", key.represent_python(conf));

                    col.inner_separator(": ")?;

                    print!("{}", value.represent_python(conf));

                    if i != data.len() - 1 {
                        col.outer_separator(", ")?;
                    }
                }
                col.outer_wrapper("}")?;
                println!("");
            },
            Output::Table { data, column_headers, .. } => {
                // compute column widths
                let mut column_widths = Vec::new();
                for column_description in column_headers.iter() {
                    column_widths.push(column_description.len());
                }
                for row in data.iter() {
                    for (i, col) in row.iter().enumerate() {
                        column_widths[i] = column_widths[i].max(col.represent_python(conf).len());
                    }
                }

                // generate representation
                col.outer_wrapper("[")?;
                col.inner_wrapper("[")?;
                for (description, width) in column_headers.iter().zip(&column_widths) {
                    let header = OutputValue::SingleLineText(format!("{:─^width$}", description, width=*width));
                    print!("{}", header.represent_python(conf));
                    col.inner_wrapper(", ")?;
                }
                col.inner_wrapper("]")?;
                col.outer_separator(",")?;
                println!("");

                for (i, row) in data.iter().enumerate() {
                    col.inner_wrapper(" [")?;
                    for (i, (cell, width)) in row.iter().zip(&column_widths).enumerate() {
                        print!("{:─^width$}", cell.represent_python(conf), width=*width);
                        if i != row.len() - 1 {
                            col.inner_separator(", ")?;
                        }
                    }
                    col.inner_wrapper("]")?;
                    if i != data.len() - 1 {
                        col.outer_separator(",")?;
                    }
                }
                col.outer_wrapper("]")?;
                println!("");
            },
        }

        Ok(())
    }

    fn print_rust(&self, conf: &Configuration) -> Err {
        let col = conf.color_scheme;
        let repr_val = |val: &OutputValue| {
            match val {
                OutputValue::Bool(b) => print!("Val::Bool({})", if *b { "true" } else { "false" }),
                OutputValue::Byte(y) => print!("Val::Byte(0x{:02X})", y),
                OutputValue::Int(i) => print!("Val::Int({})", *i),
                OutputValue::SingleLineText(_) => print!("Val::OneLineString({}.to_owned())", val.represent_rust(conf)),
                OutputValue::MultiLineText(_) => print!("Val::MultiLineString({}.to_owned())", val.represent_rust(conf)),
            }
        };

        let val_declaration = || -> Err {
            println!("#[derive(Clone, Debug, Hash, PartialEq)]");
            print!("enum ");
            col.keyword("Val")?;
            println!(" {{");

            print!("  ");
            col.keyword("Bool")?;
            println!("(bool),");

            print!("  ");
            col.keyword("Byte")?;
            println!("(u8),");

            print!("  ");
            col.keyword("Int")?;
            println!("(i64),");

            print!("  ");
            col.keyword("OneLineString")?;
            println!("(String),");

            print!("  ");
            col.keyword("MultiLineString")?;
            println!("(String),");
            println!("}}");

            Ok(())
        };

        match self {
            Output::Scalar { data, .. } => {
                println!("{}", data.represent_rust(conf));
            },
            Output::HomogeneousList { data,  .. } => {
                if data.is_empty() {
                    print!("let mut ");
                    col.keyword("list")?;
                    println!(": [i32; 0];");
                } else {
                    print!("let mut ");
                    col.keyword("array")?;
                    print!(": [");
                    col.keyword(data[0].typename(conf))?;
                    print!("; ");
                    col.keyword(&format!("{}", data.len()))?;
                    print!("] = ");
                    col.inner_wrapper("[")?;
                    for (i, elem) in data.iter().enumerate() {
                        print!("{}", elem.represent_rust(conf));

                        if i != data.len() - 1 {
                            col.inner_separator(", ")?;
                        }
                    }
                    col.inner_wrapper("]")?;
                    println!(";");
                }
            },
            Output::HeterogeneousList { data, .. } => {
                if data.is_empty() {
                    print!("let mut ");
                    col.keyword("list")?;
                    println!(": [i32; 0];");

                } else {
                    val_declaration()?;

                    print!("let ");
                    col.keyword("list")?;
                    print!(": [Val; {}] = ", data.len());
                    col.inner_wrapper("[")?;
                    for (i, elem) in data.iter().enumerate() {
                        repr_val(elem);

                        if i != data.len() - 1 {
                            col.inner_separator(", ")?;
                        }
                    }
                    col.inner_wrapper("]")?;
                    println!(";");
                }
            },
            Output::Association { data, .. } => {
                col.note_label("NOTE")?;
                println!(": heterogeneous maps cannot be handled in this syntax");

                if data.is_empty() {
                    print!("let mut ");
                    col.keyword("map")?;
                    println!(": HashMap<String, String> = HashMap::new();");

                } else {
                    val_declaration()?;

                    print!("let mut ");
                    col.keyword("map")?;
                    println!(": HashMap<Val, Val> = HashMap::new();");
                    for (key, value) in data.iter() {
                        print!("map.insert");
                        col.inner_wrapper("(")?;
                        repr_val(key);
                        col.inner_separator(", ")?;
                        repr_val(value);
                        col.inner_wrapper(")")?;
                        println!(";");
                    }
                }
            },
            Output::Table { data, column_headers, .. } => {
                val_declaration()?;

                // generate representation
                print!("let ");
                col.keyword("headers")?;
                print!("[&'static str; {}] = ", column_headers.len());
                col.inner_wrapper("[")?;
                for (i, description) in column_headers.iter().enumerate() {
                    let header = OutputValue::SingleLineText(description.to_owned());
                    print!("{}", header.represent_rust(conf));
                    if i != column_headers.len() - 1 {
                        col.inner_separator(", ")?;
                    }
                }
                col.inner_wrapper("]")?;
                println!(";");

                if data.is_empty() {
                    print!("let ");
                    col.keyword("table")?;
                    println!(" = vec![];");

                } else {
                    print!("let ");
                    col.keyword("table")?;
                    println!(": Vec<Vec<Val>> = vec!");
                    col.outer_wrapper("[")?;
                    println!("");
                    for (i, row) in data.iter().enumerate() {
                        print!("  vec!");
                        col.inner_wrapper("[")?;
                        for (j, cell) in row.iter().enumerate() {
                            print!("{}", cell.represent_rust(conf));
                            if j != row.len() - 1 {
                                col.inner_separator(", ")?;
                            }
                        }
                        col.inner_wrapper("]")?;
                        if i != data.len() - 1 {
                            col.outer_separator(",")?;
                        }
                        println!("");
                    }
                    col.outer_wrapper("]")?;
                    println!(";");
                }
            },
        }

        Ok(())
    }
}

// NOTE: simple conversions follow as syntactic sugar

impl From<bool> for Output {
    fn from(value: bool) -> Self {
        Output::Scalar{ data: OutputValue::Bool(value), notes: vec![] }
    }
}

impl From<u8> for Output {
    fn from(value: u8) -> Self {
        Output::Scalar{ data: OutputValue::Byte(value), notes: vec![] }
    }
}

impl From<usize> for Output {
    fn from(value: usize) -> Self {
        Output::Scalar{ data: OutputValue::Int(value as i64), notes: vec![] }
    }
}

impl From<i64> for Output {
    fn from(value: i64) -> Self {
        Output::Scalar{ data: OutputValue::Int(value), notes: vec![] }
    }
}

impl From<String> for Output {
    fn from(value: String) -> Self {
        if value.lines().count() <= 1 {
            Output::Scalar{ data: OutputValue::SingleLineText(value), notes: vec![] }
        } else {
            Output::Scalar{ data: OutputValue::MultiLineText(value), notes: vec![] }
        }
    }
}

impl From<&str> for Output {
    fn from(value: &str) -> Self {
        if value.lines().count() <= 1 {
            Output::Scalar{ data: OutputValue::SingleLineText(value.to_owned()), notes: vec![] }
        } else {
            Output::Scalar{ data: OutputValue::MultiLineText(value.to_owned()), notes: vec![] }
        }
    }
}
