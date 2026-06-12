/// combine_vowel는 모음을 받아 하나의 곂모음으로 반환합니다.
///
/// ```rust
/// use hangul_core::combine_vowels;
///
/// assert_eq!(combine_vowels("ㅗ", "ㅏ"), "ㅘ");
/// assert_eq!(combine_vowels("ㅗ", "ㅛ"), "ㅗㅛ");
/// ```
pub fn combine_vowels(v1: &str, v2: &str) -> String {
    match format!("{}{}", v1, v2).as_str() {
        "ㅗㅏ" => "ㅘ",
        "ㅗㅐ" => "ㅙ",
        "ㅗㅣ" => "ㅚ",
        "ㅜㅓ" => "ㅝ",
        "ㅜㅔ" => "ㅞ",
        "ㅜㅣ" => "ㅟ",
        "ㅡㅣ" => "ㅢ",

        "ㅣㅏ" => "ㅑ",
        "ㅣㅓ" => "ㅕ",
        "ㅣㅗ" => "ㅛ",
        "ㅣㅔ" => "ㅖ",
        "ㅣㅜ" => "ㅠ",
        "ㅣㅐ" => "ㅒ",

        s => return s.to_string(),
    }
    .to_string()
}
