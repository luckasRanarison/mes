use crate::{
    apu::{
        envelope::Envelope,
        frame_counter::{ClockFrame, Frame},
        length_counter::LengthCounter,
        timer::Timer,
    },
    utils::{BitFlag, Clock},
};

use super::Channel;

const PERIODS: [u16; 16] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];

#[derive(Debug, Default)]
pub struct Noise {
    envolope: Envelope,
    timer: Timer,
    length_counter: LengthCounter,
    mode: bool,
    shift: u16,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            shift: 1,
            ..Default::default()
        }
    }
}

impl Channel for Noise {
    fn write(&mut self, address: u16, value: u8) {
        match address % 4 {
            0 => {
                self.envolope.write(value);
                self.length_counter.set_halt(value.contains(5));
            }
            2 => {
                let index = value.get_range(0..4) as usize;
                self.mode = value.contains(7);
                self.timer.period = PERIODS[index];
            }
            3 => self.length_counter.set_length(value >> 3),
            _ => {} // unused
        }
    }

    fn sample(&self) -> u8 {
        self.envolope.volume()
    }

    fn active(&self) -> bool {
        self.length_counter.active()
    }

    fn set_enabled(&mut self, value: bool) {
        self.length_counter.set_enabled(value);
    }

    fn is_mute(&self) -> bool {
        !self.length_counter.active() || self.shift.contains(0)
    }
}

impl Clock for Noise {
    fn tick(&mut self) {
        self.timer.tick();

        if self.timer.is_zero() {
            let rhs_bit = if self.mode { 6 } else { 1 };
            let rhs = self.shift.get(rhs_bit);
            let lhs = self.shift.get(0);
            let feedback = lhs ^ rhs;
            self.shift >>= 1;
            self.shift |= feedback << 14;
        }
    }
}

impl ClockFrame for Noise {
    fn tick_frame(&mut self, frame: &Frame) {
        self.envolope.tick();

        if frame.is_half() {
            self.length_counter.tick();
        }
    }
}
