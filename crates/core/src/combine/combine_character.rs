use crate::{
    can_be_choseong, can_be_jongseong, can_be_jungseong, disassembles, AssembleErr, CHOSEONGS,
    COMPLETE_HANGUL_START_CHARCODE, JONGSEONGS, JUNGSEONGS, NUMBER_OF_JONGSEONG,
    NUMBER_OF_JUNGSEONG,
};

/// combine_character는 초성, 중성, 종성을 받아 하나의 한글 문자로 반환합니다.
///
/// ```rust
/// use hangul_core::combine_character;
///
/// assert_eq!(combine_character("ㄱ", "ㅏ", Some("ㅂㅅ")).unwrap(), "값");
/// assert_eq!(combine_character("ㄱ", "ㅏ", Some("ㅄ")).unwrap(), "값");
/// assert_eq!(combine_character("ㄱ", "ㅏ", None).unwrap(), "가");
/// ```
pub fn combine_character(
    choseong: &str,
    jungseong: &str,
    jongcheong: Option<&str>,
) -> Result<String, AssembleErr> {
    let jongseong = jongcheong.unwrap_or("");

    if !can_be_choseong(choseong) || !can_be_jungseong(jungseong) || !can_be_jongseong(jongseong) {
        return Err(AssembleErr::InvalidParams);
    }

    let cho_index = CHOSEONGS
        .iter()
        .position(|&c| c.to_string() == choseong.to_string())
        .ok_or(AssembleErr::InvalidChoseong)?;
    let jung_index = JUNGSEONGS
        .iter()
        .position(|&c| c.to_string() == disassembles(jungseong))
        .ok_or(AssembleErr::InvalidJungseong)?;
    let jong_index = JONGSEONGS
        .iter()
        .position(|&c| c.to_string() == disassembles(jongseong))
        .ok_or(AssembleErr::InvalidJongseong)?;

    let choseong_of_target_consonant =
        (cho_index * NUMBER_OF_JONGSEONG * NUMBER_OF_JUNGSEONG) as u32;
    let choseong_of_target_vowel = (jung_index * NUMBER_OF_JONGSEONG) as u32;

    let unicode = COMPLETE_HANGUL_START_CHARCODE
        + choseong_of_target_consonant
        + choseong_of_target_vowel
        + (jong_index as u32);

    let result = char::from_u32(unicode).ok_or(AssembleErr::InvalidUnicode)?;

    Ok(result.to_string())
}
