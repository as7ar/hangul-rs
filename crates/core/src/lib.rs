mod assemble;
pub(crate) mod constants;
mod disassemble;
mod err;
mod hangul;
mod utils;

pub use err::HangulError;

pub use crate::hangul::{is_hangul, is_hangul_alphabet, is_hangul_char, parser_hangul};

pub use crate::constants::{
    alphabet_to_korean, disassemble_consonant, disassemble_vowel, _JASO_HANGUL_NFD, CHOSEONGS,
    COMPLETE_HANGUL_END_CHARCODE, COMPLETE_HANGUL_START_CHARCODE, HANGUL_CARDINAL, HANGUL_DIGITS,
    HANGUL_DIGITS_MAX, HANGUL_NUMBERS, HANGUL_NUMBERS_FOR_DECIMAL, JONGSEONGS, JUNGSEONGS,
    NUMBER_OF_JONGSEONG, NUMBER_OF_JUNGSEONG,
};

pub use crate::utils::{export_last_element, is_blank, join_str};
