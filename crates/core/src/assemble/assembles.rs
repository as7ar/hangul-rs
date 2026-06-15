use crate::{binary_assemble, disassembles, AssembleErr};

/// assemble은 인자로 받은 배열에 담긴 한글 문장과 문자를 한글 규칙에 맞게 합성합니다.
///
/// ```rust
/// use hangul_core::assemble;
///
/// assert_eq!(assemble(vec!["아버지", "가방에", "들어가신다"]).unwrap(), "아버지 가방에 들어가신다".to_string());
/// ```
pub fn assemble(frag: Vec<&str>) -> Result<String, AssembleErr> {
    let words = frag.join(" ");
    let disassembled = disassembles(&words);
    let mut chars = disassembled.chars();
    let first = chars.next().ok_or(AssembleErr::InvalidHangul);

    chars.try_fold(first.unwrap().to_string(), |acc, c| {
        binary_assemble(&acc, &c.to_string())
    })
}
