use crate::utils::BitFlag;

use super::{length_counter::LengthCounter, sweep::Sweep};

const WAVEFORMS: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];

#[derive(Debug, Default)]
pub struct Pulse {
    duty_cycle: u8,
    length_counter: LengthCounter,
    sweep: Sweep,
}

impl Pulse {
    pub fn write(&mut self, address: u16, value: u8) {
        match address % 4 {
            0 => {
                self.duty_cycle = value.get_range(6..8);
                self.length_counter.set_halt(value.contains(5));
            }
            1 => self.sweep.write(value),
            2 => {}
            _ => {
                self.length_counter.set_length(value.get_range(3..8));
            }
        }
    }

    pub fn active(&self) -> bool {
        self.length_counter.active()
    }

    pub fn set_enabled(&mut self, value: bool) {
        self.length_counter.set_enabled(value);
    }
}
