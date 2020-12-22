use std::default::Default;
use std::fmt;
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Bit {
    O,
    I,
}

impl Bit {
    pub fn from_bool(it: bool) -> Self {
        match it {
            true => Bit::I,
            false => Bit::O,
        }
    }
    pub fn to_string(&self) -> String {
        match *self {
            Bit::O => "O",
            Bit::I => "I",
        }
    }
}

impl Default for Bit {
    fn default() -> Self {
        Bit::O
    }
}

impl fmt::Display for Bit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let v = match *self {
            Bit::O => "O",
            Bit::I => "I",
        };
        write!(f, "{}", v)
    }
}
