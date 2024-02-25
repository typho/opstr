use crate::auxiliary;
use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

// TODO this map is incomplete
// c.f. https://en.wikipedia.org/wiki/Unicode_subscripts_and_superscripts
// TODO "superscript" versus "superscript small cap"

// TODO read data based on https://en.wikipedia.org/wiki/Unicode_character_property


pub struct Subscript {}

impl Subscript {
    const SUBSCRIPT_MAP: &'static [(char, char)] = &[
        // digits and symbols
        ('0', '₀'),
        ('1', '₁'),
        ('2', '₂'),
        ('3', '₃'),
        ('4', '₄'),
        ('5', '₅'),
        ('6', '₆'),
        ('7', '₇'),
        ('8', '₈'),
        ('9', '₉'),
        ('+', '₊'),
        ('-', '₋'),
        ('=', '₌'),
        ('(', '₍'),
        (')', '₎'),
        // latin letters
        ('a', 'ₐ'),
        ('e', 'ₑ'),
        ('h', 'ₕ'),
        ('i', 'ᵢ'),
        ('j', 'ⱼ'),
        ('k', 'ₖ'),
        ('l', 'ₗ'),
        ('m', 'ₘ'),
        ('n', 'ₙ'),
        ('o', 'ₒ'),
        ('p', 'ₚ'),
        ('r', 'ᵣ'),
        ('s', 'ₛ'),
        ('t', 'ₜ'),
        ('u', 'ᵤ'),
        ('v', 'ᵥ'),
        ('x', 'ₓ'),
        // IPA letters
        ('ə', 'ₔ'),
        // greek letters
        ('β', 'ᵦ'),
        ('γ', 'ᵧ'),
        ('ρ', 'ᵨ'),
        ('φ', 'ᵩ'),
        ('χ', 'ᵪ')
    ];
    
    fn lookup_char(arg: char) -> Option<char> {
        for (regular, subscr) in Self::SUBSCRIPT_MAP.iter() {
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

impl traits::OpOne for Subscript {
    fn name() -> &'static str { "subscript" }
    fn description() -> &'static str { "return the subscript version of the provided string" }
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
