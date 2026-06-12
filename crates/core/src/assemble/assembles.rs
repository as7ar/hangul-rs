use crate::{binary_assemble, disassembles, AssembleErr};

pub fn assemble(frag: Vec<&str>) -> Result<String, AssembleErr> {
    let words = frag.join(" ");
    let disassembled = disassembles(&words);
    let mut chars = disassembled.chars();
    let first = chars.next().ok_or(AssembleErr::InvalidHangul);

    chars.try_fold(first.unwrap().to_string(), |acc, c| {
        binary_assemble(&acc, &c.to_string())
    })
}
