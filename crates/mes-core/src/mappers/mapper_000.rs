// https://www.nesdev.org/wiki/NROM

use super::Mapper;
use crate::{
    rom::{
        cartridge::{Cartridge, ChrPage, PrgPage},
        Mirroring,
    },
    utils::Reset,
};

#[derive(Debug)]
pub struct NRom {
    cartridge: Cartridge,
}

impl NRom {
    pub fn new(cartridge: Cartridge) -> Self {
        Self { cartridge }
    }
}

impl Mapper for NRom {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.cartridge.read_chr(address, ChrPage::Index8(0)),
            0x4020..=0x5FFF => 0,
            0x6000..=0x7FFF => self.cartridge.read_prg_ram(address),
            0x8000..=0xBFFF => self.cartridge.read_prg_rom(address, PrgPage::Index16(0)),
            0xC000..=0xFFFF => self.cartridge.read_prg_rom(address, PrgPage::Last16),
            _ => panic!("Trying to read from an invalid address: 0x{:x}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        if let 0x6000..=0x7FFF = address {
            self.cartridge.write_prg_ram(address, value);
        }
    }

    fn get_mirroring(&self) -> Mirroring {
        self.cartridge.header.mirroring
    }
}

impl Reset for NRom {
    fn reset(&mut self) {}
}
