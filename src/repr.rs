// TODO: remove?


pub struct CLIConfig {
    spaces_indentation: u8,
}

impl Default for CLIConfig {
    fn default() -> Self {
        Self { spaces_indentation: 2 }
    }
}

pub trait CLIRepresentation {
    fn to_str(&self, conf: &CLIConfig, indent_level: u8) -> String;
}

pub enum Response {
    ShortSingleLine(String),
    LongSingleLine(String),
    DefinitionList(Vec<(String, String)>),
    Array(Vec<String>),
    Set(Vec<String>),
    Table(Vec<Vec<String>>),
    ComplexText(Vec<Response>),
}

impl CLIRepresentation for Response {
    fn to_str(&self, conf: &CLIConfig, indent_level: u8) -> String {
        let indentation = " ".repeat((conf.spaces_indentation * indent_level) as usize);
        let indent = |t| {
            let mut s = indentation.clone();
            s.push_str(t);
            s
        };
        match &self {
            Response::ShortSingleLine(s) => indent(s),
            Response::LongSingleLine(s) => indent(s),
            Response::DefinitionList(dl) => {
                let mut max_key_width = 0;
                for (key, _) in dl {
                    let width = key.chars().count();
                    if width > max_key_width {
                        max_key_width = width;
                    }
                }

                let mut out = String::new();
                for (key, val) in dl {
                    let width = key.chars().count();
                    out.push_str(&" ".repeat((conf.spaces_indentation * indent_level) as usize));
                    out.push_str(key);
                    out.push_str("::");
                    out.push_str(&" ".repeat(max_key_width - width));
                    out.push_str(val);
                    out.push('\n');
                }

                out
            },
            Response::Array(arr) => {
                let mut out = String::new();
                for val in arr {
                    out.push_str(&" ".repeat((conf.spaces_indentation * indent_level) as usize));
                    out.push_str("• ");
                    out.push_str(val);
                    out.push('\n');
                }
                out
            },
            Response::Set(set) => {
                let mut out = String::new();
                for val in set {
                    out.push_str(&" ".repeat((conf.spaces_indentation * indent_level) as usize));
                    out.push_str("• ");
                    out.push_str(val);
                    out.push('\n');
                }
                out
            },
            Response::Table(tab) => {
                if tab.is_empty() {
                    return String::new();
                }

                let (rows, columns) = (tab.len(), tab[0].len());
                let mut column_widths = Vec::new();

                for colno in 0..columns {
                    column_widths.push(tab[0][colno].chars().count());
                }

                for row in tab.iter().take(rows) {
                    for (colno, cell) in row.iter().take(columns).enumerate() {
                        let given_width = cell.chars().count();
                        if column_widths[colno] < given_width {
                            column_widths[colno] = given_width;
                        }
                    }
                }

                // header line
                let mut header_line = String::new();
                header_line.push_str(&indentation);
                for w in column_widths {
                    header_line.push_str(&"–".repeat(w));
                    header_line.push(' ');
                }
                header_line.push(' ');

                // generate table
                let mut out = String::new();
                out.push_str(&header_line);
                for row in tab.iter().take(rows) {
                    out.push_str(&indentation);
                    for cell in row.iter().take(columns) {
                        out.push_str(cell);
                        out.push(' ');
                    }
                    out.push('\n');
                }
                out.push_str(&header_line);

                out
            },
            Response::ComplexText(ct) => {
                let mut out = String::new();
                for entry in ct {
                    out.push_str(&entry.to_str(conf, indent_level));
                }
                out
            },
        }

    }
}
