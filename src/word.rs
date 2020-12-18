use crate::bit::Bit;
use std::default::Default;
use std::ops::{Index, IndexMut};

const BIT_WIDTH: usize = 16;

#[derive(Default, IndexMut)]
pub struct Word([Bit; BIT_WIDTH]);

impl Index<usize> for Word {
    type Output = Bit;

    fn index(&self, index: usize) -> &Self::Output {
        if (BIT_WIDTH - 1) < index {
            panic!(format!("index fail: {} is out of range.", index))
        }
        &self.0[index]
    }
}

impl IndexMut<usize> for Word {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if (BIT_WIDTH - 1) < index {
            panic!(format!("index fail: {} is out of range.", index))
        }
        self.0.index_mut(index)
    }
}

fn one_bit_word(index: usize) -> Word {
    if (BIT_WIDTH - 1) < index {
        panic!(format!("index fail: {} is out of range.", index))
    }
    let ar: Word = Default::default();
    ar[index] = Bit::I;
    ar
}

#[cfg(test)]
mod tests {
    use super::Bit;
    use super::Bit::{I, O};
    use super::Word;

    #[test]
    fn for_word() {
        let bit_array_01: [Bit; 16] = [I, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O];
        let bit_array_15: [Bit; 16] = [O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, I];
        let word_01 = Word(bit_array_01);
        let word_15 = Word(bit_array_15);
        assert_eq!(word_01[0], I);
        assert_eq!(word_01[1], O);
        assert_eq!(word_15[0], O);
        assert_eq!(word_15[14], O);
        assert_eq!(word_15[15], I);
    }
}
