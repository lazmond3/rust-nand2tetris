use crate::bit::Bit;
use crate::clock::{Clock, ClockState};

pub struct Dff {
    state_past: Bit,
    state_next: Bit,
}

impl Dff {
    fn new() -> Self {
        Dff {
            state_past: Bit::O,
            state_next: Bit::O,
        }
    }

    fn input(&mut self, a: Bit, clock: &Clock) {
        if clock.state() == Bit::I {
            self.state_past = self.state_next.clone();
            self.state_next = a;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn for_dff() {
        let mut dff = Dff::new();
        let mut clock = Clock::new();
        // before clock
        assert_eq!(dff.state_past, Bit::O);
        assert_eq!(dff.state_next, Bit::O);
        // clock up
        clock.next();
        dff.input(Bit::I, &clock);
        // clock
        assert_eq!(dff.state_past, Bit::O);
        assert_eq!(dff.state_next, Bit::I);
        // clock down & up
        clock.next();
        clock.next();
        dff.input(Bit::O, &clock);
        assert_eq!(dff.state_past, Bit::I);
        assert_eq!(dff.state_next, Bit::O);
    }
}
