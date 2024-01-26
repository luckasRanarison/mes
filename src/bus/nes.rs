use crate::{
    cartridge::{error::LoadError, Cartridge},
    mappers::{get_mapper, Mapper, MapperRef},
    ppu::Ppu,
};

use super::Bus;

#[derive(Debug)]
pub struct NesBus {
    ram: [u8; 2048],
    ppu: Ppu,
    mapper: MapperRef,
}

impl NesBus {
    pub fn new(cartridge: Cartridge) -> Result<Self, LoadError> {
        let mapper = get_mapper(cartridge).ok_or(LoadError::UnsupportedMapper)?;
        let ppu = Ppu::new(mapper.clone());
        let ram = [0; 2048];

        Ok(NesBus { ram, ppu, mapper })
    }

    fn read_ram(&self, address: u16) -> u8 {
        self.ram[address as usize % 0x8000]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.ram[address as usize % 0x8000] = value;
    }
}

impl Bus for NesBus {
    fn read_u8(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.read_ram(address),
            0x2002 => self.ppu.read_status(),
            0x2004 => self.ppu.read_oam_data(),
            0x2007 => self.ppu.read_data(),
            0x2008..=0x3FFF => self.read_u8(address & 0x2007),
            0x4020..=0xFFFF => self.mapper.read(address),
            _ => panic!("Trying to read from write-only address: 0x{:x}", address),
        }
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.write_ram(address, value),
            0x2000 => self.ppu.write_ctrl(value),
            0x2001 => self.ppu.write_mask(value),
            0x2003 => self.ppu.write_oam_address(value),
            0x2004 => self.ppu.write_oam_data(value),
            0x2005 => self.ppu.write_scroll(value),
            0x2006 => self.ppu.write_addr(value),
            0x2007 => self.ppu.write_data(value),
            0x2008..=0x3FFF => self.write_u8(address & 0x2007, value),
            0x4000..=0x4013 | 0x4015 => {}
            0x4014 => {}
            0x4020..=0x5FFF => {}
            0x6000..=0xFFFF => self.mapper.write(address, value),
            _ => panic!("Trying to write to read-only address: 0x{:x}", address),
        }
    }
}
