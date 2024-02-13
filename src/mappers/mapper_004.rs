use super::Mapper;
use crate::{
    cartridge::{Cartridge, Mirroring},
    utils::{BitFlag, Reset},
};

#[derive(Debug)]
pub struct TxRom {
    cartridge: Cartridge,
    registers: [u8; 8],
    current_register: u8,
    mirroring: Mirroring,
    prg_bank_mode: u8,
    chr_inversion: u8,
    irq_counter: u8,
    irq_latch: u8,
    irq_reset: bool,
    irq_enable: bool,
    irq: Option<bool>,
}

impl TxRom {
    pub fn new(cartridge: Cartridge) -> Self {
        let mirroring = cartridge.header.mirroring;

        Self {
            cartridge,
            registers: [0; 8],
            mirroring,
            current_register: 0,
            prg_bank_mode: 0,
            chr_inversion: 0,
            irq_counter: 0,
            irq_latch: 0,
            irq_reset: false,
            irq_enable: false,
            irq: None,
        }
    }
}

impl Mapper for TxRom {
    fn read(&self, address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, address: u16, value: u8) {
        match (address, address % 2 == 0) {
            (0x8000..=0x9FFE, true) => {
                self.current_register = value & 0b111;
                self.prg_bank_mode = value.get(6);
                self.chr_inversion = value.get(7);
            }
            (0x8001..=0x9FFF, false) => {
                self.registers[self.current_register as usize] = value;
            }
            (0xA000..=0xBFFE, true) if self.mirroring != Mirroring::FourScreen => {
                self.mirroring = match value.get(0) {
                    0 => Mirroring::Vertical,
                    _ => Mirroring::Horizontal,
                }
            }
            (0xC000..=0xDFFE, true) => self.irq_latch = value,
            (0xC001..=0xDFFF, false) => self.irq_reset = true,
            (0xE000..=0xFFFE, true) => {
                self.irq_enable = false;
                self.irq.take();
            }
            (0xE001..=0xFFFF, false) => self.irq_enable = true,
            _ => {}
        }
    }

    fn get_mirroring(&self) -> Mirroring {
        self.mirroring
    }

    fn poll_irq(&mut self) -> bool {
        self.irq.take().is_some()
    }

    fn scanline_hook(&mut self) {
        if self.irq_counter == 0 || self.irq_reset {
            if self.irq_enable {
                self.irq = Some(true);
            }

            self.irq_counter = self.irq_latch;
        } else {
            self.irq_counter -= 1;
        }
    }
}

impl Reset for TxRom {
    fn reset(&mut self) {}
}
