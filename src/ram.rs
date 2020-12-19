use crate::constant::RAM_WORDS_NUM;
use crate::word::Word;
use std::ops::{Index, IndexMut};

pub type InternalRam = [Word; RAM_WORDS_NUM];
pub struct Ram(InternalRam);

impl Ram {
    pub fn new() -> Self {
        // Ram([Word::new_empty(); RAM_WORDS_NUM])
        Ram([Word::new_empty(); RAM_WORDS_NUM])
    }
    pub fn internal(&self) -> &InternalRam {
        &(self.0)
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
}
