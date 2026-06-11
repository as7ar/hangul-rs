use crate::{
    is_hangul_char, CHOSEONGS, COMPLETE_HANGUL_START_CHARCODE, JONGSEONGS, JUNGSEONGS,
    NUMBER_OF_JONGSEONG, NUMBER_OF_JUNGSEONG,
};

#[derive(Debug, Clone, Copy)]
pub struct ReturnTypeDisassembleCompleteCharacter {
    pub choseong: char,
    pub jungseong: &'static str,
    pub jongseong: &'static str,
}

pub fn disassemble_complete_character(
    letter: char,
) -> Option<ReturnTypeDisassembleCompleteCharacter> {
    let char_code = letter as u32;

    let is_complete_hangul = is_hangul_char(letter);

    if !is_complete_hangul {
        return None;
    }

    let hangul_code = (char_code - COMPLETE_HANGUL_START_CHARCODE) as usize;

    let jongseong_index = hangul_code % NUMBER_OF_JONGSEONG;
    let jungseong_index =
        ((hangul_code - jongseong_index) / NUMBER_OF_JONGSEONG) % NUMBER_OF_JUNGSEONG;
    let choseong_index =
        (hangul_code - jongseong_index) / NUMBER_OF_JONGSEONG / NUMBER_OF_JUNGSEONG;

    Some(ReturnTypeDisassembleCompleteCharacter {
        choseong: CHOSEONGS[choseong_index],
        jungseong: JUNGSEONGS[jungseong_index],
        jongseong: JONGSEONGS[jongseong_index],
    })
}
