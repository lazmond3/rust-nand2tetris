use crate::bit::Bit;
use crate::not_word::not_word;
use crate::word::Word;
fn alu(
    a: Word,
    b: Word,
    a_is_zero_x: Bit,  // zx : a -> 0
    b_is_zero_x: Bit,  // zy : b -> 0
    not_a_x: Bit,      // nx : a -> !a
    not_b_x: Bit,      // ny : b -> !b
    functional_x: Bit, // f : when 0 -> add, when 1 -> and
    not_out_x: Bit,    // no : out -> !out)
) -> (Word, Bit, Bit) {
    let n_a: Word = if a_is_zero_x == Bit::I {
        Word::new_empty()
    } else if not_a_x == Bit::I {
        not_word(a)
    } else {
        a
    };
    let n_b = if b_is_zero_x == Bit::I {
        Word::new_empty()
    } else if not_b_x == Bit::I {
        not_word(b)
    } else {
        b
    };

    let mut res = if functional_x == Bit::O {
        n_a + n_b
    } else {
        n_a & n_b
    };

    res = if not_out_x == Bit::I {
        not_word(res)
    } else {
        res
    };

    let zr = Bit::from_bool(res == Word::new_empty());
    let ng = Bit::O;

    (res, zr, ng)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn for_alu_add() {
        let word_05: Word = Word::num_to_bit(5);
        let word_03: Word = Word::num_to_bit(3);

        assert_eq!(
            alu(
                word_03.clone(),
                word_05.clone(),
                Bit::O,
                Bit::O,
                Bit::O,
                Bit::O,
                Bit::O, // add
                Bit::O
            ),
            (word_03 + word_05, Bit::O, Bit::O)
        );
    }
}
