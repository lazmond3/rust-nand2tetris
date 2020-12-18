use crate::bit::Bit;
use crate::bit::Bit::{I, O};
use crate::nand::nand;

pub fn not(a: Bit) -> Bit {
    nand(a.clone(), a)
}

#[cfg(test)]
mod tests {
    use super::not;
    use super::Bit::{I, O};

    #[test]
    fn for_not() {
        assert_eq!(not(O), I);
        assert_eq!(not(I), O);
    }
}
