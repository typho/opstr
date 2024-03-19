use std::collections::HashMap;
use std::time::SystemTime;

use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

// NOTE: these transition probabilities have been determined from some example Lorem Ipsum text.
//       It is simply a bi-gram Markov chain. I decided to exclude any punctuation and whitespace.
const TRANSITION_PROBABILITY_A: [(u16, char); 18] = [(1411, 'm'), (297, 'd'), (204, 'b'), (250, 'g'), (385, 'a'), (296, 'l'), (1299, 't'), (457, 'c'), (181, 'r'), (385, 'k'), (431, 's'), (342, 'n'), (138, 'u'), (115, 'f'), (115, 'e'), (46, 'i'), (46, 'o'), (23, 'z')];
const TRANSITION_PROBABILITY_B: [(u16, char); 6] = [(250, 'o'), (204, 'u'), (227, 'e'), (46, 'l'), (46, 'h'), (23, 'i')];
const TRANSITION_PROBABILITY_C: [(u16, char); 7] = [(481, 'o'), (504, 'i'), (250, 'c'), (273, 'u'), (204, 'l'), (250, 't'), (46, 'e')];
const TRANSITION_PROBABILITY_D: [(u16, char); 10] = [(1140, 'o'), (844, 'i'), (386, 'd'), (273, 't'), (569, 'u'), (181, 'g'), (69, 'r'), (115, 'e'), (92, 'm'), (23, 'q')];
const TRANSITION_PROBABILITY_E: [(u16, char); 18] = [(1002, 'm'), (2093, 't'), (527, 'l'), (570, 'd'), (228, 'i'), (296, 'e'), (1049, 'r'), (204, 'o'), (593, 's'), (454, 'a'), (204, 'b'), (503, 'n'), (299, 'u'), (69, 'v'), (115, 'c'), (115, 'q'), (46, 'f'), (92, 'x')];
const TRANSITION_PROBABILITY_F: [(u16, char); 2] = [(138, 'e'), (138, 'a')];
const TRANSITION_PROBABILITY_G: [(u16, char); 6] = [(251, 'e'), (296, 'n'), (273, 'u'), (204, 'r'), (92, 'i'), (46, 'a')];
const TRANSITION_PROBABILITY_H: [(u16, char); 2] = [(115, 'e'), (23, 'i')];
const TRANSITION_PROBABILITY_I: [(u16, char); 17] = [(753, 'p'), (1233, 't'), (731, 'n'), (547, 'a'), (274, 'r'), (273, 'd'), (296, 'q'), (411, 'm'), (644, 's'), (115, 'u'), (138, 'e'), (276, 'l'), (115, 'o'), (46, 'g'), (115, 'b'), (23, 'f'), (23, 'h')];
const TRANSITION_PROBABILITY_J: [(u16, char); 1] = [(204, 'u')];
const TRANSITION_PROBABILITY_K: [(u16, char); 2] = [(204, 'a'), (204, 'i')];
const TRANSITION_PROBABILITY_L: [(u16, char); 8] = [(1504, 'o'), (1050, 'i'), (480, 'a'), (411, 'u'), (207, 'e'), (69, 'p'), (230, 'l'), (46, 'd')];
const TRANSITION_PROBABILITY_M: [(u16, char); 15] = [(548, 'i'), (548, 'd'), (818, 'e'), (228, 'n'), (251, 'y'), (366, 'o'), (273, 'p'), (569, 'a'), (273, 'v'), (273, 's'), (92, 'q'), (46, 'z'), (92, 'm'), (46, 'c'), (46, 'l')];
const TRANSITION_PROBABILITY_N: [(u16, char); 14] = [(366, 's'), (297, 'g'), (524, 'o'), (412, 'u'), (273, 'v'), (296, 't'), (273, 'a'), (181, 'n'), (273, 'c'), (69, 'h'), (138, 'd'), (46, 'e'), (345, 'i'), (23, 'k')];
const TRANSITION_PROBABILITY_O: [(u16, char); 12] = [(2050, 'r'), (1390, 'l'), (709, 'n'), (774, 'd'), (250, 'e'), (500, 's'), (46, 'o'), (69, 'b'), (69, 'm'), (46, 'c'), (23, 'p'), (23, 'v')];
const TRANSITION_PROBABILITY_P: [(u16, char); 9] = [(615, 's'), (250, 'o'), (250, 't'), (69, 'u'), (46, 'r'), (92, 'i'), (46, 'a'), (115, 'e'), (23, 'l')];
const TRANSITION_PROBABILITY_Q: [(u16, char); 1] = [(526, 'u')];
const TRANSITION_PROBABILITY_R: [(u16, char); 12] = [(1798, 'e'), (843, 's'), (205, 'm'), (434, 'i'), (388, 'a'), (296, 'o'), (204, 'g'), (46, 'u'), (69, 'c'), (69, 'p'), (69, 't'), (23, 'd')];
const TRANSITION_PROBABILITY_S: [(u16, char); 12] = [(479, 'u'), (640, 'i'), (1730, 'e'), (797, 'a'), (297, 'c'), (751, 't'), (273, 'd'), (184, 's'), (46, 'm'), (92, 'n'), (68, 'l'), (23, 'o')];
const TRANSITION_PROBABILITY_T: [(u16, char); 17] = [(1548, 'a'), (455, 'c'), (1162, 'e'), (955, 'u'), (251, 'r'), (660, 'l'), (319, 'd'), (204, 's'), (365, 'v'), (204, 'j'), (250, 'o'), (368, 'i'), (138, 'n'), (92, 'p'), (46, 'w'), (23, 'f'), (23, 'g')];
const TRANSITION_PROBABILITY_U: [(u16, char); 16] = [(1141, 'm'), (274, 'r'), (250, 'n'), (549, 't'), (204, 'y'), (227, 'p'), (342, 'a'), (704, 's'), (227, 'o'), (204, 'b'), (299, 'i'), (230, 'l'), (69, 'f'), (161, 'g'), (115, 'e'), (46, 'd')];
const TRANSITION_PROBABILITY_V: [(u16, char); 4] = [(204, 'i'), (227, 'o'), (503, 'e'), (69, 'u')];
const TRANSITION_PROBABILITY_W: [(u16, char); 1] = [(46, 'i')];
const TRANSITION_PROBABILITY_X: [(u16, char); 1] = [(92, 'e')];
const TRANSITION_PROBABILITY_Y: [(u16, char); 4] = [(182, 'e'), (204, 'a'), (46, 'n'), (23, 's')];
const TRANSITION_PROBABILITY_Z: [(u16, char); 3] = [(46, 'z'), (46, 'r'), (23, 'i')];

