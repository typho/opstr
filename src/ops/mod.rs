use crate::Configuration;

macro_rules! spec {
    ($s:ident) => { ($s::name, $s::description, $s::usage, $s::acceptable_number_of_arguments, $s::priority, $s::run) };
}

/// operations
mod uppercase_for_ascii;
mod base64_decode;
mod base64_url_safe_decode;
mod base64_encode;
mod base64_url_safe_encode;
mod camelcase_for_ascii;
mod center;
mod codepoints_ustring_list;
mod codepoints;
mod codepoint_frequency;
mod codepoint_lookup;
mod codepoint_names;
mod combining_codepoint_list;
mod concatenate;
mod count_codepoints;
mod count_grapheme_clusters;
mod count_substring;
mod count_utf16_bytes;
mod count_utf8_bytes;
mod dedent;
mod dedent_with_substring;
mod format;
mod grapheme_clusters;
mod guarantee_prefix;
mod guarantee_suffix;
mod human_readable_bytes;
mod indent_with_substring;
mod is_ascii;
mod is_caseinsensitively_equal;
mod is_charset_id;
mod is_contained;
mod is_crlf_lineterminated;
mod is_empty;
mod is_equal;
mod is_lf_lineterminated;
mod is_prefix;
mod is_suffix;
mod is_whitespace;
mod is_whitespace_agnostically_equal;
mod join;
mod length_minimum;
mod length_maximum;
mod levensthein_distance;
mod linebreak_before;
mod lines_shortened;
mod lorem_ipsum;
mod lowercase_for_ascii;
mod normalize_with_nfc;
mod normalize_with_nfd;
mod normalize_with_nfkd;
mod normalize_with_nfkc;
mod remove_ansi_escape_sequences;
mod repeat;
mod replace;
mod sentence_clusters;
mod skip_prefix;
mod skip_suffix;
mod similarity;
mod sort;
mod split;
mod split_by_whitespaces;
mod split_by_whitespaces_limited_at_end;
mod split_by_whitespaces_limited_at_start;
mod strip_codepoints;
mod strip_codepoints_at_end;
mod strip_codepoints_at_start;
mod strip_whitespaces;
mod strip_whitespaces_at_start;
mod strip_whitespaces_at_end;
mod strikethrough;
mod subscript;
mod substring_byte_indices;
mod superscript;
mod text_to_emoji;
mod utf8_bytes;
mod utf16_big_endian_bytes;
mod utf16_little_endian_bytes;
mod word_clusters;
mod xml_decode;
mod xml_encode;

#[cfg(feature = "digest")]
mod digest_md5;
#[cfg(feature = "digest")]
mod digest_sha1;
#[cfg(feature = "digest")]
mod digest_sha256;
#[cfg(feature = "digest")]
mod digest_sha3;

