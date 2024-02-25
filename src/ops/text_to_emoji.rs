use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct TextToEmoji {}

const EMOJI_DATA: &'static [u8] = include_bytes!("text_to_emoji_data/emoji-data.bin");

impl TextToEmoji {
    fn function_for_str(arg: &str, arg_id: usize) -> Result<String, Errors> {
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
                Err(_) => return Err(Errors::InvalidData("Emoji data contains internal error".to_owned())),
            };
            let emoji_description = match String::from_utf8(data[1].to_vec()) {
                Ok(emoji) => emoji,
                Err(_) => return Err(Errors::InvalidData("Emoji data descriptions contain internal error".to_owned())),
            };

            if emoji_description == argument {
                return Ok(utf8_encoded_emoji);
            }
        }

        Err(Errors::ArgValueError(arg_id, format!("Unknown emoji identifier: '{}'", arg)))
    }
}

impl traits::OpOne for TextToEmoji {
    fn name() -> &'static str { "text-to-emoji" }
    fn description() -> &'static str { "given a Emoji Sequence Data (UTS #51) description return the corresponding emoji (e.g. 'smiling face with halo' returns '😇')" }

    fn priority(arg: &StrArg) -> f32 {
        if Self::function_for_str(arg.into(), 0).is_ok() {
            1.0
        } else {
            0.87
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        Ok(Self::function_for_str(arg.into(), 0)?.into())
    }
}
