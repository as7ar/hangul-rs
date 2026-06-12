use crate::{
    can_be_choseong, can_be_jongseong, can_be_jungseong, combine_character, combine_vowels,
    disassemble_complete_character, disassemble_to_group, err::HangulError, has_batchim,
    remove_last_character, AssembleErr, CHOSEONGS,
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
/// assert_eq!(parser_hangul("값").unwrap(), "값");
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
/// assert_eq!(get_choseong("사과"), "ㅅㄱ");
/// assert_eq!(get_choseong("띄어 쓰기"), "ㄸㅇ ㅆㄱ");
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
/// assert_eq!(binary_assemble_alphabets("ㄱ", "ㅏ").unwrap(), "가");
/// assert_eq!(binary_assemble_alphabets("ㅗ", "ㅏ").unwrap(), "ㅘ");
/// ```
pub fn binary_assemble_alphabets(c1: &str, c2: &str) -> Result<String, AssembleErr> {
    if can_be_jungseong(&format!("{}{}", c1, c2)) {
        return Ok(combine_vowels(c1, c2));
    }

    if !can_be_jungseong(c1) && can_be_jungseong(c2) {
        return combine_character(c1, c2, None);
    }

    Ok(format!("{}{}", c1, c2))
}

/// link_hangul_characters는 연음 법칙을 적용하여 두 개의 한글 문자를 연결합니다.
pub fn link_hangul_characters(source: &str, next_character: &str) -> Result<String, AssembleErr> {
    let groups = disassemble_to_group(source);
    let source_jamo = groups.last().ok_or(AssembleErr::InvalidHangul)?;
    let binding = source_jamo
        .last()
        .ok_or(AssembleErr::InvalidHangul)?
        .to_string();
    let last_jamo = binding.as_str();
    let left = remove_last_character(source);
    let right = combine_character(last_jamo, next_character, None)?;

    Ok(format!("{left}{right}"))
}

/// binary_assemble_characters는 인자로 받은 한글 2개를 합성합니다.
///
/// ```rust
/// use crate::binary_assemble_characters;
///
/// assert_eq!(binary_assemble_characters('ㄱ', 'ㅏ').unwrap(), "가".to_string());
/// assert_eq!(binary_assemble_characters('갑', 'ㅅ').unwrap(), "값".to_string());
/// ```
pub fn binary_assemble_characters(c1: char, c2: char) -> Result<String, AssembleErr> {
    if !is_hangul_char(c1) && !is_hangul_alphabet(c1) {
        return Err(AssembleErr::InvalidHangul);
    }

    if !is_hangul_alphabet(c2) {
        return Err(AssembleErr::InvalidHangul);
    }

    let s1 = c1.to_string();
    let s2 = c2.to_string();

    let s1_group = disassemble_to_group(&s1);

    let c1_jamo = s1_group.first().unwrap();
    if c1_jamo.len() == 1 {
        let c1_character = c1_jamo.get(0).unwrap();
        return binary_assemble_alphabets(&c1_character.to_string(), &c2.to_string());
    }

    let last_jamo = c1_jamo.last().ok_or(AssembleErr::InvalidHangul)?;
    let rest_jamos = &c1_jamo[..c1_jamo.len() - 1];

    let secondary_last_jamo = if rest_jamos.len() >= 2 {
        Some(rest_jamos[rest_jamos.len() - 1])
    } else {
        None
    };

    let _need_linking = can_be_choseong(&last_jamo.to_string()) && can_be_jungseong(&s2);
    if _need_linking {
        return link_hangul_characters(&s1, &s2);
    }
    let choseong = rest_jamos[0].to_string();

    let combined_jungseong = format!("{last_jamo}{c2}");

    if can_be_jungseong(&combined_jungseong) {
        return combine_character(&choseong, &combined_jungseong, None);
    }

    if let Some(secondary_last_jamo) = secondary_last_jamo {
        let merged_vowel = format!("{secondary_last_jamo}{last_jamo}");

        if can_be_jungseong(&merged_vowel) && can_be_jongseong(&s2) {
            return combine_character(&choseong, &merged_vowel, Some(&s2));
        }
    }

    let last_jamo_str = last_jamo.to_string();

    if can_be_jungseong(&last_jamo_str) && can_be_jongseong(&s2) {
        return combine_character(&choseong, &last_jamo_str, Some(&s2));
    }

    let vowel = if rest_jamos.len() >= 3 {
        let merged = format!("{}{}", rest_jamos[1], rest_jamos[2]);

        if can_be_jungseong(&merged) {
            merged
        } else {
            rest_jamos[1].to_string()
        }
    } else {
        rest_jamos[1].to_string()
    };

    let merged_jongseong = format!("{last_jamo}{c2}");

    if has_batchim(&s1, crate::NumOfBatchim::SINGLE) && can_be_jongseong(&merged_jongseong) {
        return combine_character(&choseong, &vowel, Some(&merged_jongseong));
    }

    Ok(format!("{c1}{c2}"))
}

/// binary_assemble은 인자로 받은 한글 문장과 한글 문자 하나를 합쳐 반환합니다.
///
/// ```rust
/// use hangul_core::binary_assemble;
///
/// assert_eq!(binary_assemble("사고","ㅏ").unwrap(), "사과".to_string());
/// assert_eq!(binary_assemble("저는 고양이를 좋아합닏","ㅏ").unwrap(), "저는 고양이를 좋아합니다".to_string());
/// ```
pub fn binary_assemble(c1: &str, c2: &str) -> Result<String, AssembleErr> {
    let mut chars = c1.chars();
    let last = chars.next_back().ok_or(AssembleErr::InvalidHangul)?;
    let rest: String = chars.collect();
    let next = c2.chars().next().ok_or(AssembleErr::InvalidHangul)?;
    let need_join = last.is_whitespace() || next.is_whitespace();

    let result = if need_join {
        format!("{last}{next}")
    } else {
        binary_assemble_characters(last, next)?
    };

    Ok(format!("{rest}{result}"))
}

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
