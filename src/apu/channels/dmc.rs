// https://www.nesdev.org/dmc.txt
// https://www.nesdev.org/wiki/APU_DMC
// https://www.slack.net/~ant/nes-emu/apu_ref.txt

use crate::{
    mappers::{Mapper, MapperChip},
    utils::{BitFlag, Clock},
};

use super::common::{Channel, Timer};

#[derive(Debug)]
struct Reader {
    mapper: MapperChip,
    sample_address: u16,
    sample_length: u16,
    remaining_bytes: u16,
    current_address: u16,
    dma_cycles: Option<u8>,
}

impl Reader {
    fn new(mapper: MapperChip) -> Self {
        Self {
            mapper,
            sample_address: 0,
            sample_length: 0,
            remaining_bytes: 0,
            current_address: 0,
            dma_cycles: None,
        }
    }

    fn restart(&mut self) {
        self.current_address = self.sample_address;
        self.remaining_bytes = self.sample_length;
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.mapper.read(self.current_address);

        self.remaining_bytes -= 1;
        self.dma_cycles = Some(4); // TODO: variable cycle length

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
    fn new() -> Self {
        Self {
            shift_counter: 8,
            ..Default::default()
        }
    }
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
        let bit = self.shift_register.get(0);

        match bit {
            _ if self.silence_flag => {}
            1 if self.level < 126 => self.level += 2,
            0 if self.level > 1 => self.level -= 2,
            _ => {}
        }

        self.shift_register >>= 1;
        self.shift_counter -= 1;
    }
}

impl Clock for OutputUnit {
    fn tick(&mut self) {
        if self.shift_counter == 0 {
            self.start_cycle();
        }

        self.shift();
    }
}

#[derive(Debug)]
pub struct Dmc {
    irq_flag: bool,
    irq_status: bool,
    loop_flag: bool,
    reader: Reader,
    output: OutputUnit,
    timer: Timer,
}

impl Dmc {
    #[rustfmt::skip]
    const SAMPLE_RATES: [u16; 16] = [
        0x1AC, 0x17C, 0x154, 0x140, 0x11E, 0x0FE, 0x0E2, 0x0D6,
        0x0BE, 0x0A0, 0x08E, 0x080, 0x06A, 0x054, 0x048, 0x036,
    ];

    pub fn new(mapper: MapperChip) -> Self {
        Self {
            irq_flag: false,
            irq_status: false,
            loop_flag: false,
            reader: Reader::new(mapper),
            output: OutputUnit::new(),
            timer: Timer::default(),
        }
    }

    pub fn irq(&self) -> bool {
        self.irq_status
    }

    pub fn clear_irq(&mut self) {
        self.irq_status = false;
        self.irq_flag = false;
    }

    pub fn take_dma_cycles(&mut self) -> Option<u8> {
        self.reader.dma_cycles.take()
    }

    pub fn should_fetch(&self) -> bool {
        self.output.shift_counter == 0 && self.reader.remaining_bytes > 0
    }
}

impl Channel for Dmc {
    fn write_register(&mut self, address: u16, value: u8) {
        match address % 4 {
            0 => {
                self.irq_flag = value.contains(7);
                self.loop_flag = value.contains(6);
                self.timer.period = Self::SAMPLE_RATES[value.get_range(0..4) as usize];
            }
            1 => self.output.level = value.get_range(0..7),
            2 => self.reader.sample_address = 0xC000 + (64 * value as u16),
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
        false
    }

    fn set_enabled(&mut self, value: bool) {
        if !value {
            self.reader.remaining_bytes = 0;
            self.output.silence_flag = true;
        } else if self.reader.remaining_bytes == 0 {
            self.reader.restart();
        }
    }
}

impl Clock for Dmc {
    fn tick(&mut self) {
        if self.should_fetch() {
            self.output.buffer = self.reader.fetch_byte().into();

            if self.reader.remaining_bytes == 0 {
                if self.loop_flag {
                    self.reader.restart();
                } else if self.irq_flag {
                    self.irq_status = true;
                }
            }
        }

        self.timer.tick();

        if self.timer.is_zero() {
            self.output.tick();
        }
    }
}
