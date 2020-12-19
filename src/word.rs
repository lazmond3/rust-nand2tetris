use crate::bit::Bit;
use crate::constant::{BIT_WIDTH, MAX_VALUE};
use std::convert::{TryFrom, TryInto};
use std::default::Default;
use std::ops::{Index, IndexMut};

use std::cmp::PartialEq;
use std::convert::Infallible;
use std::fmt::Display;
use std::ops::Add;
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

    pub fn convert_vec_to_word(a: Vec<Bit>) -> Word {
        if a.len() != BIT_WIDTH {
            panic!(format!(
                "range error: vector range:{} is mismatched to BIT_WIDTH:{}.",
                a.len(),
                BIT_WIDTH
            ))
        }

        let boxed_slice = a.clone().into_boxed_slice();
        let boxed_array: Box<InternalWord> = match boxed_slice.try_into() {
            Ok(ba) => ba,
            Err(_) => {
                panic!(
                    "Expected a Vec of length {} but it was {}.",
                    BIT_WIDTH,
                    a.len()
                )
            }
        };
        Word::new(*boxed_array)
    }

    pub fn num_to_bit(num: usize) -> Word {
        if num >= MAX_VALUE {
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
            num_m = num_m >> 1;
        }
        while vec.len() != BIT_WIDTH {
            vec.push(Bit::O);
        }

        Word::convert_vec_to_word(vec)
    }

    pub fn to_num(&self) -> usize {
        let mut v: usize = 0;
        for i in 0..BIT_WIDTH {
            if (*self)[i] == Bit::I {
                v = v + 1 * (1 << i)
            }
        }
        v
    }
}

// impl Display for Word {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let mut v = vec!("");
//         for i in 0..BIT_WIDTH

//         write!(f, "{}")
//     }
// }

impl Add for Word {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let added = self.to_num() + other.to_num();
        if added >= MAX_VALUE {
            Word::num_to_bit(MAX_VALUE)
        } else {
            Word::num_to_bit(added)
        }
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

    #[test]
    fn for_convert_vec_to_internal_word() {
        let word_00: Word = Word::bit_position(0);
        let vec = word_00.internal().to_vec();
        let word_from_vec = Word::convert_vec_to_word(vec);
        assert_eq!(word_00, word_from_vec);
    }

    #[test]
    fn for_num_to_bit() {
        let word_08: Word = Word::num_to_bit(8);
        let word_03_bit: Word = Word::bit_position(3);
        assert_eq!(word_08, word_03_bit);
    }

    #[test]
    fn for_to_num() {
        let word_03_bit: Word = Word::bit_position(3);
        assert_eq!(word_03_bit.to_num(), 8);
    }

    #[test]
    fn for_bit_shift_direction() {
        assert_eq!(1 << 1, 2);
        assert_eq!(1 << 2, 4);
        assert_eq!(4 >> 1, 2);
    }

    #[test]
    fn for_add_test() {
        assert_eq!(
            Word::num_to_bit(5) + Word::num_to_bit(5),
            Word::num_to_bit(10)
        );
    }
}
