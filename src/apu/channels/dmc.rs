// https://www.nesdev.org/dmc.txt
// https://www.nesdev.org/wiki/APU_DMC
// https://www.slack.net/~ant/nes-emu/apu_ref.txt

use crate::{
    mappers::{Mapper, MapperChip},
    utils::{BitFlag, Clock},
};

use super::common::{Channel, Timer};

const RATES: [u16; 16] = [
    428, 380, 340, 320, 286, 254, 226, 214, 190, 160, 142, 128, 106, 84, 72, 54,
];

#[derive(Debug)]
pub struct Dmc {
    enabled: bool,
    irq_flag: bool,
    loop_flag: bool,
    output_level: u8,
    sample_rate: u16,
    sample_address: u16,
    sample_length: u16,
    sample_buffer: u8,
    remaining_bytes: u16,
    current_address: u16,
    shift_register: u8,
    shift_counter: u8,
    silence_flag: bool,
    dma_cycles: Option<u8>,
    timer: Timer,
    mapper: MapperChip,
}

impl Dmc {
    pub fn new(mapper: MapperChip) -> Self {
        Self {
            mapper,
            enabled: false,
            irq_flag: false,
            loop_flag: false,
            output_level: 0,
            sample_rate: 0,
            sample_address: 0,
            sample_length: 0,
            sample_buffer: 0,
            remaining_bytes: 0,
            current_address: 0,
            shift_register: 0,
            shift_counter: 0,
            silence_flag: false,
            dma_cycles: Some(0),
            timer: Timer::default(),
        }
    }

    pub fn irq(&self) -> bool {
        self.irq_flag
    }

    pub fn clear_irq(&mut self) {
        self.irq_flag = false;
    }

    pub fn take_dma_cycles(&mut self) -> Option<u8> {
        self.dma_cycles.take()
    }

    fn restart(&mut self) {
        self.current_address = self.sample_address;
        self.remaining_bytes = self.sample_length;
    }
}

impl Channel for Dmc {
    fn write_register(&mut self, address: u16, value: u8) {
        match address % 4 {
            0 => {
                self.irq_flag = value.contains(7);
                self.loop_flag = value.contains(6);
                self.sample_rate = RATES[value.get_range(0..4) as usize];
                self.timer.period = self.sample_rate;
            }
            1 => self.output_level = value.get_range(0..7),
            2 => self.sample_address = 0xC00 + (64 * value as u16),
            _ => self.sample_length = (16 * value as u16) + 1,
        }
    }

    fn raw_sample(&self) -> u8 {
        self.output_level
    }

    fn is_active(&self) -> bool {
        self.remaining_bytes > 0
    }

    fn is_mute(&self) -> bool {
        !self.enabled
    }

    fn set_enabled(&mut self, value: bool) {
        self.enabled = value;

        if !self.enabled {
            self.remaining_bytes = 0;
        } else {
            self.restart();
        }
    }
}

impl Clock for Dmc {
    fn tick(&mut self) {
        if !self.enabled {
            return;
        }

        self.timer.tick();

        if self.timer.is_zero() {
            // TODO
        }
    }
}
