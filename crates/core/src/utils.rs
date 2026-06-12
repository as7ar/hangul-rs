use crate::{
    combine_character, disassemble_to_group, disassembles, is_hangul_char, CHOSEONGS,
    COMPLETE_HANGUL_START_CHARCODE, JONGSEONGS, JUNGSEONGS, NUMBER_OF_JONGSEONG,
};

pub fn export_last_element<'a>(array: &'a [&'a str]) -> (&'a [&'a str], &'a str) {
    let (e, arr) = array.split_last().unwrap();
    (arr, e)
}

pub fn join_str(args: &[&str]) -> String {
    args.join("")
}

pub fn is_blank(str: &str) -> bool {
    str.chars().all(|c| c.is_whitespace())
}

pub fn defined<T>(value: Option<T>) -> T {
    value.expect("Value is undefined")
}

/// can_be_choseong은 인자가 초성으로 쓰일 수 있다면 true를 반환합니다.
///
/// ```rust
/// use hangul_core::can_be_choseong;
///
/// assert_eq!(can_be_choseong("ㄱ"), true);
/// ```
pub fn can_be_choseong(letter: &str) -> bool {
    letter.chars().all(|c| CHOSEONGS.contains(&c))
}

/// can_be_jungseong은 인자가 중성으로 쓰일 수 있다면 true를 반환합니다.
///
/// ```rust
/// use hangul_core::can_be_jungseong;
///
/// assert_eq!(can_be_jungseong("ㄱ"), false);
/// assert_eq!(can_be_jungseong("ㅏ"), true);
/// assert_eq!(can_be_jungseong("ㅘ"), true);
/// assert_eq!(can_be_jungseong("ㅗㅏ"), true);
/// ```
pub fn can_be_jungseong(letter: &str) -> bool {
    let letter = disassembles(letter);
    let str: &str = letter.as_str();
    JUNGSEONGS.contains(&str)
}

/// can_be_jongseong은 인자가 중성으로 쓰일 수 있다면 true를 반환합니다.
///
/// ```rust
/// use hangul_core::can_be_jongseong;
///
/// assert_eq!(can_be_jongseong("ㄱ"), true);
/// assert_eq!(can_be_jongseong("ㅏ"), false);
/// assert_eq!(can_be_jongseong("ㅘ"), false);
/// assert_eq!(can_be_jongseong("ㄺ"), true);
/// assert_eq!(can_be_jongseong("ㄹㄱ"), true);
/// ```
pub fn can_be_jongseong(letter: &str) -> bool {
    let letter = disassembles(letter);
    let str = letter.as_str();
    JONGSEONGS.contains(&str)
}

/// remove_last_character는 주어진 한글 문자열에서 가장 마지막 문자 하나를제거해 반환합니다.
///
/// ```rust
/// use hangul_core::remove_last_character;
///
/// assert_eq!(remove_last_character("안녕"), "안녀".to_string());
/// assert_eq!(remove_last_character("안녕, 세상"), "안녕, 세사".to_string());
/// ```
pub fn remove_last_character(words: &str) -> String {
    let Some(last_char) = words.chars().last() else {
        return String::new();
    };

    let mut prefix = words.to_string();
    prefix.pop();
    let mut jamos = disassemble_to_group(&last_char.to_string());

    if jamos.is_empty() {
        return prefix;
    }

    let group = &mut jamos[0];

    if group.is_empty() {
        return prefix;
    }

    group.pop();

    let rebuilt = match group.len() {
        0 => String::new(),
        1 => group[0].to_string(),
        2 => combine_character(
            group[0].to_string().as_str(),
            group[1].to_string().as_str(),
            None,
        )
        .unwrap(),
        3 => {
            if can_be_jungseong(group[2].to_string().as_str()) {
                combine_character(
                    group[0].to_string().as_str(),
                    &format!("{}{}", group[1], group[2]),
                    None,
                )
                .unwrap()
            } else {
                combine_character(
                    group[0].to_string().as_str(),
                    group[1].to_string().as_str(),
                    Some(group[2].to_string().as_str()),
                )
                .unwrap()
            }
        }
        4 => combine_character(
            group[0].to_string().as_str(),
            format!("{}{}", group[1], group[2]).as_str(),
            Some(group[3].to_string().as_str()),
        )
        .unwrap(),

        _ => unreachable!(),
    };

    prefix.push_str(&rebuilt);
    prefix
}

pub enum NumOfBatchim {
    SINGLE,
    DOUBLE,
}

/// has_batchim은 한글 문자열 마지막에 받침이 있는 지 확인합니다.
///
/// ```rust
/// use hangul_core::{has_batchim, NumOfBatchim};
///
/// assert_eq!(has_batchim("값", NumOfBatchim::SINGLE), false);
/// assert_eq!(has_batchim("값", NumOfBatchim::DOUBLE), true);
/// assert_eq!(has_batchim("가", NumOfBatchim::SINGLE), false);
/// ```
pub fn has_batchim(str: &str, num: NumOfBatchim) -> bool {
    let chars = str.chars();
    let last_char = chars.last().unwrap();
    let char_code = last_char as u32;

    let is_not_complete_hangul = !is_hangul_char(last_char);
    if is_not_complete_hangul {
        return false;
    }

    let batchim_code =
        ((char_code - COMPLETE_HANGUL_START_CHARCODE) as usize) % NUMBER_OF_JONGSEONG;
    let batchim_length = JONGSEONGS.get(batchim_code).iter().len();

    return match num {
        NumOfBatchim::SINGLE => batchim_length == 1,
        NumOfBatchim::DOUBLE => batchim_length == 2,
        _ => batchim_length > 0,
    };
}
