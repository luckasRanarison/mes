// https://www.nesdev.org/wiki/INES_Mapper_003

use super::Mapper;
use crate::{
    cartridge::{Cartridge, ChrPage, Mirroring, PrgPage},
    utils::Reset,
};

#[derive(Debug)]
pub struct CnRom {
    cartridge: Cartridge,
    chr_bank: u8,
}

impl CnRom {
    pub fn new(cartridge: Cartridge) -> Self {
        Self {
            cartridge,
            chr_bank: 0,
        }
    }
}

impl Mapper for CnRom {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self
                .cartridge
                .read_chr(address, ChrPage::Index8(self.chr_bank)),
            0x4020..=0x5FFF => 0,
            0x6000..=0x7FFF => self.cartridge.read_prg_ram(address),
            0x8000..=0xBFFF => self.cartridge.read_prg_rom(address, PrgPage::Index16(0)),
            0xC000..=0xFFFF => self.cartridge.read_prg_rom(address, PrgPage::Last16),
            _ => panic!("Trying to read from an invalid address: 0x{address:x}"),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => {
                self.cartridge
                    .write_chr_ram(address, value, ChrPage::Index8(self.chr_bank))
            }
            0x6000..=0x7FFF => self.cartridge.write_prg_ram(address, value),
            0x8000..=0xFFFF => self.chr_bank = value,
            _ => {}
        }
    }

    fn get_mirroring(&self) -> Mirroring {
        self.cartridge.header.mirroring
    }
}

impl Reset for CnRom {
    fn reset(&mut self) {
        self.chr_bank = 0;
    }
}