fn get_seed() -> u32 {
    // NOTE: I don't use an actual random number, because I don't want
    //       to include a crate because of a low-requirement random number
    let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    seed.wrapping_rem(u32::MAX as u128) as u32
}

pub struct LoremIpsum {}

impl LoremIpsum {
    pub fn generate(words_count: i64) -> String {
        let mut transition_probabilities: HashMap<char, (u32, &[(u16, char)])> = HashMap::new();

        transition_probabilities.insert('a', (6421, &TRANSITION_PROBABILITY_A));
        transition_probabilities.insert('b', (796, &TRANSITION_PROBABILITY_B));
        transition_probabilities.insert('c', (2008, &TRANSITION_PROBABILITY_C));
        transition_probabilities.insert('d', (3692, &TRANSITION_PROBABILITY_D));
        transition_probabilities.insert('e', (8459, &TRANSITION_PROBABILITY_E));
        transition_probabilities.insert('f', (276, &TRANSITION_PROBABILITY_F));
        transition_probabilities.insert('g', (1162, &TRANSITION_PROBABILITY_G));
        transition_probabilities.insert('h', (138, &TRANSITION_PROBABILITY_H));
        transition_probabilities.insert('i', (6013, &TRANSITION_PROBABILITY_I));
        transition_probabilities.insert('j', (204, &TRANSITION_PROBABILITY_J));
        transition_probabilities.insert('k', (408, &TRANSITION_PROBABILITY_K));
        transition_probabilities.insert('l', (3997, &TRANSITION_PROBABILITY_L));
        transition_probabilities.insert('m', (4469, &TRANSITION_PROBABILITY_M));
        transition_probabilities.insert('n', (3516, &TRANSITION_PROBABILITY_N));
        transition_probabilities.insert('o', (5949, &TRANSITION_PROBABILITY_O));
        transition_probabilities.insert('p', (1506, &TRANSITION_PROBABILITY_P));
        transition_probabilities.insert('q', (526, &TRANSITION_PROBABILITY_Q));
        transition_probabilities.insert('r', (4444, &TRANSITION_PROBABILITY_R));
        transition_probabilities.insert('s', (5380, &TRANSITION_PROBABILITY_S));
        transition_probabilities.insert('t', (7063, &TRANSITION_PROBABILITY_T));
        transition_probabilities.insert('u', (5042, &TRANSITION_PROBABILITY_U));
        transition_probabilities.insert('v', (1003, &TRANSITION_PROBABILITY_V));
        transition_probabilities.insert('w', (46, &TRANSITION_PROBABILITY_W));
        transition_probabilities.insert('x', (92, &TRANSITION_PROBABILITY_X));
        transition_probabilities.insert('y', (455, &TRANSITION_PROBABILITY_Y));
        transition_probabilities.insert('z', (115, &TRANSITION_PROBABILITY_Z));

        let mut result = String::new();
        result.push_str("Lorem ipsum ");

        let mut prev_char = 'l';
        let mut new_sentence_started = false;
        let mut word_length = 0; // number of Latin characters
        let mut sentence_length = 0; // number of words
        let mut paragraph_length = 0; // number of sentences
        let mut words_so_far = 0;

        while words_so_far < words_count {
            // look up probabilities
            let (total, probs) = transition_probabilities[&prev_char];
            let mut current_char = '\0';

            let rand = get_seed();
            let rand_pointer = rand % total;
            let mut accu = 0;

            // NOTE: at this point, `rand_pointer` points to exactly one array element
            for (ratio, follow_up_char) in probs {
                if accu <= rand_pointer && rand_pointer < accu + (*ratio as u32) {
                    // NOTE: yes, `rand_pointer` points to this array element
                    current_char = *follow_up_char;
                    break;
                }
                accu += *ratio as u32;
            }

            let rand = rand % 97;

            let make_new_word = (word_length >= 3) && (rand < 30);
            let make_comma_word = (word_length >= 4) && (30 <= rand && rand < 34);
            let make_new_sentence = (word_length >= 3 && sentence_length >= 5) && (40 <= rand && rand < 70);
            let make_new_paragraph = (word_length >= 3 && paragraph_length >= 3) && (90 <= rand);

            // use current_char
            if make_new_word {
                result.push(current_char);
                result.push(' ');

                new_sentence_started = false;
                word_length = 0;
                sentence_length += 1;
                //paragraph_length = paragraph_length;
                words_so_far += 1;
    
            } else if make_comma_word {
                result.push(current_char);
                result.push(',');
                result.push(' ');

                new_sentence_started = false;
                word_length = 0;
                sentence_length += 1;
                //paragraph_length = paragraph_length;
                words_so_far += 1;

            } else if make_new_sentence {
                result.push(current_char);
                result.push('.');
                result.push(' ');

                new_sentence_started = true;
                word_length = 0;
                sentence_length = 0;
                paragraph_length += 1;
                words_so_far += 1;

            } else if make_new_paragraph {
                result.push(current_char);
                result.push('.');
                result.push('\n');
                result.push('\n');

                new_sentence_started = true;
                word_length = 0;
                sentence_length = 0;
                paragraph_length = 0;
                words_so_far += 1;

            } else {
                result.push(if new_sentence_started {
                    (current_char).to_ascii_uppercase()
                } else {
                    current_char
                });

                new_sentence_started = false;
                word_length += 1;
                //sentence_length = sentence_length;
                //paragraph_length = paragraph_length;
                //words_so_far += 1;
            }

            prev_char = current_char;
        }

        if let Some(last) = result.chars().last() {
            if last != '\n' && last != '.' && last != ',' {
                if last == ' ' {
                    result.pop();
                }

                result.push('.');
            }
        }

        result
    }
}

impl traits::Op for LoremIpsum {
    fn name() -> &'static str { "lorem-ipsum" }
    fn usage() -> &'static str { "<#1 int number-of-words>" }
    fn description() -> &'static str { "generate (int #1) words of an Lorem Ipsum text" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let string: &str = args.get(0)?.try_into()?;
        Ok(match string.parse::<i64>() {
            Ok(count) => {
                if (1..=10).contains(&count) {
                    0.21
                } else {
                    0.04
                }
            },
            Err(_) => 0.0,
        })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let words: &str = args.get(0)?.try_into()?;
        match words.parse::<i64>() {
            Ok(count_words) => Ok(Self::generate(count_words).into()),
            Err(_) => Err(Errors::ArgTypeError(0, "argument must be an integer indicating the number of desired words".to_owned())),
        }
    }
}
