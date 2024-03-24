use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

const ESC: char = '\x1B';

/// States for a finite state machine
///
/// 
/// ## Admissible strings
///
/// ␛ M
/// ␛ 7
/// ␛[=ℕh
/// ␛[=ℕl
/// ␛[=ℕL
/// ␛[?ℕl
/// ␛[?ℕh
/// ␛[s
/// ␛[u
/// ␛[J
/// ␛[H
/// ␛[K
/// ␛[ℕ{A,B,C,D,E,F,G,f,n,m,p,H,R,J,K}
/// ␛[ℕ;{f,m,p,H,R}
/// ␛[ℕ;"(any non-double-quoted string)"{m,p}
/// ␛[ℕ;"(any non-double-quoted string)";(continue at LOOP)
/// ␛[ℕ;ℕ{m,p,H,f,R}
/// ␛[ℕ;ℕ;(continue at LOOP)
///
/// LOOP:
///   ℕ{m,p}
///   ℕ;(continue at LOOP)
///
/// … where ℕ denotes any non-empty digit-sequence, "{x,y}" denotes
/// a set of two characters, and "(…)" describes an informal instruction.
/// Yes, ANSI escape sequences are difficult to parse.
///
/// ## Strategy
///
/// This list of admissible strings follows the strategy, that the standardized
/// sequence is admissible, but additionally the full set of integers is admissible.
/// For example, only `ESC[0J`, `ESC[1J`, `ESC[2J`, and `ESC[3J` are standardized.
/// But all sequences `ESC[ℕJ` are accepted. The “;” before the finalizing letter
/// is usually optional.
///
/// ## Encoding of the states
///
/// I encode the states based on received bytes in hexadecimal notation;
/// except for the ESC character which is encoded as `Esc` and any ℕ is encoded as `NN`.
/// So `Esc5B3DNN` represents “the characters ESC (U+001B), [ (U+005B), = (U+003D), and
/// any digit like 3 (U+0033) were read”
#[derive(Clone,Copy,Debug,Hash,PartialEq)]
enum FsmState {
    Outside, // outside an ESC sequence
    Esc, // prefix `ESC`
    Esc20, // prefix `ESC `
    Esc5B, // prefix `ESC[`
    Esc5B3D, // prefix `ESC[=`
    Esc5B3DNN, // prefix `ESC[=ℕ`
    Esc5B3F, // prefix `ESC[?`
    Esc5B3FNN, // prefix `ESC[?ℕ`
    Esc5BNN, // prefix `ESC[ℕ`
    Esc5BNN3B, // prefix `ESC[ℕ;`
    Esc5BNN3BQuotedStringStart, // prefix `ESC[ℕ;"`
    Esc5BNN3BQuotedStringEnd, // prefix `ESC[ℕ;"…"` where `…` is any string without U+0022 QUOTATION MARK
    Esc5BNN3BNN, // prefix `ESC[ℕ;ℕ`
    Esc5BNN3BNN3B, // prefix `ESC[ℕ;ℕ;`
    Esc5BNN3BNN3BQuotedStringStart, // prefix `ESC[ℕ;ℕ;"`
    Esc5BNN3BNN3BQuotedStringEnd, // prefix `ESC[ℕ;ℕ;"…"` where `…` is any string without U+0022 QUOTATION MARK
    Esc5BNN3BNN3BNN, // prefix `ESC[ℕ;ℕ;ℕ` where the final ";ℕ" can be repeated multiple times
    End,
}

impl FsmState {
    fn new() -> FsmState {
        FsmState::Outside
    }

