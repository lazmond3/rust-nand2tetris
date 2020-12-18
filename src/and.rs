use crate::bit::Bit;
use crate::nand::nand;
use crate::not::not;

fn and(a: Bit, b: Bit) -> Bit {
    not(nand(a, b))
}

#[cfg(test)]
mod tests {
    use super::and;
    use super::Bit::{I, O};

    #[test]
    fn for_and() {
        assert_eq!(and(O, O), O);
        assert_eq!(and(O, I), O);
        assert_eq!(and(I, O), O);
        assert_eq!(and(I, I), I);
    }
}
