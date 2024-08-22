use crate::{
    bus::Bus,
    cartridge::Mirroring,
    mappers::{Mapper, MapperChip},
    utils::{Clock, Reset},
};

const VRAM_SIZE: usize = 2048;
const PALETTE_SIZE: usize = 32;

#[derive(Debug)]
pub struct PpuBus {
    vram: [u8; VRAM_SIZE],
    palette: [u8; PALETTE_SIZE],
    mapper: MapperChip,
}

impl Bus for PpuBus {
    fn read_u8(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.mapper.read(address),
            0x2000..=0x3EFF => self.read_vram(address),
            0x3F00..=0x3FFF => self.read_palette(address),
            _ => self.read_u8(address & 0x3FFF),
        }
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.mapper.write(address, value),
            0x2000..=0x3EFF => self.write_vram(address, value),
            0x3F00..=0x3FFF => self.write_palette(address, value),
            _ => self.write_u8(address & 0x3FFF, value),
        }
    }
}

impl PpuBus {
    pub fn new(mapper: MapperChip) -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            palette: [0; PALETTE_SIZE],
            mapper,
        }
    }

    pub fn set_mapper(&mut self, mapper: MapperChip) {
        self.mapper = mapper;
    }

    fn read_palette(&self, address: u16) -> u8 {
        let address = address as usize & (PALETTE_SIZE - 1);
        let address = if address == 0x10 { 0 } else { address };
        self.palette[address]
    }

    fn write_palette(&mut self, address: u16, value: u8) {
        let address = address as usize & (PALETTE_SIZE - 1);
        let address = if address == 0x10 { 0 } else { address };
        self.palette[address] = value;
    }

    fn read_vram(&self, address: u16) -> u8 {
        let vram_address = self.get_vram_address(address);
        self.vram[vram_address as usize]
    }

    fn write_vram(&mut self, address: u16, value: u8) {
        let vram_address = self.get_vram_address(address);
        self.vram[vram_address as usize] = value;
    }

    fn get_vram_address(&self, address: u16) -> u16 {
        let relative_address = address & 0x0FFF;
        let nametable_id = relative_address / 0x400;
        let mirroring = self.mapper.get_mirroring();
        let mirrored_address = match mirroring {
            Mirroring::Horizontal if matches!(nametable_id, 1 | 2) => relative_address - 0x400,
            Mirroring::OneScreen => relative_address & 0x03FF,
            _ => relative_address,
        };

        mirrored_address & 0x07FF
    }
}

impl Clock for PpuBus {}

impl Reset for PpuBus {
    fn reset(&mut self) {
        self.vram = [0; VRAM_SIZE];
        self.palette = [0; PALETTE_SIZE];
        self.mapper.reset();
    }
}

#[cfg(test)]
mod tests {
    use super::PpuBus;
    use crate::{bus::Bus, mappers::create_mapper_mock};

    #[test]
    fn test_ppu_bus_read_write() {
        let mapper = create_mapper_mock();
        let mut bus = PpuBus::new(mapper);

        bus.write_u8(0x2000, 0x20);
        bus.write_u8(0x2400, 0x60);

        assert_eq!(bus.read_u8(0x2000), 0x20);
        assert_eq!(bus.read_u8(0x2800), 0x20);
        assert_eq!(bus.read_u8(0x2400), 0x60);
        assert_eq!(bus.read_u8(0x2C00), 0x60);
        assert_eq!(bus.vram[0x000], 0x20);
        assert_eq!(bus.vram[0x400], 0x60);

        bus.write_u8(0x3F00, 0x10);

        assert_eq!(bus.read_u8(0x3F00), 0x10);
        assert_eq!(bus.read_u8(0x3F20), 0x10);
        assert_eq!(bus.palette[0], 0x10);
    }
}
