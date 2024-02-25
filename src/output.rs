//! Output module responsible for generating helpful representation
//! for the answers to the query

use crate::config::Configuration;
use crate::config::Syntax;

use std::collections;

#[derive(Clone, Debug, Hash, PartialEq)]
pub enum OutputValue {
    Bool(bool),
    Byte(u8),
    Int(i64),
    SingleLineText(String),
    MultiLineText(String),
}

impl Eq for OutputValue {}

// TODO: how debug messages for stderr?

impl OutputValue {
    pub(crate) fn typename(&self, conf: &Configuration) -> &'static str {
        self.represent_value_type(conf.output_syntax)
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
                }
            },
            OutputValue::Byte(_) => {
                match syntax {
                    Syntax::C | Syntax::Cpp => "char",
                    Syntax::Golang | Syntax::Human => "byte",
                    Syntax::Java => "int",
                    Syntax::Kotlin => "UByte",
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
                    Syntax::Python => "str",
                    Syntax::Rust => "&str",
                }
            },
        }
    }

    pub(crate) fn represent(&self, conf: &Configuration) -> String {
        match conf.output_syntax {
            Syntax::C | Syntax::Cpp => self.represent_c_cpp(conf),
            Syntax::Golang => self.represent_golang(conf),
            Syntax::Human => self.represent_human(conf),
            Syntax::Java => self.represent_java(conf),
            Syntax::Kotlin => self.represent_kotlin(conf),
            Syntax::Python => self.represent_python(conf),
            Syntax::Rust => self.represent_rust(conf),
            _ => "value-of-unsupported-type".to_owned(),
        }
    }

    pub fn represent_c_cpp(&self, conf: &Configuration) -> String {
        match self {
            OutputValue::Bool(b) => {
                match conf.output_syntax {
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

#[derive(Clone, Debug, PartialEq)]
pub enum Output {
    Scalar{ data: OutputValue, notes: Vec<String> },
    /// A list where all items have the same OutputValue type (e.g. Int)
    HomogeneousList{ data: Vec<OutputValue>, notes: Vec<String> },
    /// A list where the items likely have different OutputValue types (e.g. Int and Bool)
    HeterogeneousList{ data: Vec<OutputValue>, notes: Vec<String> },
    /// Associates a key to a value
    Association{ data: collections::HashMap<OutputValue, OutputValue>, notes: Vec<String> },
    /// Creates a table.
    /// ASSUME: for every row in data { assert!(len(row) == len(column_headers)); }
    Table{ data: Vec<Vec<OutputValue>>, column_headers: Vec<String>, notes: Vec<String> },
}

impl Eq for Output {}

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
    pub(crate) fn add_note(&mut self, note: &str) {
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

    pub fn print(&self, conf: &Configuration) {
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


    fn print_internally(&self, conf: &Configuration) {
        let col = conf.color_scheme;

        match self {
            Output::Scalar { notes, .. } |
            Output::HomogeneousList { notes, .. } |
            Output::HeterogeneousList { notes, .. } |
            Output::Association { notes, .. } |
            Output::Table { notes, .. } => {
                for note in notes {
                    col.start_note();
                    eprintln!("NOTE: {}", note);
                    col.end_note();
                }
            }
        }

        match conf.output_syntax {
            Syntax::C | Syntax::Cpp => self.print_c_cpp(conf),
            Syntax::Golang => self.print_golang(conf),
            Syntax::Human => self.print_human(conf),
            Syntax::Java => self.print_java(conf),
            Syntax::Kotlin => self.print_kotlin(conf),
            Syntax::Python => self.print_python(conf),
            Syntax::Rust => self.print_rust(conf),
        }
    }

    pub fn print_c_cpp(&self, conf: &Configuration) {
        let col = conf.color_scheme;

        match self {
            Output::Scalar { data, .. } => {
                col.start_scalar();
                println!("{}", data.represent_c_cpp(conf));
                col.end_scalar();
            },
            Output::HomogeneousList { data,  .. } => {
                if data.is_empty() {
                    println!("int list[0] = {{}};");
                } else {
                    col.start_list();
                    print!("{} list[{}] = {{", data[0].typename(conf), data.len());
                    for (i, elem) in data.iter().enumerate() {
                        col.start_list_item();
                        print!("{}", elem.represent_c_cpp(conf));
                        col.end_list_item();

                        if i != data.len() - 1 {
                            print!(", ");
                        }
                    }
                    println!("}};");
                    col.end_list();
                }
            },
            Output::HeterogeneousList { data, .. } => {
                eprintln!("NOTE: heterogeneous lists cannot be handled in this syntax!");

                if data.is_empty() {
                    println!("int list[0] = {{}};");
                    return;
                }

                col.start_list();
                print!("void* list[{}] = {{", data.len());
                for (i, elem) in data.iter().enumerate() {
                    col.start_list_item();
                    print!("{}", elem.represent_c_cpp(conf));
                    col.end_list_item();

                    if i != data.len() - 1 {
                        print!(", ");
                    }
                }
                println!("}};");
                col.end_list();
            },
            Output::Association { data, .. } => {
                // TODO sort by keys
                if data.is_empty() {
                    println!("int keys[0] = {{}};");
                    println!("int values[0] = {{}};");
                    return;
                }

                let keys = &Vec::from_iter(data.keys().cloned());
                let values = &Vec::from_iter(data.values().cloned());

                let key_list = Output::from_value_list(keys, &[]);
                let value_list = Output::from_value_list(values, &[]);

                col.start_assoc();
                print!("// {} keys and values", data.len());
                // NOTE: no col.start_assoc_key, no col.start_assoc_value, no proper variable name for two lists, …
                //       I did not put a lot of effort into this.
                key_list.print_c_cpp(conf);
                value_list.print_c_cpp(conf);
                col.end_assoc();
            },
            Output::Table { data, column_headers, .. } => {
                // generate representation
                col.start_table();
                col.start_table_header();
                print!("const char* headers[{}] = {{", column_headers.len());
                for (i, description) in column_headers.iter().enumerate() {
                    col.start_table_header_item();
                    let header = OutputValue::SingleLineText(description.to_owned());
                    print!("{}", header.represent_c_cpp(conf));
                    col.end_table_header_item();
                    if i != column_headers.len() - 1 {
                        print!(", ");
                    }
                }
                println!("}}");
                col.end_table_header();

                if data.is_empty() {
                    println!("int table[0][0] = {{}};");
                    return;
                }

                // NOTE: no col.start_table_cell, no two-dimensional array, …
                //       I did not put a lot of effort into this.
                col.start_table();
                for row in data.iter() {
                    let list = Output::from_value_list(row, &[]);
                    list.print_c_cpp(conf);
                }
                col.end_table();
            },
        }
    }

    pub fn print_golang(&self, conf: &Configuration) {
        let col = conf.color_scheme;

        match self {
            Output::Scalar { data, .. } => {
                col.start_scalar();
                println!("{}", data.represent_golang(conf));
                col.end_scalar();
            },
            Output::HomogeneousList{ data, .. } => {
                if data.is_empty() {
                    println!("[]int64{{}}");
                } else {
                    col.start_list();
                    print!("[]{}{{", data[0].typename(conf));
                    for (i, elem) in data.iter().enumerate() {
                        col.start_list_item();
                        print!("{}", elem.represent_golang(conf));
                        col.end_list_item();

                        if i != data.len() - 1 {
                            print!(", ");
                        }
                    }
                    println!("}}");
                    col.end_list();
                }
            },
            Output::HeterogeneousList{ data, .. } => {
                if data.is_empty() {
                    println!("[]any{{}}");
                } else {
                    col.start_list();
                    print!("[]any{{");
                    for (i, elem) in data.iter().enumerate() {
                        col.start_list_item();
                        print!("{}", elem.represent_golang(conf));
                        col.end_list_item();

                        if i != data.len() - 1 {
                            print!(", ");
                        }
                    }
                    println!("}}");
                    col.end_list();
                }
            },
            Output::Association { data, .. } => {
                // TODO sort by keys
                // TODO introduce special case if data.is_homogeneous()?
                col.start_assoc();
                print!("map[any]any{{");
                for (i, (key, value)) in data.iter().enumerate() {
                    col.start_assoc_key();
                    print!("{}", key.represent_golang(conf));
                    col.end_assoc_key();

                    print!(": ");

                    col.start_assoc_value();
                    print!("{}", value.represent_golang(conf));
                    col.end_assoc_value();

                    if i != data.len() - 1 {
                        print!(", ");
                    }
                }
                println!("}}");
                col.end_assoc();
            },
            Output::Table { data, column_headers, .. } => {
                // generate representation
                col.start_table();
                col.start_table_header();
                print!("header := []string{{");
                for (i, description) in column_headers.iter().enumerate() {
                    col.start_table_header_item();
                    let header = OutputValue::SingleLineText(description.to_owned());
                    print!("{}", header.represent_golang(conf));
                    col.end_table_header_item();
                    if i != column_headers.len() - 1 {
                        print!(", ");
                    }
                }
                println!("}}");
                col.end_table_header();

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

                print!("[][]{}", typename);
                print!("{{");
                for (i, row) in data.iter().enumerate() {
                    let list = Output::from_value_list(row, &[]);
                    list.print_golang(conf);
                    if i != row.len() - 1 {
                        print!(", ");
                    }
                }
                println!("}}");
                col.end_table();
            },
        }
    }

    pub fn print_human(&self, conf: &Configuration) {
        let col = conf.color_scheme;

        match self {
            Output::Scalar{ data: scalar, .. } => {
                col.start_scalar();
                println!("{}", scalar.represent_human(conf));
                col.end_scalar();
            },
            Output::HomogeneousList{ data: list, .. } |
            Output::HeterogeneousList{ data: list, .. } => {
                //let mut any_is_multiline = false;
                col.start_list();
                print!("[ ");
                for (i, elem) in list.iter().enumerate() {
                    col.start_list_item();
                    println!("{}", elem.represent_human(conf));
                    col.end_list_item();

                    if i != list.len() - 1 {
                        print!("| ");
                    }
                }
                println!("]");
                col.end_list();
            },
            Output::Association{ data: assoc, .. } => {
                // TODO sort by keys
                col.start_assoc();
                print!("{{ ");
                for (i, (key, value)) in assoc.iter().enumerate() {
                    if i != 0 {
                        println!();
                        print!("| ");
                    }

                    col.start_assoc_key();
                    print!("{}", key.represent_human(conf));
                    col.end_assoc_key();

                    print!(":: ");

                    col.start_assoc_value();
                    print!("{}", value.represent_human(conf));
                    col.end_assoc_value();
                }
                println!("}}");
                col.end_assoc();
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
                col.start_table();
                col.start_table_header();
                for (description, width) in column_headers.iter().zip(&column_widths) {
                    col.start_table_header_item();
                    print!("{0: <width$}", description, width=width);
                    col.end_table_header_item();
                    print!(" ");
                }
                println!();
                for width in column_widths.iter() {
                    col.start_table_header_item();
                    print!("{}", "─".repeat(*width));
                    col.end_table_header_item();
                    print!(" ");
                }
                println!();
                col.end_table_header();

                // TODO continuous line below table header?

                for row in table_data.iter() {
                    for (column, width) in row.iter().zip(&column_widths) {
                        col.start_table_cell();
                        print!("{: <width$}", column.represent_human(conf), width=width);
                        col.end_table_cell();
                        print!(" ");
                    }
                    println!();
                }
                col.end_table();
            },
        }
    }

    pub fn print_java(&self, conf: &Configuration) {
        let col = conf.color_scheme;

        match self {
            Output::Scalar { data, .. } => {
                col.start_scalar();
                println!("{}", data.represent_java(conf));
                col.end_scalar();
            },
            Output::HomogeneousList { data,  .. } => {
                if data.is_empty() {
                    println!("new int[]{{}};");
                } else {
                    col.start_list();
                    print!("new {}[] = {{", data[0].typename(conf));
                    for (i, elem) in data.iter().enumerate() {
                        col.start_list_item();
                        print!("{}", elem.represent_java(conf));
                        col.end_list_item();

                        if i != data.len() - 1 {
                            print!(", ");
                        }
                    }
                    println!("}};");
                    col.end_list();
                }
            },
            Output::HeterogeneousList { data, .. } => {
                if data.is_empty() {
                    println!("List<Object> items = new ArrayList();");
                    return;
                }

                col.start_list();
                print!("List<Object> items = Arrays.asList(");
                for (i, elem) in data.iter().enumerate() {
                    col.start_list_item();
                    print!("{}", elem.represent_java(conf));
                    col.end_list_item();

                    if i != data.len() - 1 {
                        print!(", ");
                    }
                }
                println!(");");
                col.end_list();
            },
            Output::Association { data, .. } => {
                // TODO sort by keys
                if data.is_empty() {
                    println!("Map<String, String> map = new HashMap();");
                } else if data.len() == 1 {
                    let key = data.values().next().unwrap();
                    let value = data.values().next().unwrap();
                    col.start_assoc();
                    println!("Collections.singletonMap({}, {})", key.represent_java(conf), value.represent_java(conf));
                    col.end_assoc();
                } else {
                    col.start_assoc();
                    println!("Map<Object, Object> ,ap = Map.ofEntries(");
                    for (key, value) in data.iter() {
                        print!("  entry(");
                        col.start_assoc_key();
                        print!("{}", key.represent_java(conf));
                        col.end_assoc_key();
                        print!(", ");
                        col.start_assoc_value();
                        print!("{}", value.represent_java(conf));
                        col.end_assoc_value();
                        println!("),");
                    }
                    println!(");");
                    col.end_assoc();
                }
            },
            Output::Table { data, column_headers, .. } => {
                // generate representation
                col.start_table();
                col.start_table_header();
                print!("List<String> headers = Arrays.asList(");
                for (i, description) in column_headers.iter().enumerate() {
                    col.start_table_header_item();
                    let header = OutputValue::SingleLineText(description.to_owned());
                    print!("{}", header.represent_java(conf));
                    col.end_table_header_item();
                    if i != column_headers.len() - 1 {
                        print!(", ");
                    }
                }
                println!(");");
                col.end_table_header();

                if data.is_empty() {
                    println!("int[][] emptyTable;");
                    return;
                }

                col.start_table();
                println!("Object[][] table = {{");
                for (row_id, row) in data.iter().enumerate() {
                    print!("  {{ ");
                    for (cell_id, cell) in row.iter().enumerate() {
                        print!("{}", cell.represent_java(conf));
                        if cell_id < row.len() - 1 {
                            print!(", ");
                        }
                    }
                    if row_id < data.len() - 1 {
                        println!(" }},");
                    } else {
                        println!(" }}");
                    }
                }
                print!("}};");
                col.end_table();
            },
        }
    }

    pub fn print_kotlin(&self, conf: &Configuration) {
        let col = conf.color_scheme;

        match self {
            Output::Scalar { data, .. } => {
                col.start_scalar();
                println!("{}", data.represent_kotlin(conf));
                col.end_scalar();
            },
            Output::HomogeneousList { data,  .. } => {
                if data.is_empty() {
                    println!("emptyArray()");
                } else {
                    col.start_list();
                    print!("arrayOf(");
                    for (i, elem) in data.iter().enumerate() {
                        col.start_list_item();
                        print!("{}", elem.represent_kotlin(conf));
                        col.end_list_item();

                        if i != data.len() - 1 {
                            print!(", ");
                        }
                    }
                    println!(")");
                    col.end_list();
                }
            },
            Output::HeterogeneousList { data, .. } => {
                if data.is_empty() {
                    println!("emptyList()");
                    return;
                }

                col.start_list();
                print!("listOf(");
                for (i, elem) in data.iter().enumerate() {
                    col.start_list_item();
                    print!("{}", elem.represent_kotlin(conf));
                    col.end_list_item();

                    if i != data.len() - 1 {
                        print!(", ");
                    }
                }
                println!(")");
                col.end_list();
            },
            Output::Association { data, .. } => {
                // TODO sort by keys
                if data.is_empty() {
                    println!("emptyMap()");
                    return;
                }

                col.start_assoc();
                print!("mapOf(");
                for (key, value) in data.iter() {
                    col.start_assoc_key();
                    print!("{}", key.represent_kotlin(conf));
                    col.end_assoc_key();
                    print!(" to ");
                    col.start_assoc_value();
                    print!("{}", value.represent_kotlin(conf));
                    col.end_assoc_value();
                    println!(",");
                }
                print!(")");
                col.end_assoc();
            },
            Output::Table { data, column_headers, .. } => {
                // generate representation
                col.start_table();
                col.start_table_header();
                print!("val headers = listOf(");
                for (i, description) in column_headers.iter().enumerate() {
                    col.start_table_header_item();
                    let header = OutputValue::SingleLineText(description.to_owned());
                    print!("{}", header.represent_kotlin(conf));
                    col.end_table_header_item();
                    if i != column_headers.len() - 1 {
                        print!(", ");
                    }
                }
                println!(")");
                col.end_table_header();

                if data.is_empty() {
                    println!("val table = emptyList()");
                    return;
                }

                col.start_table();
                println!("val table = listOf(");
                for (row_id, row) in data.iter().enumerate() {
                    print!("  listOf(");
                    for (cell_id, cell) in row.iter().enumerate() {
                        print!("{}", cell.represent_kotlin(conf));
                        if cell_id < row.len() - 1 {
                            print!(", ");
                        }
                    }
                    if row_id < data.len() - 1 {
                        println!("),");
                    } else {
                        println!(")");
                    }
                }
                print!(");");
                col.end_table();
            },
        }
    }

    pub fn print_python(&self, conf: &Configuration) {
        let col = conf.color_scheme;

        match self {
            Output::Scalar { data, .. } => {
                col.start_scalar();
                println!("{}", data.represent_python(conf));
                col.end_scalar();
            },
            Output::HomogeneousList{ data: list, .. } |
            Output::HeterogeneousList{ data: list, .. } => {
                col.start_list();
                print!("[");
                for (i, elem) in list.iter().enumerate() {
                    col.start_list_item();
                    print!("{}", elem.represent_python(conf));
                    col.end_list_item();

                    if i != list.len() - 1 {
                        print!(", ");
                    }
                }
                print!("]");
                col.end_list();
            },
            Output::Association { data, .. } => {
                // TODO sort by keys
                col.start_assoc();
                print!("{{");
                for (i, (key, value)) in data.iter().enumerate() {
                    col.start_assoc_key();
                    print!("{}", key.represent_python(conf));
                    col.end_assoc_key();

                    print!(": ");

                    col.start_assoc_value();
                    print!("{}", value.represent_python(conf));
                    col.end_assoc_value();

                    if i != data.len() - 1 {
                        print!(", ");
                    }
                }
                print!("}}");
                col.end_assoc();
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
                col.start_table();
                print!("[");
                col.start_table_header();
                print!("[");
                for (description, width) in column_headers.iter().zip(&column_widths) {
                    col.start_table_header_item();
                    let header = OutputValue::SingleLineText(format!("{:─^width$}", description, width=*width));
                    print!("{}, ", header.represent_python(conf));
                    col.end_table_header_item();
                }
                println!("]");
                col.end_table_header();
                println!(",");

                for row in data.iter() {
                    print!(" [");
                    for (i, (cell, width)) in row.iter().zip(&column_widths).enumerate() {
                        col.start_table_cell();
                        print!("{:─^width$}", cell.represent_python(conf), width=*width);
                        col.end_table_cell();
                        if i != row.len() - 1 {
                            print!(", ");
                        }
                    }
                    println!("]");
                }
                print!("]");
                col.end_table();
            },
        }
    }

    pub fn print_rust(&self, conf: &Configuration) {
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

        match self {
            Output::Scalar { data, .. } => {
                col.start_scalar();
                println!("{}", data.represent_rust(conf));
                col.end_scalar();
            },
            Output::HomogeneousList { data,  .. } => {
                if data.is_empty() {
                    col.start_list();
                    println!("let mut list: [i32; 0];");
                    col.end_list();
                } else {
                    col.start_list();
                    print!("let mut array: [{}; {}] = [", data[0].typename(conf), data.len());
                    for (i, elem) in data.iter().enumerate() {
                        col.start_list_item();
                        print!("{}", elem.represent_rust(conf));
                        col.end_list_item();

                        if i != data.len() - 1 {
                            print!(", ");
                        }
                    }
                    println!("];");
                    col.end_list();
                }
            },
            Output::HeterogeneousList { data, .. } => {
                if data.is_empty() {
                    col.start_list();
                    println!("let mut list: [i32; 0];");
                    col.end_list();

                } else {
                    println!("#[derive(Clone, Debug, Hash, PartialEq)]");
                    println!("enum Val {{");
                    println!("  Bool(bool),");
                    println!("  Byte(u8),");
                    println!("  Int(i64),");
                    println!("  OneLineString(String),");
                    println!("  MultiLineString(String),");
                    println!("}}");

                    col.start_list();
                    print!("let list: [Val; {}] = [", data.len());
                    for (i, elem) in data.iter().enumerate() {
                        col.start_list_item();
                        repr_val(elem);
                        col.end_list_item();

                        if i != data.len() - 1 {
                            print!(", ");
                        }
                    }
                    println!("];");
                    col.end_list();
                }
            },
            Output::Association { data, .. } => {
                // TODO sort by keys
                eprintln!("NOTE: heterogeneous maps cannot be handled in this syntax");

                if data.is_empty() {
                    col.start_assoc();
                    println!("let mut map: HashMap<String, String> = HashMap::new();");
                    col.end_assoc();

                } else {
                    println!("#[derive(Clone, Debug, Hash, PartialEq)]");
                    println!("enum Val {{");
                    println!("  Bool(bool),");
                    println!("  Byte(u8),");
                    println!("  Int(i64),");
                    println!("  OneLineString(String),");
                    println!("  MultiLineString(String),");
                    println!("}}");

                    col.start_assoc();
                    println!("let mut map: HashMap<Val, Val> = HashMap::new();");
                    for (key, value) in data.iter() {
                        print!("map.insert(");
                        col.start_assoc_key();
                        repr_val(key);
                        col.end_assoc_key();
                        print!(", ");
                        col.start_assoc_value();
                        repr_val(value);
                        col.start_assoc_value();
                        println!(");");
                    }
                    col.end_assoc();
                }
            },
            Output::Table { data, column_headers, .. } => {
                println!("#[derive(Clone, Debug, Hash, PartialEq)]");
                println!("enum Val {{");
                println!("  Bool(bool),");
                println!("  Byte(u8),");
                println!("  Int(i64),");
                println!("  OneLineString(String),");
                println!("  MultiLineString(String),");
                println!("}}");

                // generate representation
                col.start_table();
                col.start_table_header();
                print!("let headers[&'static str; {}] = [", column_headers.len());
                for (i, description) in column_headers.iter().enumerate() {
                    col.start_table_header_item();
                    let header = OutputValue::SingleLineText(description.to_owned());
                    print!("{}", header.represent_rust(conf));
                    col.end_table_header_item();
                    if i != column_headers.len() - 1 {
                        print!(", ");
                    }
                }
                println!("];");
                col.end_table_header();

                if data.is_empty() {
                    println!("let table = vec![];");

                } else {
                    col.start_table();
                    println!("let table: Vec<Vec<Val>> = vec![");
                    for row in data.iter() {
                        print!("  vec![");
                        for cell in row.iter() {
                            col.start_table_cell();
                            print!("{}", cell.represent_rust(conf));
                            col.end_table_cell();
                        }
                        println!("]");
                    }
                    println!("];");
                    col.end_table();
                }
            },
        }
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


/*
pub struct Results {
    config: repr::CLIConfig,
    generic: Vec<repr::Response>, // ops_utf8.rs
    charsets: Vec<repr::Response>, // ops_charsets.rs
    stats: Vec<repr::Response>, // ops_stats.rs
    unicode: Vec<repr::Response>, // ops_unicode.rs
    locale: Vec<repr::Response>, // ops_locale.rs
    xml: Vec<repr::Response>, // ops_xml.rs
}

// TODO sort vec<repr::Response> based on the length of its CLI representation

impl default::Default for Results {
    fn default() -> Self {
        Results {
            config: repr::CLIConfig::default(),
            generic: vec![],
            charsets: vec![],
            stats: vec![],
            unicode: vec![],
            locale: vec![],
            xml: vec![],
        }
    }
}

impl Results {
    pub fn add_line(&mut self, section: &str, line: String) -> &mut Self {
        let line_repr = if line.len() < 80 {
            repr::Response::ShortSingleLine(line)
        } else {
            repr::Response::LongSingleLine(line)
        };
        match section {
            "generic" => self.generic.push(line_repr),
            "charsets" => self.charsets.push(line_repr),
            "stats" => self.stats.push(line_repr),
            "unicode" => self.unicode.push(line_repr),
            "locale" => self.locale.push(line_repr),
            "xml" => self.xml.push(line_repr),
            _ => panic!("programming error: unknown section '{}'", section),
        };

        self
    }
}

impl fmt::Display for Results {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.generic.is_empty() {
            for item in &self.generic {
                writeln!(f, "• {}", item.to_str(&self.config, 0))?;
            }
        }

        if !self.charsets.is_empty() {
            writeln!(f, "## text encoding", )?;
            writeln!(f, "")?;
            for item in &self.charsets {
                writeln!(f, "• {}", item.to_str(&self.config, 0))?;
            }
        }

        if !self.stats.is_empty() {
            writeln!(f, "## statistics", )?;
            writeln!(f, "")?;
            for item in &self.stats {
                writeln!(f, "• {}", item.to_str(&self.config, 0))?;
            }
        }

        if !self.unicode.is_empty() {
            writeln!(f, "## Unicode", )?;
            writeln!(f, "")?;
            for item in &self.unicode {
                writeln!(f, "• {}", item.to_str(&self.config, 0))?;
            }
        }

        if !self.locale.is_empty() {
            writeln!(f, "## locale", )?;
            writeln!(f, "")?;
            for item in &self.locale {
                writeln!(f, "• {}", item.to_str(&self.config, 0))?;
            }
        }

        if !self.xml.is_empty() {
            writeln!(f, "## XML", )?;
            writeln!(f, "")?;
            for item in &self.xml {
                writeln!(f, "• {}", item.to_str(&self.config, 0))?;
            }
        }

        write!(f, "")
    }
}
*/

/*
pub trait BytesRepresentations {
    fn to_bash_repr(&self, conf: &Configuration) -> String;
    fn to_golang_repr(&self, conf: &Configuration) -> String;
    fn to_python_repr(&self, conf: &Configuration) -> String;
    fn to_c_repr(&self, conf: &Configuration) -> String;
    fn to_rust_repr(&self, conf: &Configuration) -> String;
    fn to_hex_repr(&self, conf: &Configuration) -> String;
    fn to_hexstring_repr(&self, conf: &Configuration) -> String;
    fn has_byte(&self, b: u8) -> bool;
}

/// Auxiliary function taking a sequence of bytes and returning the same elements as strings
/// according to Configuration.
fn as_radixed_numbers(nums: &[u8], conf: &Configuration, enable_binary: bool, enable_hex: bool) -> Vec<String> {
    let radix_to_use = match conf.radix {
        2 => if enable_binary { 2 } else { 10 },
        16 => if enable_hex { 16 } else { 10 },
        _ => 10,
    };
    match radix_to_use {
        2 => nums.iter().map(|i| format!("0b{:b}", i)).collect::<Vec<String>>(),
        16 => {
            if conf.hex_upper {
                nums.iter().map(|i| format!("0x{:02X}", i)).collect::<Vec<String>>()
            } else {
                nums.iter().map(|i| format!("0x{:02x}", i)).collect::<Vec<String>>()
            }
        },
        _ => nums.iter().map(|i| format!("{}", i)).collect::<Vec<String>>(),
    }
}

impl BytesRepresentations for Vec<u8> {
    fn to_bash_repr(&self, conf: &Configuration) -> String {
        let mut result = "( ".to_string();
        result.push_str(&as_radixed_numbers(&self, conf, false, true).join(" "));
        result.push_str(" )");
        result
    }

    fn to_golang_repr(&self, conf: &Configuration) -> String {
        // TODO: b"…" representation for ASCII printables
        //let utf8_str = String::from_utf8(self);

        let mut result = "[".to_string();
        result.push_str(&as_radixed_numbers(&self, conf, true, true).join(", "));
        result.push_str("]");
        result
    }

    fn to_python_repr(&self, conf: &Configuration) -> String {
        let mut result = "[".to_string();
        result.push_str(&as_radixed_numbers(&self, conf, true, true).join(", "));
        result.push_str("]");
        result
    }

    fn to_c_repr(&self, conf: &Configuration) -> String {
        let mut result = "[".to_string();
        result.push_str(&as_radixed_numbers(&self, conf, true, true).join(", "));
        result.push_str("]");
        result
    }

    fn to_rust_repr(&self, conf: &Configuration) -> String {
        // TODO: "…" representation for ASCII printables
        //let utf8_str = String::from_utf8(self);

        let mut result = format!("const char str[{}] = {{", self.len());
        result.push_str(&as_radixed_numbers(&self, conf, true, true).join(", "));
        result.push_str("};");
        result
    }

    fn to_hex_repr(&self, conf: &Configuration) -> String {
        match conf.hex_upper {
            true => hex::encode_upper(&self),
            false => hex::encode(&self),
        }
    }

    fn to_hexstring_repr(&self, conf: &Configuration) -> String {
        match conf.hex_upper {
            true => self.iter().map(|x| format!("{:02X} ", *x)).collect(),
            false => self.iter().map(|x| format!("{:02x} ", *x)).collect(),
        }
    }

    // TODO base64?

    /// Returns whether the given byte is part of this bytes.
    /// Might be interesting e.g. for the NULL byte.
    fn has_byte(&self, b: u8) -> bool {
        self.contains(&b)
    }
}
*/