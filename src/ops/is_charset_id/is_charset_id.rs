// based on data retrieved on 2022-04-18
//  URL: https://www.iana.org/assignments/character-sets/character-sets-1.csv
//  MD5sum: 5afe8dbdee2e83301f2b756467a2c8d4

use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

use crate::ops::is_charset_id::CD;
use crate::ops::is_charset_id::CharsetDataEntry;
use crate::ops::is_charset_id::CHARSET_DATA;

pub struct IsCharsetID {}

impl IsCharsetID {
    fn lookup(name: &str) -> Option<CharsetDataEntry> {
        if !name.is_ascii() {
            return None;
        }

        for entry in CD.iter() {
            if entry.name == name || entry.names.contains(&name) {
                return Some(entry.into());
            }
        }

        None
    }

    fn lookup__(name: &str) -> Option<CharsetDataEntry> {
        // TODO deprecate and remove?
        if !name.is_ascii() {
            return None;
        }
    
        // TODO: windows() is so wrong here?! Did I write this?
        let name_ascii = name.as_bytes();
        match CHARSET_DATA.windows(name_ascii.len()).position(|window| window == name_ascii) {
            None => None,
            Some(pos) => {
                // find start of record (i.e. previous \x1D)
                let mut start = pos;
                while CHARSET_DATA[start] != b'\x1D' {
                    start -= 1;
                }
                start += 1;
    
                // our entry to be filled
                let mut data = CharsetDataEntry::default();
    
                // find preferred_name (i.e. … until next \x1E)
                let mut end = start;
                while CHARSET_DATA[end] != b'\x1E' {
                    end += 1;
                }
                data.preferred_name = String::from_utf8_lossy(&CHARSET_DATA[start..end]).to_string();
    
                // find name (i.e. … until next \x1E)
                end += 1;
                start = end;
                while CHARSET_DATA[end] != b'\x1E' {
                    end += 1;
                }
                data.name = String::from_utf8_lossy(&CHARSET_DATA[start..end]).to_string();
    
                // find names (i.e. split by \x1F until end of record \x1D)
                while CHARSET_DATA[end] != b'\x1D' {
                    end += 1;
                    start = end;
                    while CHARSET_DATA[end] != b'\x1F' && CHARSET_DATA[end] != b'\x1D' {
                        end += 1;
                    }
                    data.names.push(String::from_utf8_lossy(&CHARSET_DATA[start..end]).to_string());
                }
    
                Some(data)
            }
        }
    }
}

impl traits::Op for IsCharsetID {
    fn name() -> &'static str { "is-charset-id" }
    fn usage() -> &'static str { "<#1 string to-encode>" }
    fn description() -> &'static str { "is the given name argument #1 a valid charset identifier?" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, LibError> {
        let candidate: &str = args.get(0)?.try_into()?;

        Ok(match Self::lookup(candidate) {
            Some(data) => {
                let common_name = if !data.preferred_name.is_empty() { data.preferred_name } else { data.name };
                if candidate.to_lowercase() == common_name.to_lowercase() {
                    0.957
                } else {
                    0.897
                }
            },
            None => 0.18,
        })
    }

    fn run(args: &Args) -> Result<Output, LibError> {
        let candidate: &str = args.get(0)?.try_into()?;

        match Self::lookup(candidate) {
            Some(data) => {
                let common_name = if !data.preferred_name.is_empty() { data.preferred_name } else { data.name };

                if candidate.to_lowercase() == common_name.to_lowercase() {
                    Ok(true.into())
                } else {
                    eprintln!("Its preferred name is '{}'", common_name);
                    Ok(true.into())
                }
            },
            None => Ok(false.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! s {
        ($s:expr) => {
            $s.split_whitespace().map(|w| String::from(w)).collect::<Vec<String>>()
        };
    }

    #[test]
    fn test_static() {
        assert_eq!(
            IsCharsetID::lookup("CCSID00858"),
            Some(CharsetDataEntry {
                preferred_name: "".to_string(),
                name: "IBM00858".to_string(),
                names: s!("CCSID00858 CP00858 PC-Multilingual-850+euro csIBM00858")
            })
        );
    }
}