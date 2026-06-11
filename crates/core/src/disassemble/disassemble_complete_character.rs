use std::fmt;

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

impl ReturnTypeDisassembleCompleteCharacter {
    pub fn as_jamo(&self) -> String {
        format!("{}{}{}", self.choseong, self.jungseong, self.jongseong)
    }

    pub fn to_vec(&self) -> Vec<char> {
        self.as_jamo().chars().collect()
    }
}

impl fmt::Display for ReturnTypeDisassembleCompleteCharacter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ choseong: \'{}\', jungseong: \'{}\', jongseong: \'{}\' }}",
            self.choseong, self.jungseong, self.jongseong
        )
    }
}

///  `disassemble_complete_character`는 완전한 한글 문자열을 초성 중성 종성으로 분리합니다.
///
/// ```rust
/// use hangul_core::disassemble_complete_character;
///
/// println!("값: {}", disassemble_complete_character('값').unwrap())
/// ```
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

#[cfg(test)]
mod test {
    use crate::disassemble_complete_character;

    #[test]
    fn test_disassemble() {
        let result = disassemble_complete_character('값').unwrap();

        assert_eq!(result.choseong, 'ㄱ');
        assert_eq!(result.jungseong, "ㅏ");
        assert_eq!(result.jongseong, "ㅂㅅ");
    }
}
