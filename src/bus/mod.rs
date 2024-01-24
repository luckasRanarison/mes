use crate::{
    cartridge::{error::LoadError, Cartridge},
    mappers::{get_mapper, MapperRef},
};
use std::fmt::Debug;

pub trait Bus: Debug {
    fn read_u8(&self, address: u16) -> u8;
    fn read_u16(&self, address: u16) -> u16;
    fn write_u8(&mut self, address: u16, value: u8);
}

#[derive(Debug)]
pub struct NesBus {
    ram: [u8; 2048],
    mapper: MapperRef,
}

impl NesBus {
    pub fn new(cartridge: Cartridge) -> Result<Self, LoadError> {
        let mapper = get_mapper(cartridge).ok_or(LoadError::UnsupportedMapper)?;
        let ram = [0; 2048];

        Ok(NesBus { mapper, ram })
    }

    fn read_ram(&self, address: u16) -> u8 {
        self.ram[address as usize % 0x8000]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.ram[address as usize % 0x8000] = value;
    }
}

impl Bus for NesBus {
    fn read_u8(&self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.read_ram(address),
            0x2000..=0x401F => todo!(),
            0x4020..=0x5FFF => todo!(),
            0x6000..=0xFFFF => self.mapper.borrow().read(address),
        }
    }

    fn read_u16(&self, address: u16) -> u16 {
        let low = self.read_u8(address);
        let high = self.read_u8(address.wrapping_add(1));
        u16::from_le_bytes([low, high])
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.write_ram(address, value),
            0x2000..=0x401F => {}
            0x4020..=0x5FFF => {}
            0x6000..=0xFFFF => self.mapper.borrow_mut().write(address, value),
        }
    }
}
