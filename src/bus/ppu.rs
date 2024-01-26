use crate::{bus::Bus, mappers::MapperRef};

const VRAM_SIZE: usize = 2048;
const PALETTE_SIZE: usize = 32;

#[derive(Debug)]
pub struct PpuBus {
    vram: [u8; VRAM_SIZE],
    palette: [u8; PALETTE_SIZE],
    mapper: MapperRef,
}

impl Bus for PpuBus {
    fn read_u8(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.mapper.borrow().read(address),
            0x2000..=0x3EFF => todo!(),
            0x3F00..=0x3FFF => self.read_palette(address),
            _ => self.read_u8(address & 0x3FFF),
        }
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.mapper.borrow_mut().write(address, value),
            0x2000..=0x3EFF => todo!(),
            0x3F00..=0x3FFF => self.write_palette(address, value),
            _ => panic!("Trying to write to invalid address: 0x{:x}", address),
        }
    }
}

impl PpuBus {
    pub fn new(mapper: MapperRef) -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            palette: [0; PALETTE_SIZE],
            mapper,
        }
    }

    fn read_palette(&self, address: u16) -> u8 {
        let address = address as usize & (PALETTE_SIZE - 1);
        self.palette[address]
    }

    fn write_palette(&mut self, address: u16, value: u8) {
        let address = address as usize & (PALETTE_SIZE - 1);
        self.palette[address] = value;
    }
}
