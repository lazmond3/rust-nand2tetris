use crate::constant::RAM_WORDS_NUM;
use crate::word::Word;
use std::cmp::PartialEq;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};
use std::ops::{Index, IndexMut};

pub type InternalRam = [Word; RAM_WORDS_NUM];

#[derive(Debug, Clone, Copy)]
pub struct Ram(InternalRam);

impl Default for Ram {
    fn default() -> Self {
        Ram([Word::new_empty(); RAM_WORDS_NUM])
    }
}

impl PartialEq for Ram {
    fn eq(&self, other: &Self) -> bool {
        (*self)
            .internal()
            .to_vec()
            .iter()
            .zip(other.internal().to_vec().iter())
            .all(|(m, o)| *m == *o)
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

    pub fn load(&mut self, file_name: &str) {
        let file = File::open(file_name.clone()).expect(&format!("Fail to open {}", file_name));
        let reader = BufReader::new(file);
        let mut word_vec: Vec<Word> = vec![];
        for line in reader.lines() {
            let un = line.unwrap();
            println!("line: {} ", un.clone());
            let word = match Word::from_str(&un) {
                Ok(w) => w,
                Err(v) => panic!(format!(
                    "failed to load file: {}, due to : {}",
                    file_name, v
                )),
            };
            word_vec.push(word);
        }
        for (i, &word) in word_vec.iter().enumerate() {
            (*self)[i] = word;
        }
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

    #[test]
    fn load_test() {
        let mut answer_ram: Ram = Default::default();
        let mut test_ram: Ram = Default::default();
        let v = vec![
            Word::_from_str(&String::from("1000110101011000")),
            Word::_from_str(&String::from("1011100001101111")),
            Word::_from_str(&String::from("0010100111011110")),
            Word::_from_str(&String::from("0011011001011111")),
            Word::_from_str(&String::from("0001011011110010")),
            Word::_from_str(&String::from("0100010110100111")),
            Word::_from_str(&String::from("0110110101010110")),
            Word::_from_str(&String::from("1110001001000110")),
            Word::_from_str(&String::from("1001000000101010")),
            Word::_from_str(&String::from("0111110001100110")),
        ];
        v.iter().enumerate().for_each(|(i, &w)| {
            answer_ram[i] = w;
        });

        test_ram.load("rom_data.txt");
        assert_eq!(test_ram, answer_ram);
    }
}
