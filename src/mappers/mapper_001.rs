// https://www.nesdev.org/wiki/MMC1

use crate::{
    cartridge::{Cartridge, ChrPage, Mirroring, PrgPage},
    utils::{BitFlag, Reset},
};

use super::Mapper;

#[derive(Debug)]
pub struct SxRom {
    cartridge: Cartridge,
    shift: u8,
    control: u8,
    chr_bank_low: u8,
    chr_bank_high: u8,
    prg_bank: u8,
}

impl SxRom {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            shift: 0b10000,
            control: 0b11100,
            chr_bank_low: 0,
            chr_bank_high: 0,
            prg_bank: 0,
        }
    }

    fn shift(&mut self, value: u8) -> bool {
        if value.contains(7) {
            self.shift = 0b10000;
            false
        } else {
            let is_full = self.shift.contains(0);
            self.shift = (self.shift >> 1) | (value.get(0) << 4);
            is_full
        }
    }
}

impl Mapper for SxRom {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF if !self.control.contains(4) => self
                .cartridge
                .read_chr(address, ChrPage::Index8(self.chr_bank_low & 0b11110)),
            0x0000..=0x0FFF => self
                .cartridge
                .read_chr(address, ChrPage::Index4(self.chr_bank_low)),
            0x1000..=0x1FFF => self
                .cartridge
                .read_chr(address, ChrPage::Index4(self.chr_bank_high)),
            0x4020..=0x5FFF => 0,
            0x6000..=0x7FFF => self.cartridge.read_prg_ram(address),
            0x8000..=0xFFFF if !self.control.contains(3) => self
                .cartridge
                .read_prg_rom(address, PrgPage::Index32(self.prg_bank & 0b11110)),
            0x8000..=0xBFFF if self.control.contains(2) => self
                .cartridge
                .read_prg_rom(address, PrgPage::Index16(self.prg_bank)),
            0xC000..=0xFFFF if self.control.contains(2) => {
                self.cartridge.read_prg_rom(address, PrgPage::Last16)
            }
            0x8000..=0xBFFF if !self.control.contains(2) => {
                self.cartridge.read_prg_rom(address, PrgPage::Index16(0))
            }
            0xC000..=0xFFFF if !self.control.contains(2) => self
                .cartridge
                .read_prg_rom(address, PrgPage::Index16(self.prg_bank)),
            _ => panic!("Trying to read from invalid address: 0x{:x}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF if !self.control.contains(4) => {
                let index = self.chr_bank_low & 0b11110;
                self.cartridge
                    .write_chr_ram(address, value, ChrPage::Index8(index))
            }
            0x0000..=0x0FFF => {
                self.cartridge
                    .write_chr_ram(address, value, ChrPage::Index4(self.chr_bank_low))
            }
            0x1000..=0x1FFF => {
                self.cartridge
                    .write_chr_ram(address, value, ChrPage::Index4(self.chr_bank_high))
            }
            0x6000..=0x7FFF => self.cartridge.write_prg_ram(address, value),
            0x8000..=0xFFFF if self.shift(value) => {
                match address {
                    0x8000..=0x9FFF => self.control = self.shift,
                    0xA000..=0xBFFF => self.chr_bank_low = self.shift,
                    0xC000..=0xDFFF => self.chr_bank_high = self.shift,
                    _ => self.prg_bank = self.shift & 0b1111,
                }
                self.shift = 0b10000;
            }
            _ => {}
        }
    }

    fn get_mirroring(&self) -> crate::cartridge::Mirroring {
        match self.control & 0b11 {
            2 => Mirroring::Vertical,
            3 => Mirroring::Horizontal,
            _ => Mirroring::OneScreen,
        }
    }
}

impl Reset for SxRom {
    fn reset(&mut self) {
        self.shift = 0b10000;
        self.control = 0b11100;
        self.chr_bank_low = 0;
        self.chr_bank_high = 0;
        self.prg_bank = 0;
    }
}
