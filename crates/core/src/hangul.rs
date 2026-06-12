use crate::{
    can_be_jungseong, combine_character, combine_vowels, disassemble_complete_character,
    disassembles, err::HangulError, AssembleErr, CHOSEONGS,
};

/// `is_hangul_char`는 완전한 한글 문자를 받으면 true를 반환합니다.
///
/// ```rust
/// use hangul_core::is_hangul_char;
///
/// assert_eq!(is_hangul_char('가'), true);
/// assert_eq!(is_hangul_char('ㄱ'), false);
/// ```
pub fn is_hangul_char(character: char) -> bool {
    let code = character as u32;
    (0xAC00..=0xD7A3).contains(&code)
}

/// `is_hangul_alphabet`은 조합되지 않은 한글 문자를 받으면 true를 반환합니다.
///
/// ```rust
/// use hangul_core::is_hangul_alphabet;
///
/// assert_eq!(is_hangul_alphabet('ㄱ'), true);
/// assert_eq!(is_hangul_alphabet('가'), false);
/// ```
pub fn is_hangul_alphabet(character: char) -> bool {
    let code = character as u32;
    (0x3131..=0x314E).contains(&code) // 자음
    || (0x314F..=0x3163).contains(&code) // 모음
}

/// `is_hangul`은 한글 문자열을 받으면 true를 반환합니다.
///
/// ```rust
/// use hangul_core::is_hangul;
///
/// assert_eq!(is_hangul("가나다"), true);
/// assert_eq!(is_hangul("ㄱㅣㄴ  ㅡ"), true);
/// assert_eq!(is_hangul("abcde"), false);
/// ```
pub fn is_hangul(str: &str) -> bool {
    !str.is_empty()
        && str.chars().all(|c| {
            is_hangul_char(c)
                || is_hangul_alphabet(c)
                || c.is_whitespace() // 공백
                || c.is_ascii_punctuation() // 구두점
        })
}

/// `parser_hangul`은 한글 문자열을 받으면 그대로 반환하지만 아닌 값을 받으면 에러를 발생시킨다.
///
/// ```rust
/// use hangul_core::parser_hangul;
///
/// assert_eq!(parser_hangul("값").unwrap(), "값")
/// ```
pub fn parser_hangul(str: &str) -> Result<&str, HangulError> {
    match is_hangul(str) {
        true => Ok(str),
        false => Err(HangulError::InvalidHangul),
    }
}

/// get_choseong은 단어에서 초성을 추출합니다.
///
/// ```rust
/// use hangul_core::get_choseong;
///
/// assert_eq!(get_choseong("사과"), "ㅅㄱ")
/// assert_eq!(get_choseong("띄어 쓰기"), "ㄸㅇ ㅆㄱ")
/// ```
pub fn get_choseong(words: &str) -> String {
    let chars = words.chars();
    let mut result = String::new();

    for c in chars {
        if c.is_whitespace() {
            result.push(c);
            continue;
        }

        if CHOSEONGS.contains(&c) {
            result.push(c);
            continue;
        }

        if let Some(disassembled) = disassemble_complete_character(c) {
            result.push(disassembled.choseong);
        }
    }

    result
}

/// binary_assemble_alphabets는 두개의 한글 자모를 하나로 합칩니다.
///
/// ```rust
/// use hangul_core::binary_assemble_alphabets;
///
/// assert_eq!(binary_assemble_alphabets('ㄱ', 'ㅏ').unwrap(), "가")
/// assert_eq!(binary_assemble_alphabets('ㅗ', 'ㅏ').unwrap(), "ㅘ")
/// ```
pub fn binary_assemble_alphabets(c1: &str, c2: &str) -> Result<String, AssembleErr> {
    if can_be_jungseong(format!("{}{}", c1, c2).as_str()) {
        return Ok(combine_vowels(c1, c2));
    }

    if !can_be_jungseong(c1) && can_be_jungseong(c2) {
        return combine_character(c1, c2, None);
    }

    Ok(format!("{}{}", c1, c2))
}

pub fn link_hangul_characters(c1: &str, c2: &str) -> Result<String, AssembleErr> {
    let c1 = disassembles(c1);
    let c2 = disassembles(c2);

    Ok(String::new())
}
pub fn binary_assemble_characters(c1: &str, c2: &str) {}
pub fn binary_assemble(c1: &str, c2: &str) {}

#[cfg(test)]
mod test {
    use crate::{is_hangul, is_hangul_alphabet, is_hangul_char};

    #[test]
    fn is_hangul_test() {
        assert_eq!(is_hangul_char('가'), true);
        assert_eq!(is_hangul_alphabet('ㄱ'), true);
        assert_eq!(is_hangul("ㄱㄴㄷ, 아버지 가방에 들어가신다."), true);
    }
}
