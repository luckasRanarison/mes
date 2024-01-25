mod register;

use crate::mappers::MapperRef;

const VRAM_SIZE: usize = 16384;
const SRAM_SIZE: usize = 256;

#[derive(Debug)]
pub struct Ppu {
    mapper: MapperRef,
    vram: [u8; VRAM_SIZE],
    sram: [u8; SRAM_SIZE],
}

impl Ppu {
    pub fn new(mapper: MapperRef) -> Self {
        Self {
            mapper,
            vram: [0; VRAM_SIZE],
            sram: [0; SRAM_SIZE],
        }
    }
}
