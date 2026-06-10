pub const _JASO_HANGUL_NFD: [u32; 6] = [
    0x1100, // ᄀ
    0x1161, // ᅡ
    0x11A8, // ᆨ
    0x1112, // ᄒ
    0x1175, // ᅵ
    0x11C2, // ᇂ
];

pub const COMPLETE_HANGUL_START_CHARCODE: u32 = '가' as u32;
pub const COMPLETE_HANGUL_END_CHARCODE: u32 = '힣' as u32;

pub const NUMBER_OF_JONGSEONG: usize = 28;
pub const NUMBER_OF_JUNGSEONG: usize = 21;

pub const CHOSEONGS: [char; 19] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
];

pub const JUNGSEONGS: [&str; 21] = [
    "ㅏ", "ㅐ", "ㅑ", "ㅒ", "ㅓ", "ㅔ", "ㅕ", "ㅖ", "ㅗ", "ㅗㅏ", "ㅗㅐ", "ㅗㅣ", "ㅛ", "ㅜ",
    "ㅜㅓ", "ㅜㅔ", "ㅜㅣ", "ㅠ", "ㅡ", "ㅡㅣ", "ㅣ",
];

pub const JONGSEONGS: [&str; 28] = [
    "", "ㄱ", "ㄲ", "ㄱㅅ", "ㄴ", "ㄴㅈ", "ㄴㅎ", "ㄷ", "ㄹ", "ㄹㄱ", "ㄹㅁ", "ㄹㅂ", "ㄹㅅ",
    "ㄹㅌ", "ㄹㅍ", "ㄹㅎ", "ㅁ", "ㅂ", "ㅂㅅ", "ㅅ", "ㅆ", "ㅇ", "ㅈ", "ㅊ", "ㅋ", "ㅌ", "ㅍ",
    "ㅎ",
];

pub const HANGUL_DIGITS: [&str; 20] = [
    "",
    "만",
    "억",
    "조",
    "경",
    "해",
    "자",
    "양",
    "구",
    "간",
    "정",
    "재",
    "극",
    "항하사",
    "아승기",
    "나유타",
    "불가사의",
    "무량대수",
    "겁",
    "업",
];

pub const HANGUL_DIGITS_MAX: usize = HANGUL_DIGITS.len() * 4;

pub const HANGUL_NUMBERS: [&str; 10] = ["", "일", "이", "삼", "사", "오", "육", "칠", "팔", "구"];

pub const HANGUL_NUMBERS_FOR_DECIMAL: [&str; 10] =
    ["영", "일", "이", "삼", "사", "오", "육", "칠", "팔", "구"];

pub const HANGUL_CARDINAL: [&str; 4] = ["", "십", "백", "천"];

pub fn disassemble_consonant(consonant: &str) -> &'static str {
    match consonant {
        "" => "",
        "ㄱ" => "ㄱ",
        "ㄲ" => "ㄲ",
        "ㄳ" => "ㄱㅅ",
        "ㄴ" => "ㄴ",
        "ㄵ" => "ㄴㅈ",
        "ㄶ" => "ㄴㅎ",
        "ㄷ" => "ㄷ",
        "ㄸ" => "ㄸ",
        "ㄹ" => "ㄹ",
        "ㄺ" => "ㄹㄱ",
        "ㄻ" => "ㄹㅁ",
        "ㄼ" => "ㄹㅂ",
        "ㄽ" => "ㄹㅅ",
        "ㄾ" => "ㄹㅌ",
        "ㄿ" => "ㄹㅍ",
        "ㅀ" => "ㄹㅎ",
        "ㅁ" => "ㅁ",
        "ㅂ" => "ㅂ",
        "ㅃ" => "ㅃ",
        "ㅄ" => "ㅂㅅ",
        "ㅅ" => "ㅅ",
        "ㅆ" => "ㅆ",
        "ㅇ" => "ㅇ",
        "ㅈ" => "ㅈ",
        "ㅉ" => "ㅉ",
        "ㅊ" => "ㅊ",
        "ㅋ" => "ㅋ",
        "ㅌ" => "ㅌ",
        "ㅍ" => "ㅍ",
        "ㅎ" => "ㅎ",
        _ => panic!("Invalid consonant: {consonant}"),
    }
}

pub fn disassemble_vowel(vowel: &str) -> &'static str {
    match vowel {
        "ㅏ" => "ㅏ",
        "ㅐ" => "ㅐ",
        "ㅑ" => "ㅑ",
        "ㅒ" => "ㅒ",
        "ㅓ" => "ㅓ",
        "ㅔ" => "ㅔ",
        "ㅕ" => "ㅕ",
        "ㅖ" => "ㅖ",
        "ㅗ" => "ㅗ",
        "ㅘ" => "ㅗㅏ",
        "ㅙ" => "ㅗㅐ",
        "ㅚ" => "ㅗㅣ",
        "ㅛ" => "ㅛ",
        "ㅜ" => "ㅜ",
        "ㅝ" => "ㅜㅓ",
        "ㅞ" => "ㅜㅔ",
        "ㅟ" => "ㅜㅣ",
        "ㅠ" => "ㅠ",
        "ㅡ" => "ㅡ",
        "ㅢ" => "ㅡㅣ",
        "ㅣ" => "ㅣ",
        _ => panic!("Invalid vowel: {vowel}"),
    }
}

pub fn alphabet_to_korean(c: char) -> Option<&'static str> {
    match c {
        'A' => Some("에이"),
        'B' => Some("비"),
        'C' => Some("씨"),
        'D' => Some("디"),
        'E' => Some("이"),
        'F' => Some("에프"),
        'G' => Some("지"),
        'H' => Some("에이치"),
        'I' => Some("아이"),
        'J' => Some("제이"),
        'K' => Some("케이"),
        'L' => Some("엘"),
        'M' => Some("엠"),
        'N' => Some("엔"),
        'O' => Some("오"),
        'P' => Some("피"),
        'Q' => Some("큐"),
        'R' => Some("알"),
        'S' => Some("에스"),
        'T' => Some("티"),
        'U' => Some("유"),
        'V' => Some("브이"),
        'W' => Some("더블유"),
        'X' => Some("엑스"),
        'Y' => Some("와이"),
        'Z' => Some("지"),
        _ => None,
    }
}
