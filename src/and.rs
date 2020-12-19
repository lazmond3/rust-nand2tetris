use crate::bit::Bit;
use crate::nand::nand;
use crate::not::not;
use crate::util::convert_vec_to_word;
use crate::word::Word;

fn and(a: Bit, b: Bit) -> Bit {
    not(nand(a, b))
}

fn and_word(a: Word, b: Word) -> Word {
    let na = a
        .to_vec()
        .iter()
        .zip(b.to_vec().iter())
        .map(|(va, vb)| match *va {
            Bit::O => Bit::O,
            Bit::I => {
                if *vb == Bit::I {
                    Bit::I
                } else {
                    Bit::O
                }
            }
        })
        .collect();
    convert_vec_to_word(na)
}

#[cfg(test)]
mod tests {
    use super::Bit::{I, O};
    use super::*;

    #[test]
    fn for_and() {
        assert_eq!(and(O, O), O);
        assert_eq!(and(O, I), O);
        assert_eq!(and(I, O), O);
        assert_eq!(and(I, I), I);
    }

    fn for_nand() {
        let word00 = Word::bit_position(5);
    }
}
