use crate::{
    apu::frame_counter::{ClockFrame, Frame},
    utils::{BitFlag, Clock},
};

use super::common::{Channel, Timer};

const RATES: [u16; 16] = [
    428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 84, 72, 54,
];

#[derive(Debug, Default)]
pub struct Dmc {
    irq_flag: bool,
    loop_flag: bool,
    output_level: u8,
    sample_rate: u16,
    sample_address: u16,
    sample_length: u16,
    sample_buffer: Option<u8>,
    remaining_bytes: u16,
    timer: Timer,
}

impl Dmc {
    pub fn irq(&self) -> bool {
        self.irq_flag
    }
}

impl Channel for Dmc {
    fn write_register(&mut self, address: u16, value: u8) {
        match address % 4 {
            0 => {
                self.irq_flag = value.contains(7);
                self.loop_flag = value.contains(6);
                self.sample_rate = RATES[value.get_range(0..4) as usize];
            }
            1 => self.output_level = value.get_range(0..7),
            2 => self.sample_address = 0xC00 + (64 * value as u16),
            _ => self.sample_length = (16 * value as u16) + 1,
        }
    }

    fn raw_sample(&self) -> u8 {
        todo!()
    }

    fn is_active(&self) -> bool {
        self.remaining_bytes > 0
    }

    fn is_mute(&self) -> bool {
        todo!()
    }

    fn set_enabled(&mut self, value: bool) {
        todo!()
    }
}

impl Clock for Dmc {
    fn tick(&mut self) {}
}

impl ClockFrame for Dmc {
    fn tick_frame(&mut self, frame: &Frame) {
        todo!()
    }
}
