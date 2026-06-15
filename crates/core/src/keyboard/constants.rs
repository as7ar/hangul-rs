pub fn hangul_to_qwerty(c: char) -> Option<char> {
    match c {
        'ㄱ' => Some('r'),
        'ㄲ' => Some('R'),
        'ㄴ' => Some('s'),
        'ㄷ' => Some('e'),
        'ㄸ' => Some('E'),
        'ㄹ' => Some('f'),
        'ㅁ' => Some('a'),
        'ㅂ' => Some('q'),
        'ㅃ' => Some('Q'),
        'ㅅ' => Some('t'),
        'ㅆ' => Some('T'),
        'ㅇ' => Some('d'),
        'ㅈ' => Some('w'),
        'ㅉ' => Some('W'),
        'ㅊ' => Some('c'),
        'ㅋ' => Some('z'),
        'ㅌ' => Some('x'),
        'ㅍ' => Some('v'),
        'ㅎ' => Some('g'),
        'ㅏ' => Some('k'),
        'ㅐ' => Some('o'),
        'ㅑ' => Some('i'),
        'ㅒ' => Some('O'),
        'ㅓ' => Some('j'),
        'ㅔ' => Some('p'),
        'ㅕ' => Some('u'),
        'ㅖ' => Some('P'),
        'ㅗ' => Some('h'),
        'ㅛ' => Some('y'),
        'ㅜ' => Some('n'),
        'ㅠ' => Some('b'),
        'ㅡ' => Some('m'),
        'ㅣ' => Some('l'),
        el => Some(el),
    }
}

pub fn qwerty_to_hangul(c: char) -> Option<char> {
    match c {
        'q' => Some('ㅂ'),
        'Q' => Some('ㅃ'),
        'w' => Some('ㅈ'),
        'W' => Some('ㅉ'),
        'e' => Some('ㄷ'),
        'E' => Some('ㄸ'),
        'r' => Some('ㄱ'),
        'R' => Some('ㄲ'),
        't' => Some('ㅅ'),
        'T' => Some('ㅆ'),

        'y' | 'Y' => Some('ㅛ'),
        'u' | 'U' => Some('ㅕ'),
        'i' | 'I' => Some('ㅑ'),

        'o' => Some('ㅐ'),
        'O' => Some('ㅒ'),

        'p' => Some('ㅔ'),
        'P' => Some('ㅖ'),

        'a' | 'A' => Some('ㅁ'),
        's' | 'S' => Some('ㄴ'),
        'd' | 'D' => Some('ㅇ'),
        'f' | 'F' => Some('ㄹ'),
        'g' | 'G' => Some('ㅎ'),

        'h' | 'H' => Some('ㅗ'),
        'j' | 'J' => Some('ㅓ'),
        'k' | 'K' => Some('ㅏ'),
        'l' | 'L' => Some('ㅣ'),

        'z' | 'Z' => Some('ㅋ'),
        'x' | 'X' => Some('ㅌ'),
        'c' | 'C' => Some('ㅊ'),
        'v' | 'V' => Some('ㅍ'),

        'b' | 'B' => Some('ㅠ'),
        'n' | 'N' => Some('ㅜ'),
        'm' | 'M' => Some('ㅡ'),

        el => Some(el),
    }
}
