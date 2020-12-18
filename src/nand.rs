use crate::bit::Bit;
use crate::bit::Bit::{I, O};

pub fn nand(a: Bit, b: Bit) -> Bit {
    match a {
        O => match b {
            O => I,
            I => I,
        },
        I => match b {
            O => I,
            I => O,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::nand;
    use super::Bit::{I, O};

    #[test]
    fn for_nand() {
        assert_eq!(nand(O, O), I);
        assert_eq!(nand(O, I), I);
        assert_eq!(nand(I, O), I);
        assert_eq!(nand(I, I), O);
    }
}
