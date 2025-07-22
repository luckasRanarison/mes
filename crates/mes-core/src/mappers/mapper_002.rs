// https://www.nesdev.org/wiki/UxROM

use super::Mapper;
use crate::{
    cartridge::{Cartridge, ChrPage, Mirroring, PrgPage},
    utils::Reset,
};

#[derive(Debug)]
pub struct UxRom {
    cartridge: Cartridge,
    prg_bank: u8,
}

impl UxRom {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            prg_bank: 0,
        }
    }
}

impl Mapper for UxRom {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.cartridge.read_chr(address, ChrPage::Index8(0)),
            0x4020..=0x5FFF => 0,
            0x6000..=0x7FFF => self.cartridge.read_prg_ram(address),
            0x8000..=0xBFFF => self
                .cartridge
                .read_prg_rom(address, PrgPage::Index16(self.prg_bank)),
            0xC000..=0xFFFF => self.cartridge.read_prg_rom(address, PrgPage::Last16),
            _ => panic!("Trying to read from an invalid address: 0x{address:x}"),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self
                .cartridge
                .write_chr_ram(address, value, ChrPage::Index8(0)),
            0x6000..=0x7FFF => self.cartridge.write_prg_ram(address, value),
            0x8000..=0xFFFF => self.prg_bank = value & 0b1111,
            _ => {}
        }
    }

    fn get_mirroring(&self) -> Mirroring {
        self.cartridge.header.mirroring
    }
}

impl Reset for UxRom {
    fn reset(&mut self) {
        self.prg_bank = 0;
    }
}
