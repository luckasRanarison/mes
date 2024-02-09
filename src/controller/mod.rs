use crate::utils::{BitFlag, Reset};

#[derive(Debug, Default)]
pub struct ControllerState {
    state: [u8; 2],
    shift: [u8; 2],
}

impl ControllerState {
    pub fn set_state(&mut self, id: usize, value: u8) {
        self.state[id] = value;
    }

    pub fn reload_shift_registers(&mut self) {
        self.shift = self.state;
    }

    pub fn poll_button(&mut self, id: usize) -> u8 {
        let value = self.shift[id].get(7);
        self.shift[id] <<= 1;
        value
    }
}

impl Reset for ControllerState {
    fn reset(&mut self) {
        self.state = [0; 2];
        self.shift = [0; 2];
    }
}