    fn consume(&self, chr: char) -> Option<FsmState> {
        use FsmState::*;
        match self {
            Outside => match chr {
                ESC => Some(Esc),
                _ => Some(Outside),
            },
            Esc => match chr {
                '\u{0020}' => Some(Esc20), // "ESC " was read
                '\u{005B}' => Some(Esc5B), // "ESC[" was read
                _ => None,
            },
            Esc20 => match chr {
                '\u{004D}' => Some(End),
                '\u{0037}' => Some(End),
                '\u{0038}' => Some(End),
                _ => None,
            },
            Esc5B => match chr {
                '\u{0048}' => Some(End),
                '\u{0073}' => Some(End),
                '\u{0075}' => Some(End),
                '\u{004A}' => Some(End),
                '\u{004B}' => Some(End),
                '\u{003D}' => Some(Esc5B3D), // "ESC[=" was read
                '\u{003F}' => Some(Esc5B3F), // "ESC[?" was read
                '\u{0030}' ..= '\u{0039}' => Some(Esc5BNN), // "ESC[?ℕ" was read
                _ => None,
            },
            Esc5B3D => match chr {
                '\u{0030}' ..= '\u{0039}' => Some(Esc5B3DNN),
                _ => None,
            },
            Esc5B3DNN => match chr {
                '\u{0030}' ..= '\u{0039}' => Some(Esc5B3DNN),
                'h' | 'l' | 'L' => Some(End),
                _ => None,
            },
            Esc5B3F => match chr {
                '\u{0030}' ..= '\u{0039}' => Some(Esc5B3FNN),
                _ => None,
            },
            Esc5B3FNN => match chr {
                '\u{0030}' ..= '\u{0039}' => Some(Esc5B3FNN),
                'h' | 'l' => Some(End),
                _ => None,
            },
            Esc5BNN => match chr {
                '\u{0030}' ..= '\u{0039}' => Some(Esc5BNN),
                'A' ..= 'H' => Some(End),
                'J' | 'K' | 'R' | 'f' | 'm' | 'n' | 'p' => Some(End),
                ';' => Some(Esc5BNN3B),
                _ => None,
            },
            Esc5BNN3B => match chr {
                '\u{0030}' ..= '\u{0039}' => Some(Esc5BNN3BNN),
                'A' ..= 'H' => Some(End),
                'J' | 'K' | 'R' | 'f' | 'm' | 'n' | 'p' => Some(End),
                '"' => Some(Esc5BNN3BQuotedStringStart),
                _ => None,
            },
            Esc5BNN3BQuotedStringStart => match chr {
                '"' => Some(Esc5BNN3BQuotedStringEnd),
                _ => Some(Esc5BNN3BQuotedStringStart),
            },
            Esc5BNN3BQuotedStringEnd => match chr {
                ';' => Some(Esc5BNN3BQuotedStringEnd),
                'f' | 'm' | 'p' | 'H' | 'R' => Some(End),
                _ => None,
            },
            Esc5BNN3BNN => match chr {
                '\u{0030}' ..= '\u{0039}' => Some(Esc5BNN3BNN),
                'f' | 'm' | 'p' | 'H' | 'R' => Some(End),
                ';' => Some(Esc5BNN3BNN3B),
                _ => None,
            },
            Esc5BNN3BNN3B => match chr {
                '\u{0030}' ..= '\u{0039}' => Some(Esc5BNN3BNN3BNN),
                'f' | 'm' | 'p' | 'H' | 'R' => Some(End),
                '"' => Some(Esc5BNN3BNN3BQuotedStringStart),
                _ => None,
            },
            Esc5BNN3BNN3BQuotedStringStart => match chr {
                '"' => Some(Esc5BNN3BNN3BQuotedStringEnd),
                _ => Some(Esc5BNN3BNN3BQuotedStringStart),
            },
            Esc5BNN3BNN3BQuotedStringEnd => match chr {
                ';' => Some(Esc5BNN3BNN3BQuotedStringEnd),
                'm' | 'p' => Some(End),
                _ => None,
            },
            Esc5BNN3BNN3BNN => match chr {
                '\u{0030}' ..= '\u{0039}' => Some(Esc5BNN3BNN3BNN),
                'm' | 'p' => Some(End),
                ';' => Some(Esc5BNN3BNN3BNN),
                _ => None,
            },
            End => Some(Outside),
        }
    }
}

pub struct RemoveAnsiEscapeSequences {}

impl RemoveAnsiEscapeSequences {
    fn function_for_chars(src: &str) -> String {
        let mut dst = String::new();

        // main implementation based on cheatsheet by fnky
        // https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797#erase-functions
        //
        // the official specification is here:
        // https://ecma-international.org/publications-and-standards/standards/ecma-48/
        //
        // NOTE: "ESC[{code};{string};{...}p" is annoying to parse.
        //       "{U+001B}[H" can either mean "moves cursor to home position (0, 0)"
        //       or is the prefix of "{U+001B}[0;71"

        let mut state = FsmState::new();
        let mut cache = String::new();
        for chr in src.chars() {
            let new_state_or_fail = state.consume(chr);

            match new_state_or_fail {
                Some(FsmState::Outside) => {
                    dst.push(chr);
                    state = FsmState::Outside;
                },
                Some(FsmState::Esc) if state == FsmState::Outside => {
                    // initialize ESC sequence
                    cache.clear();
                    cache.push(ESC);
                    state = FsmState::Esc;
                },
                Some(FsmState::End) => {
                    // ESC sequence finished
                    // so the cache content is truncated to ignore this ESC sequence
                    cache.clear();
                    state = FsmState::new();
                },
                Some(s) => {
                    // we do not know whether this part of the ESC sequence will be used
                    // so we store it in the cache for now
                    cache.push(chr);
                    state = s;
                },
                None => {
                    // abort ESC sequence, because it is not a valid ESC sequence
                    // so push it to the destination string
                    dst.push_str(&cache);
                    dst.push(chr);

                    state = FsmState::Outside;
                },
            }
        }

        dst
    }
}

