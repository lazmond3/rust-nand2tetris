use crate::bit::Bit;
use crate::constant::{BIT_WIDTH, MAX_VALUE};
use std::convert::{TryFrom, TryInto};
use std::default::Default;
use std::ops::{Index, IndexMut};

use std::cmp::PartialEq;
use std::convert::Infallible;
use std::slice::Iter;

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
    pub fn to_vec(&self) -> Vec<Bit> {
        (*self).internal().to_vec()
    }

    pub fn bit_position(index: usize) -> Word {
        if (BIT_WIDTH - 1) < index {
            panic!(format!("index fail: {} is out of range.", index))
        }
        let mut ar: Word = Default::default();
        ar[index] = Bit::I;
        ar
    }

    pub fn num_to_bit(num: usize) -> Word {
        if num <= MAX_VALUE {
            panic!(format!(
                "num_to_bit conversion failed: {} is out of range.",
                num
            ))
        }
        let mut vec: Vec<Bit> = Vec::new();
        let mut num_m = num.clone();
        while num_m != 0 {
            let amari = num_m & 1;
            if amari == 1 {
                vec.push(Bit::I)
            } else {
                vec.push(Bit::O)
            }
            num_m = num_m << 1;
        }
        while vec.len() != BIT_WIDTH {
            vec.push(Bit::O);
        }

        Default::default()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_equality() {
        assert_eq!(Word::bit_position(1), Word::bit_position(1));
    }

    #[test]
    fn for_word() {
        let word_00: Word = Word::bit_position(0);
        let word_15: Word = Word::bit_position(BIT_WIDTH - 1);
        for i in 1..BIT_WIDTH {
            let word_00_val = word_00[i].clone();
            let word_15_val = word_15[i].clone();
            if i == 0 {
                assert_eq!(word_00_val, Bit::I);
            } else {
                assert_eq!(word_00_val, Bit::O);
            }

            if i == BIT_WIDTH - 1 {
                assert_eq!(word_15_val, Bit::I);
            } else {
                assert_eq!(word_15_val, Bit::O);
            }
        }
    }
}