pub(crate) use base64_decode::Base64Decode;
pub(crate) use base64_url_safe_decode::Base64UrlSafeDecode;
pub(crate) use base64_encode::Base64Encode;
pub(crate) use base64_url_safe_encode::Base64UrlSafeEncode;
pub(crate) use camelcase_for_ascii::CamelcaseForAscii;
pub(crate) use center::Center;
pub(crate) use codepoints_ustring_list::CodepointsUstringList;
pub(crate) use codepoints::Codepoints;
pub(crate) use codepoint_frequency::CodepointFrequency;
pub(crate) use codepoint_lookup::CodepointLookup;
pub(crate) use codepoint_names::CodepointNames;
pub(crate) use concatenate::Concatenate;
pub(crate) use count_codepoints::CountCodepoints;
pub(crate) use count_grapheme_clusters::CountGraphemeClusters;
pub(crate) use count_substring::CountSubstring;
pub(crate) use count_utf16_bytes::CountUtf16Bytes;
pub(crate) use count_utf8_bytes::CountUtf8Bytes;
pub(crate) use dedent::Dedent;
pub(crate) use dedent_with_substring::DedentWithSubstring;
pub(crate) use format::Format;
pub(crate) use grapheme_clusters::GraphemeClusters;
pub(crate) use guarantee_suffix::GuaranteeSuffix;
pub(crate) use guarantee_prefix::GuaranteePrefix;
pub(crate) use human_readable_bytes::HumanReadableBytes;
pub(crate) use indent_with_substring::IndentWithSubstring;
pub(crate) use is_ascii::IsAscii;
pub(crate) use is_caseinsensitively_equal::IsCaseinsensitivelyEqual;
pub(crate) use is_charset_id::IsCharsetID;
pub(crate) use is_contained::IsContained;
pub(crate) use is_crlf_lineterminated::IsCRLFLineTerminated;
pub(crate) use is_empty::IsEmpty;
pub(crate) use is_equal::IsEqual;
pub(crate) use is_lf_lineterminated::IsLFLineTerminated;
pub(crate) use is_prefix::IsPrefix;
pub(crate) use is_suffix::IsSuffix;
pub(crate) use is_whitespace::IsWhitespace;
pub(crate) use is_whitespace_agnostically_equal::IsWhitespaceAgnosticallyEqual;
pub(crate) use join::Join;
pub(crate) use length_minimum::LengthMinimum;
pub(crate) use length_maximum::LengthMaximum;
pub(crate) use levensthein_distance::LevenstheinDistance;
pub(crate) use linebreak_before::LinebreakBefore;
pub(crate) use lines_shortened::LinesShortened;
pub(crate) use lorem_ipsum::LoremIpsum;
pub(crate) use lowercase_for_ascii::LowercaseForAscii;
pub(crate) use normalize_with_nfc::NormalizeWithNFC;
pub(crate) use normalize_with_nfd::NormalizeWithNFD;
pub(crate) use normalize_with_nfkc::NormalizeWithNFKC;
pub(crate) use normalize_with_nfkd::NormalizeWithNFKD;
pub(crate) use remove_ansi_escape_sequences::RemoveAnsiEscapeSequences;
pub(crate) use repeat::Repeat;
pub(crate) use replace::Replace;
pub(crate) use sentence_clusters::SentenceClusters;
pub(crate) use similarity::Similarity;
pub(crate) use skip_prefix::SkipPrefix;
pub(crate) use skip_suffix::SkipSuffix;
pub(crate) use sort::Sort;
pub(crate) use split::Split;
pub(crate) use split_by_whitespaces::SplitByWhitespaces;
pub(crate) use split_by_whitespaces_limited_at_start::SplitByWhitespacesLimitedAtStart;
pub(crate) use split_by_whitespaces_limited_at_end::SplitByWhitespacesLimitedAtEnd;
pub(crate) use strikethrough::StrikeThrough;
pub(crate) use strip_codepoints::StripCodepoints;
pub(crate) use strip_codepoints_at_end::StripCodepointsAtEnd;
pub(crate) use strip_codepoints_at_start::StripCodepointsAtStart;
pub(crate) use strip_whitespaces::StripWhitespaces;
pub(crate) use strip_whitespaces_at_start::StripWhitespacesAtStart;
pub(crate) use strip_whitespaces_at_end::StripWhitespacesAtEnd;
pub(crate) use subscript::Subscript;
pub(crate) use substring_byte_indices::SubstringByteIndices;
pub(crate) use superscript::Superscript;
pub(crate) use text_to_emoji::TextToEmoji;
pub(crate) use uppercase_for_ascii::UppercaseForAscii;
pub(crate) use utf8_bytes::Utf8Bytes;
pub(crate) use utf16_little_endian_bytes::Utf16LittleEndianBytes;
pub(crate) use utf16_big_endian_bytes::Utf16BigEndianBytes;
pub(crate) use word_clusters::WordClusters;
pub(crate) use xml_decode::XmlDecode;
pub(crate) use xml_encode::XmlEncode;

#[cfg(feature = "digest")]
pub(crate) use digest_md5::DigestMd5;
#[cfg(feature = "digest")]
pub(crate) use digest_sha1::DigestSha1;
#[cfg(feature = "digest")]
pub(crate) use digest_sha256::DigestSha256;
#[cfg(feature = "digest")]
pub(crate) use digest_sha3::DigestSha3256;



