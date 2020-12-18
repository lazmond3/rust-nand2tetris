use std::cmp::PartialEq;
use std::fmt;

#[derive(Debug)]
pub enum Bit {
    O,
    I,
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

impl PartialEq for Bit {
    fn eq(&self, other: &Self) -> bool {
        match *self {
            Bit::O => match *other {
                Bit::O => true,
                Bit::I => false,
            },
            Bit::I => match *other {
                Bit::O => false,
                Bit::I => true,
            },
        }
    }
}
