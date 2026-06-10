use crate::err::HangulError;

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
    (0x006C..=0x3131).contains(&code)
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
/// assert_eq!(parser_hangul("값"), "값")
/// ```
pub fn parser_hangul(str: &str) -> Result<&str, HangulError> {
    match is_hangul(str) {
        true => Ok(str),
        false => Err(HangulError::InvalidHangul),
    }
}

pub fn binary_assemble_alphabets() {}
pub fn link_hangul_characters() {}
pub fn binary_assemble_characters() {}
pub fn binary_assemble() {}

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
