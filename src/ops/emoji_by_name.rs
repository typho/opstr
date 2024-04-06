use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct EmojiByName {}

const EMOJI_DATA: &'static [u8] = include_bytes!("../../data/emoji_data/emoji-data.bin");

impl EmojiByName {
    fn function_for_chars(arg: &str, arg_id: usize) -> Result<String, LibError> {
        let argument = arg.to_ascii_lowercase();

        // separator := U+001E RECORD SEPARATOR
        for row in EMOJI_DATA.split(|byte| *byte == 0x1E) {
            if row.is_empty() {
                continue;
            }

            // separator := U+001F UNIT SEPARATOR
            let data = row.split(|byte| *byte == 0x1F).collect::<Vec<&[u8]>>();

            let utf8_encoded_emoji = match String::from_utf8(data[0].to_vec()) {
                Ok(emoji) => emoji,
                Err(_) => return Err(LibError::InvalidData("Emoji data contains internal error".to_owned())),
            };
            let emoji_description = match String::from_utf8(data[1].to_vec()) {
                Ok(emoji) => emoji,
                Err(_) => return Err(LibError::InvalidData("Emoji data descriptions contain internal error".to_owned())),
            };

            if emoji_description == argument {
                return Ok(utf8_encoded_emoji);
            }
        }

        Err(LibError::ArgValueError(arg_id, format!("Unknown emoji identifier: '{}'", arg)))
    }
}

impl traits::Op for EmojiByName {
    fn name() -> &'static str { "emoji-by-name" }
    fn usage() -> &'static str { "<#1 string emoji-description>" }
    fn description() -> &'static str { "given a Emoji Sequence Data (UTS #51) description string #1 return the corresponding emoji (e.g. 'smiling face with halo' returns '😇')" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let name: &str = args.get(0)?.try_into()?;
        Ok(if Self::function_for_chars(name, 0).is_ok() {
            1.0
        } else {
            0.0
        })
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let name: &str = args.get(0)?.try_into()?;
        Ok(Self::function_for_chars(name, 0)?.into())
    }
}
