use crate::{
    assemble, disassembles,
    keyboard::constants::{hangul_to_qwerty, qwerty_to_hangul},
    AssembleErr,
};

/// convert_hangul_to_qwerty는 한글을 qwerty 자판과 매칭되는 영어 알파벳으로 변환합니다.
///
/// ```rust
/// use hangul_core::keyboard::convert_hangul_to_qwerty;
///
/// assert_eq!(convert_hangul_to_qwerty("겨노"), "rush".to_string());
/// ```
pub fn convert_hangul_to_qwerty(word: &str) -> String {
    let disassembled = disassembles(word);
    let mut result = String::new();
    for c in disassembled.chars() {
        let converted = hangul_to_qwerty(c).unwrap();
        result.push(converted);
    }
    result
}

/// fn convert_qwerty_to_alphabets 영어 알파벳을 qwerty 자판과 매칭되는 한글 음소로 변환합니다.
///
/// ```rust
/// use hangul_core::keyboard::convert_qwerty_to_alphabets;
///
/// assert_eq!(convert_qwerty_to_alphabets("6월 30dlf"), "6월 30ㅇㅣㄹ".to_string());
/// ```
pub fn convert_qwerty_to_alphabets(word: &str) -> String {
    let mut result = String::new();
    for c in word.chars() {
        let converted = qwerty_to_hangul(c).unwrap();
        result.push(converted);
    }
    result
}

/// convert_qwerty_to_hangul는 영어 알파벳을 qwerty 자판과 매칭과는 한글 문자와 문장으로 변환합니다.
///
/// ```rust
/// use hangul_core::keyboard::convert_qwerty_to_hangul;
///
/// assert_eq!(convert_qwerty_to_hangul("dkssudgktpdy").unwrap(), "안녕하세요".to_string());
/// ```
pub fn convert_qwerty_to_hangul(word: &str) -> Result<String, AssembleErr> {
    let converted = convert_qwerty_to_alphabets(word);
    let mut v: Vec<&str> = Vec::new();
    for c in converted.split(" ") {
        let mut other = vec![c];
        v.append(&mut other);
    }
    assemble(v)
}
