use crate::constant::RAM_WORDS_NUM;
use crate::word::Word;
use std::convert::TryInto;
use std::ops::{Index, IndexMut};

pub type InternalRam = [Word; RAM_WORDS_NUM];

#[derive(Clone)]
pub struct Ram(InternalRam);

impl Default for Ram {
    fn default() -> Self {
        Ram([Word::new_empty(); RAM_WORDS_NUM])
    }
}

impl Ram {
    pub fn new() -> Self {
        // Ram([Word::new_empty(); RAM_WORDS_NUM])
        Default::default()
    }
    pub fn internal(&self) -> &InternalRam {
        &(self.0)
    }
    pub fn to_vec(&self) -> Vec<Word> {
        (*self).internal().to_vec()
    }
    pub fn one_line_word(pos: usize, a: Word) -> Self {
        let mut v = Ram::new().internal().clone();
        if RAM_WORDS_NUM <= pos {
            panic!(format!(
                "range error: vector range:{} is mismatched to RAM_WORDS_NUM:{}.",
                pos, RAM_WORDS_NUM
            ))
        }
        v[pos] = a;
        Ram(v)
    }

    pub fn from_vec_word(a: Vec<Word>) -> Self {
        if a.len() != RAM_WORDS_NUM {
            panic!(format!(
                "range error: vector range:{} is mismatched to RAM_WORDS_NUM:{}.",
                a.len(),
                RAM_WORDS_NUM
            ))
        }
        let boxed_slice = a.clone().into_boxed_slice();
        let boxed_array: Box<InternalRam> = match boxed_slice.try_into() {
            Ok(ba) => ba,
            Err(_) => {
                panic!(
                    "Expected a Vec of length {} but it was {}.",
                    RAM_WORDS_NUM,
                    a.len()
                )
            }
        };
        Ram(*boxed_array)
    }
}

impl Index<usize> for Ram {
    type Output = Word;
    fn index(&self, index: usize) -> &Self::Output {
        if RAM_WORDS_NUM <= index {
            panic!(format!("index fail: {} is out of range.", index))
        }
        &self.0[index]
    }
}
impl IndexMut<usize> for Ram {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if RAM_WORDS_NUM <= index {
            panic!(format!("index fail: {} is out of range.", index))
        }
        self.0.index_mut(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_ram() {
        let ram = Ram::new();
        assert_eq!(ram[0], Word::new_empty());
    }

    #[test]
    fn for_one_line_word() {
        let pos: usize = 100;
        let ram = Ram::one_line_word(pos, Word::bit_position(0));
        assert_eq!(ram[pos], Word::bit_position(0));
    }

    // todo: vec <-> array のテスト
    #[test]
    fn for_vec_array() {
        let ram = Ram::one_line_word(0, Word::bit_position(0));
        let vec = ram.to_vec();
        assert_eq!(vec[0], Word::bit_position(0));
        let to_ram = Ram::from_vec_word(vec);
        assert_eq!(to_ram[0], Word::bit_position(0));
    }
}
