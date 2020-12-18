#[cfg(test)]
mod tests {
    use super::bit::{0,I};
    use super::Nand;

    #[test]
    fn for_nand() {
        assert_eq!(Nand(O, O), I);
        assert_eq!(Nand(O, I), I);
        assert_eq!(Nand(I, O), I);
        assert_eq!(Nand(I, I), O);
    }
}