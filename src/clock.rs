use crate::bit::Bit;
use crate::not::not;

pub type ClockState = Bit;

pub struct Clock {
    state: ClockState,
}

impl Clock {
    pub fn state(&self) -> ClockState {
        self.state.clone()
    }

    pub fn next(&mut self) {
        self.state = not(self.state.clone())
    }

    pub fn new() -> Self {
        Clock { state: Bit::O }
    }
}
