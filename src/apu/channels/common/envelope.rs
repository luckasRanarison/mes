// https://www.nesdev.org/wiki/APU_Envelope

use crate::utils::{BitFlag, Clock};

#[derive(Debug, Default)]
pub struct Envelope {
    loop_flag: bool,
    const_flag: bool,
    volume: u8,
    start: bool,
    decay_level: u8,
    counter: u8,
}

impl Envelope {
    pub fn write(&mut self, value: u8) {
        self.loop_flag = value.contains(5);
        self.const_flag = value.contains(4);
        self.volume = value.get_range(0..4);
        self.start = true;
    }

    pub fn volume(&self) -> u8 {
        match self.const_flag {
            true => self.volume,
            false => self.decay_level,
        }
    }

    pub fn restart(&mut self) {
        self.start = true;
    }
}

impl Clock for Envelope {
    fn tick(&mut self) {
        if self.start {
            self.start = false;
            self.decay_level = 15;
        } else if self.counter == 0 {
            self.counter = self.volume;

            if self.decay_level > 0 {
                self.decay_level -= 1;
            } else if self.loop_flag {
                self.decay_level = 15;
            }
        } else {
            self.counter -= 1;
        }
    }
}
