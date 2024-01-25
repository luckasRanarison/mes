mod registers;

use crate::{mappers::MapperRef, ppu::registers::*};

const VRAM_SIZE: usize = 16384;
const SRAM_SIZE: usize = 256;

#[derive(Debug)]
pub struct Ppu {
    mapper: MapperRef,
    vram: [u8; VRAM_SIZE],
    oam_data: [u8; SRAM_SIZE],
    oam_addr: u8,
    address_latch: bool,
    ppu_ctrl: ControlRegister,
    ppu_mask: MaskRegister,
    ppu_status: StatusRegister,
    ppu_scroll: ScrollRegister,
    ppu_addr: AddressRegiser,
}

impl Ppu {
    pub fn new(mapper: MapperRef) -> Self {
        Self {
            mapper,
            vram: [0; VRAM_SIZE],
            oam_data: [0; SRAM_SIZE],
            oam_addr: 0,
            address_latch: false,
            ppu_ctrl: ControlRegister::default(),
            ppu_mask: MaskRegister::default(),
            ppu_status: StatusRegister::default(),
            ppu_scroll: ScrollRegister::default(),
            ppu_addr: AddressRegiser::default(),
        }
    }
}