impl traits::Op for RemoveAnsiEscapeSequences {
    fn name() -> &'static str { "remove-ansi-escape-sequences" }
    fn usage() -> &'static str { "<#1 string to-simplify>" }
    fn description() -> &'static str { "remove any ANSI X3.64 (also found in ECMA-48/ISO 6429) sequences in string #1 starting with U+001B ESCAPE" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let s: &str = args.get(0)?.try_into()?;
        if s.contains(ESC) {
            Ok(0.493)
        } else {
            Ok(0.0)
        }
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let src: &str = args.get(0)?.try_into()?;
        let dst = Self::function_for_chars(src);

        Ok(dst.into())
    }
}

#[cfg(test)]
mod tests {
    use super::RemoveAnsiEscapeSequences;

    macro_rules! accept {
        ($input:expr, $output:expr) => {
            assert_eq!(RemoveAnsiEscapeSequences::function_for_chars($input), $output);
        };
    }
    macro_rules! reject {
        ($input:expr) => {
            assert_eq!(RemoveAnsiEscapeSequences::function_for_chars($input), $input);
        };
    }

    #[test]
    fn move_cursor() {
        accept!("abc \x1b[Hdef", "abc def");
        accept!("abc \x1b[3Hdef", "abc def");
        accept!("abc \x1b[3;2Hdef", "abc def");
        accept!("abc \x1b[56;0Hdef", "abc def");
        accept!("abc \x1b[3;2fdef", "abc def");
        accept!("abc \x1b[56;0Hdef", "abc def");
        accept!("cursor up\x1b[1A done", "cursor up done");
        accept!("cursor to column\x1b[42G done", "cursor to column done");
        accept!("a\x1b[42Fb", "ab");

        accept!("req pos\x1b[6n done", "req pos done");
        accept!("move cursor \x1b Mone up", "move cursor one up");
        accept!("save \x1b 7cursor", "save cursor");
        accept!("save \x1b[scursor", "save cursor");
        accept!("restore \x1b[ucursor", "restore cursor");
    }

    #[test]
    fn sequential_sequences() {
        accept!("<\x1b[3m\x1b[5mtext\x1b[0m>", "<text>");
    }

    #[test]
    fn erase() {
        accept!("dis\x1b[Jplay", "display");
        accept!("to\x1b[0JEOL", "toEOL");
        accept!("saved \x1b[3Jlines", "saved lines");
        accept!("in\x1b[Kline", "inline");
        accept!("entire\x1b[2Kline", "entireline");
    }

    #[test]
    fn colors() {
        accept!("<\x1b[0m>", "<>");
        accept!("<\x1b[8m>", "<>");
        accept!("<\x1b[30;41m>", "<>");
        accept!("<\x1b[123456789m>", "<>");
    }

    #[test]
    fn screen_mode() {
        accept!("40x25 \x1b[=0hmonochrome", "40x25 monochrome");
        accept!("40x25 \x1b[=0lmonochrome reset", "40x25 monochrome reset");
        accept!("320x200 \x1b[=4h4-color graphics", "320x200 4-color graphics");
        accept!("enable \x1b[=7hline wrapping", "enable line wrapping");
        accept!("reset \x1b[=7lline wrapping", "reset line wrapping");
        accept!("320x200 \x1b[=13hcolors", "320x200 colors");
        accept!("320x200 \x1b[=19hcolors", "320x200 colors");
        reject!("inv\x1b[=8Halid");
        reject!("inv\x1b[=20Valid");
        reject!("inv\x1b[=valid");
    }

    #[test]
    fn private_modes() {
        accept!("cursor \x1b[?25linvisible", "cursor invisible");
        accept!("restore \x1b[?47lscreen", "restore screen");
        accept!("save \x1b[?47hscreen", "save screen");
        accept!("enable \x1b[?1049lbuffer", "enable buffer");
        accept!("disable \x1b[?1049hbuffer", "disable buffer");
        reject!("<\x1b[?1048H>");
    }

    #[test]
    fn keyboard_string() {
        accept!("<\x1b[0;59p>", "<>");
        accept!("<\x1b[0;114;\"PRINT SCREEN\"p>", "<>");
        accept!("<\x1b[38;7p>", "<>");
    }
}