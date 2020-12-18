use std::fmt;

#[derive(Debug, PartialEq, Clone)]
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
