// https://www.nesdev.org/wiki/MMC3

use super::Mapper;
use crate::{
    cartridge::{Cartridge, ChrPage, Mirroring, PrgPage},
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
        match address {
            0x0000..=0x03FF if self.chr_inversion == 0 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[0] & 0b1111_1110)),
            0x0400..=0x07FF if self.chr_inversion == 0 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[0] | 1)),
            0x0800..=0x0BFF if self.chr_inversion == 0 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[1] & 0b1111_1110)),
            0x0C00..=0x0FFF if self.chr_inversion == 0 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[1] | 1)),
            0x1000..=0x13FF if self.chr_inversion == 0 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[2])),
            0x1400..=0x17FF if self.chr_inversion == 0 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[3])),
            0x1800..=0x1BFF if self.chr_inversion == 0 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[4])),
            0x1C00..=0x1FFF if self.chr_inversion == 0 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[5])),
            0x0000..=0x03FF if self.chr_inversion == 1 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[2])),
            0x0400..=0x07FF if self.chr_inversion == 1 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[3])),
            0x0800..=0x0BFF if self.chr_inversion == 1 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[4])),
            0x0C00..=0x0FFF if self.chr_inversion == 1 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[5])),
            0x1000..=0x13FF if self.chr_inversion == 1 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[0] & 0b1111_1110)),
            0x1400..=0x17FF if self.chr_inversion == 1 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[0] | 1)),
            0x1800..=0x1BFF if self.chr_inversion == 1 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[1] & 0b1111_1110)),
            0x1C00..=0x1FFF if self.chr_inversion == 1 => self
                .cartridge
                .read_chr(address, ChrPage::Index1(self.registers[1] | 1)),
            0x4200..=0x5FFF => 0,
            0x6000..=0x7FFF => self.cartridge.read_prg_ram(address),
            0x8000..=0x9FFF if self.prg_bank_mode == 0 => self
                .cartridge
                .read_prg_rom(address, PrgPage::Index8(self.registers[6] & 0b0011_1111)),
            0xA000..=0xBFFF => self
                .cartridge
                .read_prg_rom(address, PrgPage::Index8(self.registers[7] & 0b0011_1111)),
            0xC000..=0xDFFF if self.prg_bank_mode == 0 => {
                self.cartridge.read_prg_rom(address, PrgPage::Last8(1))
            }
            0xE000..=0xFFFF => self.cartridge.read_prg_rom(address, PrgPage::Last8(0)),
            0x8000..=0x9FFF if self.prg_bank_mode == 1 => {
                self.cartridge.read_prg_rom(address, PrgPage::Last8(1))
            }
            0xC000..=0xDFFF if self.prg_bank_mode == 1 => self
                .cartridge
                .read_prg_rom(address, PrgPage::Index8(self.registers[6] & 0b0011_1111)),
            _ => panic!("Trying to read from invalid address: 0x{:x}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match (address, address % 2 == 0) {
            (0x6000..=0x7FFF, _) => self.cartridge.write_prg_ram(address, value),
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
    fn reset(&mut self) {
        self.registers = [0; 8];
        self.mirroring = self.cartridge.header.mirroring;
        self.current_register = 0;
        self.prg_bank_mode = 0;
        self.chr_inversion = 0;
        self.irq_counter = 0;
        self.irq_latch = 0;
        self.irq_reset = false;
        self.irq_enable = false;
        self.irq.take();
    }
}
