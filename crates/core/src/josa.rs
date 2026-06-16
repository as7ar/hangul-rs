use crate::{disassemble_complete_character, has_batchim};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Josa {
    IGa,
    EulReul,
    EunNeun,
    EuroRo,
    WaGwa,
    InaNa,
    IranRan,
    Aya,
    IrangRang,
    IEyoYeyo,
    EuroseoRoseo,
    EuroSseoRoSseo,
    EurobuteoRobuteo,
    IraRa,
}

fn pick_josa(word: &str, josa: Josa) -> &'static str {
    if word.is_empty() {
        return match josa {
            Josa::IGa => "이",
            Josa::EulReul => "을",
            Josa::EunNeun => "은",
            Josa::EuroRo => "으로",
            Josa::WaGwa => "과",
            Josa::InaNa => "이나",
            Josa::IranRan => "이란",
            Josa::Aya => "아",
            Josa::IrangRang => "이랑",
            Josa::IEyoYeyo => "이에요",
            Josa::EuroseoRoseo => "으로서",
            Josa::EuroSseoRoSseo => "으로써",
            Josa::EurobuteoRobuteo => "으로부터",
            Josa::IraRa => "이라",
        };
    }

    let has_batchim = has_batchim(word, crate::NumOfBatchim::ANY);

    let mut first = has_batchim;

    let is_rieul = word
        .chars()
        .last()
        .and_then(disassemble_complete_character)
        .map(|c| c.jongseong == "ㄹ")
        .unwrap_or(false);

    let is_ro_case = has_batchim
        && is_rieul
        && matches!(
            josa,
            Josa::EuroRo | Josa::EuroseoRoseo | Josa::EuroSseoRoSseo | Josa::EurobuteoRobuteo
        );

    if matches!(josa, Josa::WaGwa) || is_ro_case {
        first = !first;
    }

    match (josa, first) {
        (Josa::IGa, true) => "이",
        (Josa::IGa, false) => "가",

        (Josa::EulReul, true) => "을",
        (Josa::EulReul, false) => "를",

        (Josa::EunNeun, true) => "은",
        (Josa::EunNeun, false) => "는",

        (Josa::EuroRo, true) => "으로",
        (Josa::EuroRo, false) => "로",

        (Josa::WaGwa, true) => "과",
        (Josa::WaGwa, false) => "와",

        (Josa::InaNa, true) => "이나",
        (Josa::InaNa, false) => "나",

        (Josa::IranRan, true) => "이란",
        (Josa::IranRan, false) => "란",

        (Josa::Aya, true) => "아",
        (Josa::Aya, false) => "야",

        (Josa::IrangRang, true) => "이랑",
        (Josa::IrangRang, false) => "랑",

        (Josa::IEyoYeyo, true) => "이에요",
        (Josa::IEyoYeyo, false) => "예요",

        (Josa::EuroseoRoseo, true) => "으로서",
        (Josa::EuroseoRoseo, false) => "로서",

        (Josa::EuroSseoRoSseo, true) => "으로써",
        (Josa::EuroSseoRoSseo, false) => "로써",

        (Josa::EurobuteoRobuteo, true) => "으로부터",
        (Josa::EurobuteoRobuteo, false) => "로부터",

        (Josa::IraRa, true) => "이라",
        (Josa::IraRa, false) => "라",
    }
}

pub fn josa(word: &str, josa: Josa) -> String {
    format!("{word}{}", pick_josa(word, josa))
}
