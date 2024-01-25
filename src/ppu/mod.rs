mod register;

use self::register::{
    AddressRegiser, ControlRegister, DataRegister, MaskRegister, OamAddressRegister,
    OamDataRegister, ScrollRegister, StatusRegister,
};
use crate::mappers::MapperRef;

const VRAM_SIZE: usize = 16384;
const SRAM_SIZE: usize = 256;

#[derive(Debug)]
pub struct Ppu {
    mapper: MapperRef,
    vram: [u8; VRAM_SIZE],
    sram: [u8; SRAM_SIZE],
    latch: bool,
    ppu_ctrl: ControlRegister,
    ppu_mask: MaskRegister,
    ppu_status: StatusRegister,
    oam_addr: OamAddressRegister,
    oam_data: OamDataRegister,
    ppu_scroll: ScrollRegister,
    ppu_addr: AddressRegiser,
    ppu_data: DataRegister,
}

impl Ppu {
    pub fn new(mapper: MapperRef) -> Self {
        Self {
            mapper,
            vram: [0; VRAM_SIZE],
            sram: [0; SRAM_SIZE],
            latch: false,
            ppu_ctrl: ControlRegister::default(),
            ppu_mask: MaskRegister::default(),
            ppu_status: StatusRegister::default(),
            oam_addr: OamAddressRegister::default(),
            oam_data: OamDataRegister::default(),
            ppu_scroll: ScrollRegister::default(),
            ppu_addr: AddressRegiser::default(),
            ppu_data: DataRegister::default(),
        }
    }
}
