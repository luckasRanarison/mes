// https://www.nesdev.org/wiki/APU_Triangle

use crate::{
    apu::{
        frame_counter::{ClockFrame, Frame},
        length_counter::LengthCounter,
        sequencer::Sequencer,
        timer::Timer,
    },
    utils::{BitFlag, Clock},
};

use super::Channel;

#[rustfmt::skip]
const WAVEFORMS: [u8; 32] = [
    15, 14, 13, 12, 11, 10,  9,  8,  7,  6,  5,  4,  3,  2,  1, 0,
    0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14, 15,
];

#[derive(Debug, Default)]
pub struct Triangle {
    timer: Timer,
    length_counter: LengthCounter,
    sequencer: Sequencer,
    linear_counter: u8,
    counter_reload: u8,
    control_flag: bool,
    reload_flag: bool,
}

impl Triangle {
    pub fn new() -> Self {
        Self {
            sequencer: Sequencer::new(32),
            ..Default::default()
        }
    }
}

impl Channel for Triangle {
    fn write_register(&mut self, address: u16, value: u8) {
        match address % 4 {
            0 => {
                let c = value.contains(7);
                self.length_counter.set_halt(c);
                self.control_flag = c;
                self.counter_reload = value.get_range(0..7);
            }
            2 => self.timer.set_period_lo(value),
            3 => {
                self.timer.set_period_hi(value.get_range(0..3));
                self.length_counter.set_length(value.get_range(3..8));
                self.reload_flag = true;
            }
            _ => {} // unused
        }
    }

    fn raw_sample(&self) -> u8 {
        WAVEFORMS[self.sequencer.index()]
    }

    fn is_active(&self) -> bool {
        self.length_counter.is_active()
    }

    fn is_mute(&self) -> bool {
        !self.length_counter.is_active() || self.linear_counter == 0 || self.timer.period < 2
    }

    fn set_enabled(&mut self, value: bool) {
        self.length_counter.set_enabled(value);
    }
}

impl Clock for Triangle {
    fn tick(&mut self) {
        self.timer.tick();

        if self.timer.is_zero() {
            self.sequencer.step();
        }
    }
}

impl ClockFrame for Triangle {
    fn tick_frame(&mut self, frame: &Frame) {
        if self.reload_flag {
            self.linear_counter = self.counter_reload;
        } else if self.linear_counter > 0 {
            self.linear_counter -= 1;
        }

        if !self.control_flag {
            self.reload_flag = false;
        }

        if frame.is_half() {
            self.length_counter.tick();
        }
    }
}
