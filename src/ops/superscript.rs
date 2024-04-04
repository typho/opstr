use crate::auxiliary;
use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

// TODO this map is incomplete
// c.f. https://en.wikipedia.org/wiki/Unicode_subscripts_and_superscripts
// TODO "superscript" versus "superscript small cap"

// TODO read data based on https://en.wikipedia.org/wiki/Unicode_character_property


pub struct Superscript {}

impl Superscript {
    const SUPERSCRIPT_MAP: &'static [(char, char)] = &[
        // digits and symbols
        ('+', '⁺'),
        ('-', '⁻'),
        ('=', '⁼'),
        ('(', '⁽'),
        (')', '⁾'),
        ('0', '⁰'),
        ('1', '¹'),
        ('2', '²'),
        ('3', '³'),
        ('4', '⁴'),
        ('5', '⁵'),
        ('6', '⁶'),
        ('7', '⁷'),
        ('8', '⁸'),
        ('9', '⁹'),
        // latin letters
        ('A', 'ᴬ'),
        ('B', 'ᴮ'),
        ('D', 'ᴰ'),
        ('E', 'ᴱ'),
        ('G', 'ᴳ'),
        ('H', 'ᴴ'),
        ('I', 'ᴵ'),
        ('J', 'ᴶ'),
        ('K', 'ᴷ'),
        ('L', 'ᴸ'),
        ('M', 'ᴹ'),
        ('N', 'ᴺ'),
        ('O', 'ᴼ'),
        ('P', 'ᴾ'),
        ('R', 'ᴿ'),
        ('T', 'ᵀ'),
        ('U', 'ᵁ'),
        ('V', 'ⱽ'),
        ('W', 'ᵂ'),
        ('a', 'ᵃ'),
        ('b', 'ᵇ'),
        ('c', 'ᶜ'),
        ('d', 'ᵈ'),
        ('e', 'ᵉ'),
        ('f', 'ᶠ'),
        ('g', 'ᵍ'),
        ('h', 'ʰ'),
        ('i', 'ⁱ'),
        ('j', 'ʲ'),
        ('k', 'ᵏ'),
        ('l', 'ˡ'),
        ('m', 'ᵐ'),
        ('n', 'ⁿ'),
        ('o', 'ᵒ'),
        ('p', 'ᵖ'),
        ('r', 'ʳ'),
        ('s', 'ˢ'),
        ('t', 'ᵗ'),
        ('u', 'ᵘ'),
        ('v', 'ᵛ'),
        ('w', 'ʷ'),
        ('x', 'ˣ'),
        ('y', 'ʸ'),
        ('z', 'ᶻ'),
        // greek letters
        ('β', 'ᵝ'),
        ('γ', 'ᵞ'),
        ('δ', 'ᵟ'),
        ('ε', 'ᵋ'),
        ('θ', 'ᶿ'),
        ('ι', 'ᶥ'),
        ('υ', 'ᶹ'),
        ('φ', 'ᵠ'),
        ('χ', 'ᵡ')
    ];

    fn lookup_char(arg: char) -> Option<char> {
        for (regular, subscr) in Self::SUPERSCRIPT_MAP.iter() {
            if arg == *regular {
                return Some(*subscr);
            }
        }
        None
    }
    
    fn apply_replacements(text: &str) -> String {
        let mut subscript_string = String::new();
    
        for chr in text.chars() {
            if chr.is_whitespace() {
                subscript_string.push(chr);
                continue;
            }
    
            subscript_string.push(match Self::lookup_char(chr) {
                Some(subscript_char) => subscript_char,
                None => chr,
            });
        }
    
        subscript_string
    }
}

impl traits::Op for Superscript {
    fn name() -> &'static str { "superscript" }
    fn usage() -> &'static str { "<#1 string to-convert>" }
    fn description() -> &'static str { "return the superscript version of the provided string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let text: &str = args.get(0)?.try_into()?;
        let sub: &str = &Self::apply_replacements(text);
        let diff = auxiliary::count_different_codepoints_of_shorter_string(text, sub);

        Ok(match diff {
            0 => 0.0,
            1 => 0.69,
            _ => 1. / diff as f32,
        })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let text: &str = args.get(0)?.try_into()?;
        Ok(Self::apply_replacements(text).into())
    }
}
