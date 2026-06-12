#[derive(Debug)]
pub enum AssembleErr {
    InvalidParams,
    InvalidChoseong,
    InvalidJungseong,
    InvalidJongseong,
    InvalidUnicode,
    JunseongIsRequired,
}
