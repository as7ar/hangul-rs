use std::fmt;

#[derive(Debug)]
pub enum HangulError {
    InvalidHangul,
}

#[derive(Debug)]
pub enum NumberToHangulError {
    EmptyInput,
    InvalidNumber,
    UnitRangeExceeded,
}

impl fmt::Display for NumberToHangulError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => write!(f, "입력이 비어 있습니다."),
            Self::InvalidNumber => write!(f, "유효하지 않은 숫자입니다."),
            Self::UnitRangeExceeded => write!(f, "한글 숫자 단위 범위를 초과했습니다."),
        }
    }
}
