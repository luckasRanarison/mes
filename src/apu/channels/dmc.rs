// https://www.nesdev.org/dmc.txt
// https://www.nesdev.org/wiki/APU_DMC
// https://www.slack.net/~ant/nes-emu/apu_ref.txt

use crate::{
    mappers::{Mapper, MapperChip},
    utils::{BitFlag, Clock},
};

use super::common::{Channel, Timer};

#[derive(Debug)]
struct DmaReader {
    mapper: MapperChip,
    sample_address: u16,
    sample_length: u16,
    sample_buffer: Option<u8>,
    remaining_bytes: u16,
    current_address: u16,
    dma_cycles: Option<u8>,
}

impl DmaReader {
    fn new(mapper: MapperChip) -> Self {
        Self {
            mapper,
            sample_address: 0,
            sample_length: 0,
            sample_buffer: None,
            remaining_bytes: 0,
            current_address: 0,
            dma_cycles: None,
        }
    }

    fn restart(&mut self) {
        self.current_address = self.sample_address;
        self.remaining_bytes = self.sample_length;
    }

    fn should_fetch(&self) -> bool {
        self.remaining_bytes > 0 && self.sample_buffer.is_none()
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.mapper.read(self.current_address);

        self.sample_buffer = Some(byte);
        self.remaining_bytes -= 1;
        *self.dma_cycles.get_or_insert(0) += 4;

        if self.current_address == 0xFFFF {
            self.current_address = 0x8000;
        } else {
            self.current_address += 1;
        }

        byte
    }
}

#[derive(Debug, Default)]
struct OutputUnit {
    buffer: Option<u8>,
    shift_register: u8,
    shift_counter: u8,
    silence_flag: bool,
    level: u8,
}

impl OutputUnit {
    fn start_cycle(&mut self) {
        self.shift_counter = 8;

        if let Some(buffer) = self.buffer.take() {
            self.silence_flag = false;
            self.shift_register = buffer;
        } else {
            self.silence_flag = true;
        }
    }

    fn shift(&mut self) {
        match self.shift_register.contains(0) {
            true if self.level < 126 => self.level += 2,
            false if self.level > 2 => self.level -= 2,
            _ => {}
        }
    }
}

impl Clock for OutputUnit {
    fn tick(&mut self) {
        if !self.silence_flag {
            self.shift();
        }

        if self.shift_counter > 0 {
            self.shift_counter -= 1;
        }

        if self.shift_counter == 0 {
            self.start_cycle();
        }
    }
}

#[derive(Debug)]
pub struct Dmc {
    enabled: bool,
    irq_flag: bool,
    irq_status: bool,
    loop_flag: bool,
    reader: DmaReader,
    output: OutputUnit,
    timer: Timer,
}

impl Dmc {
    #[rustfmt::skip]
    const RATES: [u16; 16] = [
        428, 380, 340, 320, 286, 254, 226, 214,
        190, 160, 142, 128, 106, 84, 72, 54,
    ];

    pub fn new(mapper: MapperChip) -> Self {
        Self {
            enabled: false,
            irq_flag: false,
            irq_status: false,
            loop_flag: false,
            reader: DmaReader::new(mapper),
            output: OutputUnit::default(),
            timer: Timer::default(),
        }
    }

    pub fn irq(&self) -> bool {
        self.irq_status
    }

    pub fn clear_irq(&mut self) {
        self.irq_status = false;
    }

    pub fn take_dma_cycles(&mut self) -> Option<u8> {
        self.reader.dma_cycles.take()
    }

    fn fetch_sample(&mut self) {
        self.output.buffer = self.reader.fetch_byte().into();

        if self.reader.remaining_bytes == 0 {
            if self.loop_flag {
                self.reader.restart();
            } else {
                self.irq_status = true;
            }
        }
    }
}

impl Channel for Dmc {
    fn write_register(&mut self, address: u16, value: u8) {
        match address % 4 {
            0 => {
                self.irq_flag = value.contains(7);
                self.loop_flag = value.contains(6);
                self.timer.period = Self::RATES[value.get_range(0..4) as usize] / 2;
            }
            1 => self.output.level = value.get_range(0..7),
            2 => self.reader.sample_address = 0xC00 + (64 * value as u16),
            _ => self.reader.sample_length = (16 * value as u16) + 1,
        }
    }

    fn raw_sample(&self) -> u8 {
        self.output.level
    }

    fn is_active(&self) -> bool {
        self.reader.remaining_bytes > 0
    }

    fn is_mute(&self) -> bool {
        !self.enabled
    }

    fn set_enabled(&mut self, value: bool) {
        self.enabled = value;

        if !self.enabled {
            self.reader.remaining_bytes = 0;
        } else {
            self.reader.restart();
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
            self.output.tick();
        }

        if self.reader.should_fetch() {
            self.fetch_sample();
        }
    }
}
