use crate::bit::Bit;
use crate::bit::Bit::{I, O};
use crate::clock::Clock;
use crate::word::Word;
use std::io;

pub struct Keyboard {
    word: Word,
}

impl Keyboard {
    pub fn new() -> Self {
        Keyboard {
            word: Word::new_empty(),
        }
    }

    fn matching(line: String) -> Option<Word> {
        let b = line.as_bytes();
        match *b {
            [] => Some(Word::new_empty()),
            // one character
            [num] => {
                if (48 <= num && num <= 58)  // 0-9
                || (65 <= num && num <= 90)  // A-Z
                || (97 <= num && num <= 122)
                // a-z
                {
                    Some(Word::from_num(num.into()))
                } else {
                    match num {
                        // new line
                        10 => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, O, O, O])),
                        // esc
                        27 => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, O, I, I])),
                        // home
                        32 => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, I, I, O])),
                        _ => None,
                    }
                }
            }
            // new line
            [110, 101, 119, 108, 105, 110, 101] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, O, O, O]))
            }
            // backspace
            [98, 97, 99, 107, 115, 112, 97, 99, 101] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, O, O, I]))
            }
            // leftarrow
            [27, 91, 68] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, O, I, O])),
            [108, 101, 102, 116, 97, 114, 114, 111, 119] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, O, I, O]))
            }
            // uparrow
            [27, 91, 65] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, O, I, I])),
            [117, 112, 97, 114, 114, 111, 119] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, O, I, I]))
            }
            // rightarrow
            [27, 91, 67] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, I, O, O])),
            [114, 105, 103, 104, 116, 97, 114, 114, 111, 119] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, I, O, O]))
            }
            // downarrow
            [27, 91, 66] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, I, O, I])),
            [100, 111, 119, 110, 97, 114, 114, 111, 119] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, I, O, I]))
            }
            // home
            // [32] =>
            [104, 111, 109, 101] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, I, I, O]))
            }
            // end
            [101, 110, 100] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, I, I, I])),
            // pageup
            [112, 97, 103, 101, 117, 112] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, O, O, O]))
            }
            // pagedown
            [112, 97, 103, 101, 100, 111, 119, 110] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, O, O, I]))
            }
            // insert
            [105, 110, 115, 101, 114, 116] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, O, I, O]))
            }
            // delete
            [100, 101, 108, 101, 116, 101] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, O, I, O]))
            }
            // esc
            // [27] =>
            [101, 115, 99] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, O, I, I])),
            // f1
            [27, 79, 80] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, O, O])),
            [102, 49] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, O, O])),
            [70, 49] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, O, O])),
            // f2
            [27, 79, 81] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, O, I])),
            [102, 50] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, O, I])),
            [70, 50] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, O, I])),
            // f3
            [27, 79, 82] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, I, O])),
            [102, 51] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, I, O])),
            [70, 51] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, I, O])),
            // f4
            [27, 79, 83] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, I, I])),
            [102, 52] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, I, I])),
            [70, 52] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, O, I, I, I, I])),
            // f5
            [27, 91, 49, 53, 126] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, O, O]))
            }
            [102, 53] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, O, O])),
            [70, 53] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, O, O])),
            // f6
            [27, 91, 49, 55, 126] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, O, I]))
            }
            [102, 54] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, O, I])),
            [70, 54] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, O, I])),
            // f7
            [27, 91, 49, 56, 126] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, I, I]))
            }
            [102, 55] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, I, I])),
            [70, 55] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, O, I, I])),
            // f8
            [27, 91, 49, 57, 126] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, O, O]))
            }
            [102, 56] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, O, O])),
            [70, 56] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, O, O])),
            // f9
            [27, 91, 50, 48, 126] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, O, I]))
            }
            [102, 57] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, O, I])),
            [70, 57] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, O, I])),
            // f10
            [27, 91, 50, 49, 126] => {
                Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, I, O]))
            }
            [102, 49, 48] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, I, O])),
            [70, 49, 48] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, I, O])),
            // f11
            [102, 49, 49] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, I, I])),
            [70, 49, 49] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, O, I, I, I])),
            // f12
            [102, 49, 50] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, I, O, O, O])),
            [70, 49, 50] => Some(Word::new([O, O, O, O, O, O, O, O, I, O, O, I, I, O, O, O])),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matching() {
        let mut line = "0";
        assert_eq!(
            Keyboard::matching(String::from(line)).expect("0"),
            Word::from_num(48)
        );

        line = "a";
        assert_eq!(
            Keyboard::matching(String::from(line)).expect("a"),
            Word::from_num(97)
        );

        line = "newline";
        assert_eq!(
            Keyboard::matching(String::from(line)).expect("new line"),
            Word::new([O, O, O, O, O, O, O, O, I, O, O, O, O, O, O, O])
        );
    }
}
