use crate::bit::Bit;



pub fn dmux(a: Bit, b: Bit, sel: Bit) -> Bit {
    match sel {
        Bit::O => a,
        Bit::I => b,
    }
}

#[cfg(test)]
mod tests {
    use super::dmux;
    use super::Bit::{I, O};

    #[test]
    fn for_dmux() {
        assert_eq!(dmux(O, O, O), O);
        assert_eq!(dmux(O, O, I), O);
        assert_eq!(dmux(O, I, O), O);
        assert_eq!(dmux(O, I, I), I);
        assert_eq!(dmux(I, O, O), I);
        assert_eq!(dmux(I, O, I), O);
        assert_eq!(dmux(I, I, I), I);
    }
}
