use std::default::Default;
use std::fmt;
#[derive(Debug, PartialEq, Clone)]
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