/// operation index
pub(crate) mod traits;
use crate::errors;
use crate::input;
use crate::output;
use crate::range;

use self::combining_codepoint_list::CombiningCodepointList;
use self::traits::Op;

type FnName = fn () -> &'static str;
type FnDesc = fn () -> &'static str;
type FnUse = fn () -> &'static str;
type FnNum = fn () -> range::Range;
type FnPriority = fn (args: &input::Args, conf: &Configuration) -> Result<f32, errors::LibError>;
type Fn = fn (args: &input::Args, conf: &Configuration) -> Result<output::Output, errors::LibError>;

pub(crate) const INDEX: &[(FnName, FnDesc, FnUse, FnNum, FnPriority, Fn)] = &[
    spec!(Base64Decode),
    spec!(Base64UrlSafeDecode),
    spec!(Base64Encode),
    spec!(Base64UrlSafeEncode),
    spec!(CamelcaseForAscii),
    spec!(Center),
    spec!(CodepointsUstringList),
    spec!(CodepointFrequency),
    spec!(CodepointLookup),
    spec!(CodepointNames),
    spec!(Codepoints),
    spec!(CombiningCodepointList),
    spec!(Concatenate),
    spec!(CountCodepoints),
    spec!(CountGraphemeClusters),
    spec!(CountSubstring),
    spec!(CountUtf8Bytes),
    spec!(CountUtf16Bytes),
    spec!(Dedent),
    spec!(DedentWithSubstring),
    spec!(Format),
    spec!(GraphemeClusters),
    spec!(GuaranteePrefix),
    spec!(GuaranteeSuffix),
    spec!(HumanReadableBytes),
    spec!(IndentWithSubstring),
    spec!(IsAscii),
    spec!(IsCaseinsensitivelyEqual),
    spec!(IsContained),
    spec!(IsCRLFLineTerminated),
    spec!(IsCharsetID),
    spec!(IsEmpty),
    spec!(IsEqual),
    spec!(IsLFLineTerminated),
    spec!(IsPrefix),
    spec!(IsSuffix),
    spec!(IsWhitespace),
    spec!(IsWhitespaceAgnosticallyEqual),
    spec!(Join),
    spec!(LengthMinimum),
    spec!(LengthMaximum),
    spec!(LevenstheinDistance),
    spec!(LinebreakBefore),
    spec!(LinesShortened),
    spec!(LoremIpsum),
    spec!(LowercaseForAscii),
    spec!(NormalizeWithNFC),
    spec!(NormalizeWithNFD),
    spec!(NormalizeWithNFKC),
    spec!(NormalizeWithNFKD),
    spec!(RemoveAnsiEscapeSequences),
    spec!(Repeat),
    spec!(Replace),
    spec!(SentenceClusters),
    spec!(Similarity),
    spec!(SkipPrefix),
    spec!(SkipSuffix),
    spec!(Sort),
    spec!(Split),
    spec!(SplitByWhitespaces),
    spec!(SplitByWhitespacesLimitedAtStart),
    spec!(SplitByWhitespacesLimitedAtEnd),
    spec!(StrikeThrough),
    spec!(StripCodepoints),
    spec!(StripCodepointsAtEnd),
    spec!(StripCodepointsAtStart),
    spec!(StripWhitespaces),
    spec!(StripWhitespacesAtStart),
    spec!(StripWhitespacesAtEnd),
    spec!(Subscript),
    spec!(SubstringByteIndices),
    spec!(Superscript),
    spec!(TextToEmoji),
    spec!(UppercaseForAscii),
    spec!(Utf8Bytes),
    spec!(Utf16LittleEndianBytes),
    spec!(Utf16BigEndianBytes),
    spec!(WordClusters),
    spec!(XmlDecode),
    spec!(XmlEncode),
    #[cfg(feature = "digest")]
    spec!(DigestMd5),
    #[cfg(feature = "digest")]
    spec!(DigestSha1),
    #[cfg(feature = "digest")]
    spec!(DigestSha256),
    #[cfg(feature = "digest")]
    spec!(DigestSha3256),
];
