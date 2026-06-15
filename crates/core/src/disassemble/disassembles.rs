use std::vec;

use crate::{
    disassemble_complete_character, disassemble_consonant, disassemble_vowel, is_hangul_alphabet,
};

pub fn disassembles(str: &str) -> String {
    disassemble_to_group(str).into_iter().flatten().collect()
}

pub fn disassemble_to_group(str: &str) -> Vec<Vec<char>> {
    let letters = str.chars();
    let mut result: Vec<Vec<char>> = Vec::new();

    for l in letters {
        if l.is_whitespace() {
            result.push(vec![l]);
            continue;
        }

        if let Some(disassembled) = disassemble_complete_character(l) {
            result.push(disassembled.to_vec());
            continue;
        }

        if is_hangul_alphabet(l) {
            let c = l as u32;
            let s = l.to_string();
            match c {
                (0x3131..=0x314E) => {
                    let s = disassemble_consonant(&s);
                    result.push(s.chars().collect())
                }
                (0x314F..=0x3163) => {
                    let s = disassemble_vowel(&s);
                    result.push(s.chars().collect())
                }
                _ => result.push(vec![]),
            }
        }
    }

    result
}
