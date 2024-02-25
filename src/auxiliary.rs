/// Return a number between 0 (completely different) and 1 (equal) to indicate string similarity
pub(crate) fn string_similarity(ref_string: &str, provided_string: &str) -> f32 {
    // determine the longer length
    let max_len = usize::max(ref_string.len(), provided_string.len());

    // determine the number of different codepoints (plus the ones only existing in the longer one)
    let mut count_diff = count_different_codepoints_of_shorter_string(ref_string, provided_string);
    count_diff += ref_string.len().abs_diff(provided_string.len());

    // a number between 0 and 1 describing similarity linearly
    let linear_ratio = 1. - (count_diff as f32 / max_len as f32);

    // A number between 0 and 1 describing similarity but with its square.
    // So a small difference in a long string is closer to 1 than in the linear case.
    linear_ratio.sqrt()
}

pub(crate) fn count_different_codepoints_of_shorter_string(first: &str, second: &str) -> usize {
    let mut count = 0;
    for (f, s) in first.chars().zip(second.chars()) {
        if f != s {
            count += 1;
        }
    }
    count
}

//fn normalize_name (name: &str) -> String {
//    name.replace("-", "").replace("_", "").replace(" ", "").to_string()
//}

const UNICODE_DATA: &str = include_str!("../data/UnicodeData.txt");
pub const UNICODEPOINT_UNKNOWN: &str = "<unknown codepoint>";


pub(crate) fn unicode_codepoint_names_lookup(codepoint: &[char]) -> Vec<Option<&'static str>> {
    let mut result = vec![];
    for cp in codepoint.iter() {
        // NOTE: 0000 does not have a newline in front - special handling required!
        if *cp == '\0' {
            result.push(Some(&UNICODE_DATA[5..14]));
            continue;
        }

        // generic lookup
        let search_item = format!("\n{:04X};", *cp as u32);
        for (pos, found) in UNICODE_DATA.match_indices(&search_item) {
            let name_excerpt: &'static str = &UNICODE_DATA[pos + found.len()..pos + found.len() + 200];
            result.push(name_excerpt.find(';').map_or(None, |idx| Some(&name_excerpt[..idx])));
        }
    }

    result
}

pub(crate) fn unicode_name_to_codepoint(name: &str) -> Option<char> {
    let mut result = None;
    let unicode_name = name.trim();

    if name.chars().all(|c| c.is_uppercase() || c.is_numeric() || c.is_whitespace()) {
        for (pos, _) in UNICODE_DATA.match_indices(&format!(";{};", unicode_name)) {
            let mut line_start = pos;
            while !&UNICODE_DATA[line_start..].starts_with('\n') {
                if line_start == 0 { break }
                line_start -= 1;
            }

            // Unicode names also occur in the 11-th field, but unlike field 2
            // they do not pass this check
            if pos - line_start > 10 {
                continue;
            }

            let slice = &UNICODE_DATA[line_start + 1 .. pos];
            result = match u32::from_str_radix(slice, 16) {
                Ok(s) => char::from_u32(s),
                Err(_) => continue,
            };
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unicode_codepoint_names_lookup() {
        let samples = ['\0', 'A'];
        let names = unicode_codepoint_names_lookup(&samples);

        assert_eq!(names[0], Some("<control>"));
        assert_eq!(names[1], Some("LATIN CAPITAL LETTER A"));
    }

    #[test]
    fn test_unicode_name_to_codepoint() {
        assert_eq!(Some('A'), unicode_name_to_codepoint("LATIN CAPITAL LETTER A"));
    }
}