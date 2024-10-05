// https://www.nesdev.org/wiki/APU_Length_Counter

use crate::utils::Clock;

const LENGTH_TABLE: [u8; 32] = [
    0x0A, 0xFE, 0x14, 0x02, 0x28, 0x04, 0x50, 0x06, 0xA0, 0x08, 0x3C, 0x0A, 0x0E, 0x0C, 0x1A, 0x0E,
    0x0C, 0x10, 0x18, 0x12, 0x30, 0x14, 0x60, 0x16, 0xC0, 0x18, 0x48, 0x1A, 0x10, 0x1C, 0x20, 0x1E,
];

#[derive(Debug, Default)]
pub struct LengthCounter {
    counter: u8,
    halted: bool,
    enabled: bool,
}

impl LengthCounter {
    pub fn set_length(&mut self, index: u8) {
        if self.enabled {
            self.counter = LENGTH_TABLE[index as usize];
        }
    }

    pub fn set_halt(&mut self, value: bool) {
        self.halted = value;
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.enabled = value;
    }

    pub fn is_active(&self) -> bool {
        self.enabled && self.counter > 0
    }
}

impl Clock for LengthCounter {
    fn tick(&mut self) {
        if !self.halted && self.counter > 0 {
            self.counter -= 1;
        }
    }
}
