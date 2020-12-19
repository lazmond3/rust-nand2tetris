use crate::bit::Bit;
use crate::constant::BIT_WIDTH;
use std::convert::{TryFrom, TryInto};
use std::default::Default;
use std::ops::{Index, IndexMut};

use std::cmp::PartialEq;
use std::convert::Infallible;

pub type InternalWord = [Bit; BIT_WIDTH];

#[derive(Default, Debug, Clone)]
pub struct Word(InternalWord);

impl Word {
    pub fn internal(&self) -> InternalWord {
        (*self).clone().0
    }
    pub fn new(a: InternalWord) -> Word {
        Word(a)
    }
}

impl PartialEq for Word {
    fn eq(&self, other: &Self) -> bool {
        (*self)
            .internal()
            .to_vec()
            .iter()
            .zip(other.internal().to_vec().iter())
            .all(|(m, o)| *m == *o)
    }
}

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

pub fn one_bit_word(index: usize) -> Word {
    if (BIT_WIDTH - 1) < index {
        panic!(format!("index fail: {} is out of range.", index))
    }
    let mut ar: Word = Default::default();
    ar[index] = Bit::I;
    ar
}

#[cfg(test)]
mod tests {
    use super::one_bit_word;
    use super::Bit::{I, O};
    use super::Word;
    use super::{Bit, BIT_WIDTH};

    #[test]
    fn for_word() {
        let word_00: Word = one_bit_word(0);
        let word_15: Word = one_bit_word(BIT_WIDTH - 1);
        for i in 1..BIT_WIDTH {
            let word_00_val = word_00[i].clone();
            let word_15_val = word_15[i].clone();
            if i == 0 {
                assert_eq!(word_00_val, I);
            } else {
                assert_eq!(word_00_val, O);
            }

            if i == BIT_WIDTH - 1 {
                assert_eq!(word_15_val, I);
            } else {
                assert_eq!(word_15_val, O);
            }
        }
    }
}
