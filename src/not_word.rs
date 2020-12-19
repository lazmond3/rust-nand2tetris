use crate::bit::Bit;
use crate::constant::BIT_WIDTH;
use crate::not::not;
use crate::util::convert_vec_to_word;
use crate::word::Word;

use std::convert::TryInto;

pub fn not_word(a: Word) -> Word {
    let b = a
        .internal()
        .to_vec()
        .iter()
        .map(|x| not((*x).clone()))
        .collect();

    convert_vec_to_word(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_not_word() {
        let word_00: Word = Word::bit_position(0);
        let not_word_00: Word = not_word(word_00.clone());

        assert_eq!(
            word_00
                .internal()
                .to_vec()
                .iter()
                .zip(not_word_00.internal().to_vec().iter())
                .all(|(a, b)| *a != *b),
            true
        );
    }
}
