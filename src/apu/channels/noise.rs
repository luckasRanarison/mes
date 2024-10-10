// https://www.nesdev.org/wiki/APU_Noise

use crate::{
    apu::frame_counter::{ClockFrame, Frame},
    utils::{BitFlag, Clock},
};

use super::common::{Channel, Envelope, LengthCounter, Timer};

#[derive(Debug, Default)]
pub struct Noise {
    envelope: Envelope,
    timer: Timer,
    length_counter: LengthCounter,
    mode: bool,
    shift: u16,
}

impl Noise {
    #[rustfmt::skip]
    const PERIODS: [u16; 16] = [
        0x004, 0x008, 0x010, 0x020, 0x040, 0x060, 0x080, 0x0A0,
        0x0CA, 0x0FE, 0x17C, 0x1FC, 0x2FA, 0x3F8, 0x7F2, 0xFE4,
    ];

    pub fn new() -> Self {
        Self {
            shift: 1,
            ..Default::default()
        }
    }
}

impl Channel for Noise {
    fn write_register(&mut self, address: u16, value: u8) {
        match address % 4 {
            0 => {
                self.envelope.write(value.get_range(0..5));
                self.length_counter.set_halt(value.contains(5));
            }
            2 => {
                let index = value.get_range(0..4) as usize;
                self.mode = value.contains(7);
                self.timer.period = Self::PERIODS[index];
            }
            3 => {
                self.length_counter.set_length(value >> 3);
                self.envelope.restart();
            }
            _ => {} // unused
        }
    }

    fn raw_sample(&self) -> u8 {
        self.envelope.volume()
    }

    fn is_active(&self) -> bool {
        self.length_counter.is_active()
    }

    fn set_enabled(&mut self, value: bool) {
        self.length_counter.set_enabled(value);
    }

    fn is_mute(&self) -> bool {
        !self.length_counter.is_active() || self.shift.contains(0)
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
            self.shift = (self.shift >> 1) | (feedback << 14);
        }
    }
}

impl ClockFrame for Noise {
    fn tick_frame(&mut self, frame: &Frame) {
        self.envelope.tick();

        if frame.is_half() {
            self.length_counter.tick();
        }
    }
}
