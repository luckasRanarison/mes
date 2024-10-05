use crate::{
    apu::{
        envelope::Envelope,
        frame_counter::{ClockHalfFrame, ClockQuarterFrame},
        length_counter::LengthCounter,
        timer::Timer,
    },
    utils::{BitFlag, Clock},
};

use super::Channel;

const PERIODS: [u16; 16] = [
    4, 8, 14, 30, 60, 88, 118, 148, 188, 236, 354, 472, 708, 944, 1890, 3778,
];

#[derive(Debug, Default)]
pub struct Noise {
    envolope: Envelope,
    timer: Timer,
    length_counter: LengthCounter,
    mode: bool,
    shift: u8,
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
        todo!()
    }

    fn active(&self) -> bool {
        self.length_counter.active()
    }

    fn set_enabled(&mut self, value: bool) {
        self.length_counter.set_enabled(value);
    }
}

impl Clock for Noise {
    fn tick(&mut self) {
        self.timer.tick();

        if self.timer.is_zero() {
            // shifting
        }
    }
}

impl ClockQuarterFrame for Noise {
    fn tick_quarter(&mut self) {
        self.envolope.tick();
    }
}

impl ClockHalfFrame for Noise {
    fn tick_half(&mut self) {
        self.length_counter.tick();
    }
}
