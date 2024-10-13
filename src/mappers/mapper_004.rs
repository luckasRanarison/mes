// https://www.nesdev.org/wiki/MMC3

use super::Mapper;
use crate::{
    cartridge::{Cartridge, Mirroring},
    utils::Reset,
};

#[derive(Debug)]
pub struct TxRom {
    cartridge: Cartridge,
    mirroring: Mirroring,
}

impl TxRom {
    pub fn new(cartridge: Cartridge) -> Self {
        let mirroring = cartridge.header.mirroring;

        Self {
            cartridge,
            mirroring,
        }
    }
}

impl Mapper for TxRom {
    fn read(&self, address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, address: u16, value: u8) {
        todo!()
    }

    fn get_mirroring(&self) -> Mirroring {
        todo!()
    }
}

impl Reset for TxRom {
    fn reset(&mut self) {}
}
