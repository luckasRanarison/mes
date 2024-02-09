// https://www.nesdev.org/wiki/NROM

use super::Mapper;
use crate::{
    cartridge::{Cartridge, Mirroring},
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
            0x0000..=0x1FFF => self.cartridge.chr_rom[address as usize],
            0x6000..=0x7FFF => self.cartridge.prg_ram[address as usize - 0x6000],
            0x8000..=0xBFFF => self.cartridge.prg_rom[address as usize - 0x8000],
            0xC000..=0xFFFF => {
                let prg_rom_pages = self.cartridge.header.prg_rom_pages;
                let offset = if prg_rom_pages > 1 { 0x8000 } else { 0xC000 };
                self.cartridge.prg_rom[address as usize - offset]
            }
            _ => panic!("Trying to read from an invalid address: 0x{:x}", address),
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        if let 0x6000..=0x7FFF = address {
            self.cartridge.prg_ram[address as usize - 0x6000] = value
        };
    }

    fn get_mirroring(&self) -> Mirroring {
        self.cartridge.header.mirroring
    }
}

impl Reset for NRom {
    fn reset(&mut self) {}
}
