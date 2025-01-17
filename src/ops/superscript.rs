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
        ('χ', 'ᵡ'),
        // Latin Extended-D
        ('ꝯ', 'ꝰ'),
        ('C', 'ꟲ'),
        ('F', 'ꟳ'),
        ('Q', 'ꟴ'),
        // Latin Extended-E
        ('ʍ', 'ꭩ'),
        // Latin Extended-F
        ('ˑ', '𐞂'),
        ('æ', '𐞃'),
        ('ʙ', '𐞄'),
        ('ɓ', '𐞅'),
        ('ʣ', '𐞇'),
        ('ꭦ', '𐞈'),
        ('ʥ', '𐞉'),
        ('ʤ', '𐞊'),
        ('ɖ', '𐞋'),
        ('ɗ', '𐞌'),
        ('ᶑ', '𐞍'),
        ('ɘ', '𐞎'),
        ('ɞ', '𐞏'),
        ('ʩ', '𐞐'),
        ('ɤ', '𐞑'),
        ('ɢ', '𐞒'),
        ('ɠ', '𐞓'),
        ('ʛ', '𐞔'),
        ('ħ', '𐞕'),
        ('ʜ', '𐞖'),
        ('ɧ', '𐞗'),
        ('ʄ', '𐞘'),
        ('ʪ', '𐞙'),
        ('ʫ', '𐞚'),
        ('ɬ', '𐞛'),
        ('𝼄', '𐞜'),
        ('ꞎ', '𐞝'),
        ('ɮ', '𐞞'),
        ('𝼅', '𐞟'),
        ('ʎ', '𐞠'),
        ('𝼆', '𐞡'),
        ('ø', '𐞢'),
        ('ɶ', '𐞣'),
        ('ɷ', '𐞤'),
        ('q', '𐞥'),
        ('ɺ', '𐞦'),
        ('𝼈', '𐞧'),
        ('ɽ', '𐞨'),
        ('ɾ', '𐞩'),
        ('ʀ', '𐞪'),
        ('ʨ', '𐞫'),
        ('ʦ', '𐞬'),
        ('ꭧ', '𐞭'),
        ('ʧ', '𐞮'),
        ('ʈ', '𐞯'),
        ('ⱱ', '𐞰'),
        ('ʏ', '𐞲'),
        ('ʡ', '𐞳'),
        ('ʢ', '𐞴'),
        ('ʘ', '𐞵'),
        ('ǀ', '𐞶'),
        ('ǁ', '𐞷'),
        ('ǂ', '𐞸'),
        ('𝼊', '𐞹'),
        ('𝼞', '𐞺'),
        // Spacing Modifier Letters
        ('h', 'ʰ'),
        ('ɦ', 'ʱ'),
        ('j', 'ʲ'),
        ('r', 'ʳ'),
        ('ɹ', 'ʴ'),
        ('ɻ', 'ʵ'),
        ('ʀ', 'ʶ'),
        ('w', 'ʷ'),
        ('y', 'ʸ'),
        ('ʔ', 'ˀ'),
        ('ʕ', 'ˁ'),
        ('ɣ', 'ˠ'),
        ('l', 'ˡ'),
        ('s', 'ˢ'),
        ('x', 'ˣ'),
        // Phonetic Extensions
        ('A', 'ᴬ'),
        ('Æ', 'ᴭ'),
        ('B', 'ᴮ'),
        ('ᴃ', 'ᴯ'),
        ('D', 'ᴰ'),
        ('E', 'ᴱ'),
        ('Ǝ', 'ᴲ'),
        ('G', 'ᴳ'),
        ('H', 'ᴴ'),
        ('I', 'ᴵ'),
        ('J', 'ᴶ'),
        ('K', 'ᴷ'),
        ('L', 'ᴸ'),
        ('M', 'ᴹ'),
        ('N', 'ᴺ'),
        ('ᴎ', 'ᴻ'),
        ('O', 'ᴼ'),
        ('Ȣ', 'ᴽ'),
        ('P', 'ᴾ'),
        ('R', 'ᴿ'),
        ('T', 'ᵀ'),
        ('U', 'ᵁ'),
        ('W', 'ᵂ'),
        ('a', 'ᵃ'),
        ('ɐ', 'ᵄ'),
        ('ɑ', 'ᵅ'),
        ('ᴂ', 'ᵆ'),
        ('b', 'ᵇ'),
        ('d', 'ᵈ'),
        ('e', 'ᵉ'),
        ('ə', 'ᵊ'),
        ('ɛ', 'ᵋ'),
        ('ᴈ', 'ᵌ'),
        ('g', 'ᵍ'),
        ('k', 'ᵏ'),
        ('m', 'ᵐ'),
        ('ŋ', 'ᵑ'),
        ('o', 'ᵒ'),
        ('ɔ', 'ᵓ'),
        ('p', 'ᵖ'),
        ('t', 'ᵗ'),
        ('u', 'ᵘ'),
        ('ɯ', 'ᵚ'),
        ('v', 'ᵛ'),
        ('ꞵ', 'ᵝ'),
        ('γ', 'ᵞ'),
        ('ẟ', 'ᵟ'),
        ('φ', 'ᵠ'),
        ('ꭓ', 'ᵡ'),
        ('Н', 'ᵸ'),
        ('ᴉ', 'ᵎ'),
        ('ᴖ', 'ᵔ'),
        ('ᴗ', 'ᵕ'),
        ('ᴝ', 'ᵙ'),
        ('ﻌ', 'ᵜ'),
        // Phonetic Extensions Supplement
        ('ɒ', 'ᶛ'),
        ('c', 'ᶜ'),
        ('ɕ', 'ᶝ'),
        ('ð', 'ᶞ'),
        ('ɜ', 'ᶟ'),
        ('f', 'ᶠ'),
        ('ɟ', 'ᶡ'),
        ('ɡ', 'ᶢ'),
        ('ɥ', 'ᶣ'),
        ('ɨ', 'ᶤ'),
        ('ɩ', 'ᶥ'),
        ('ɪ', 'ᶦ'),
        ('ᵻ', 'ᶧ'),
        ('ʝ', 'ᶨ'),
        ('ɭ', 'ᶩ'),
        ('ᶅ', 'ᶪ'),
        ('ʟ', 'ᶫ'),
        ('ɱ', 'ᶬ'),
        ('ɰ', 'ᶭ'),
        ('ɲ', 'ᶮ'),
        ('ɳ', 'ᶯ'),
        ('ɴ', 'ᶰ'),
        ('ɵ', 'ᶱ'),
        ('ɸ', 'ᶲ'),
        ('ʂ', 'ᶳ'),
        ('ʃ', 'ᶴ'),
        ('ƫ', 'ᶵ'),
        ('ʉ', 'ᶶ'),
        ('ʊ', 'ᶷ'),
        ('ᴜ', 'ᶸ'),
        ('ʋ', 'ᶹ'),
        ('ʌ', 'ᶺ'),
        ('z', 'ᶻ'),
        ('ʐ', 'ᶼ'),
        ('ʑ', 'ᶽ'),
        ('ʒ', 'ᶾ'),
        ('θ', 'ᶿ'),
        // Cyrillic Extended-B
        ('ъ', 'ꚜ'),
        ('ь', 'ꚝ'),
        // Cyrillic Extended-D
        ('а', '𞀰'),
        ('б', '𞀱'),
        ('в', '𞀲'),
        ('г', '𞀳'),
        ('з', '𞀷'),
        ('е', '𞀵'),
        ('ж', '𞀶'),
        ('з', '𞀷'),
        ('и', '𞀸'),
        ('к', '𞀹'),
        ('л', '𞀺'),
        ('м', '𞀻'),
        ('о', '𞀼'),
        ('п', '𞀽'),
        ('р', '𞀾'),
        ('с', '𞀿'),
        ('т', '𞁀'),
        ('у', '𞁁'),
        ('ф', '𞁂'),
        ('х', '𞁃'),
        ('ц', '𞁄'),
        ('ч', '𞁅'),
        ('ш', '𞁆'),
        ('ы', '𞁇'),
        ('э', '𞁈'),
        ('ю', '𞁉'),
        ('ꚉ', '𞁊'),
        ('ә', '𞁋'),
        ('і', '𞁌'),
        ('ј', '𞁍'),
        ('ө', '𞁎'),
        ('ү', '𞁏'),
        ('ӏ', '𞁐'),
        ('ҫ', '𞁫'),
        ('ꙑ', '𞁬'),
        ('ұ', '𞁭'),
        // Georgian
        ('ნ', 'ჼ'),
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
