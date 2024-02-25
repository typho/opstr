use crate::auxiliary;
use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

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
    
    fn lookup_arg(arg: &StrArg) -> String {
        let text: &str = arg.into();
        let mut score = 0.5f32;
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

impl traits::OpOne for Superscript {
    fn name() -> &'static str { "superscript" }
    fn description() -> &'static str { "return the superscript version of the provided string" }
    fn priority(arg: &StrArg) -> f32 {
        let original: &str = arg.into();
        let sub: &str = &Self::lookup_arg(arg);
        let diff = auxiliary::count_different_codepoints_of_shorter_string(original, sub);

        match diff {
            0 => 0.0,
            1 => 0.69,
            _ => 1. / diff as f32,
        }
    }
    fn run(arg: &StrArg) -> Result<Output, Errors> {
        Ok(Self::lookup_arg(arg).into())
    }
}
