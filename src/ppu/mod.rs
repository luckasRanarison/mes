mod registers;

use crate::{mappers::MapperRef, ppu::registers::*};

const VRAM_SIZE: usize = 2048;
const SRAM_SIZE: usize = 256;

#[derive(Debug)]
pub struct Ppu {
    vram: [u8; VRAM_SIZE],
    oam_data: [u8; SRAM_SIZE],
    vram_buffer: u8,
    oam_addr: u8,
    latch: bool,
    ctrl: ControlRegister,
    mask: MaskRegister,
    status: StatusRegister,
    scroll: ScrollRegister,
    addr: AddressRegiser,
    mapper: MapperRef,
}

impl Ppu {
    pub fn new(mapper: MapperRef) -> Self {
        Self {
            mapper,
            vram: [0; VRAM_SIZE],
            vram_buffer: 0,
            oam_data: [0; SRAM_SIZE],
            oam_addr: 0,
            latch: false,
            ctrl: ControlRegister::default(),
            mask: MaskRegister::default(),
            status: StatusRegister::default(),
            scroll: ScrollRegister::default(),
            addr: AddressRegiser::default(),
        }
    }
}

impl Ppu {
    pub fn read_status(&mut self) -> u8 {
        let status = self.status.read();
        self.latch = false;
        self.status.update(StatusFlag::V, false);
        status
    }

    pub fn read_oam_data(&self) -> u8 {
        self.oam_data[self.oam_addr as usize]
    }

    pub fn read_data(&mut self) -> u8 {
        let address = self.addr.get();
        self.increment_vram_address();
        self.read_buffered_data(address)
    }

    pub fn write_ctrl(&mut self, value: u8) {
        self.ctrl.write(value);
    }

    pub fn write_mask(&mut self, value: u8) {
        self.mask.write(value);
    }

    pub fn write_oam_address(&mut self, value: u8) {
        self.oam_addr = value;
    }

    pub fn write_oam_data(&mut self, value: u8) {
        self.oam_data[self.oam_addr as usize] = value;
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    pub fn write_scroll(&mut self, value: u8) {
        self.scroll.write(value, &mut self.latch);
    }

    pub fn write_addr(&mut self, value: u8) {
        self.addr.write(value, &mut self.latch);
    }

    pub fn write_data(&mut self, value: u8) {
        let address = self.addr.get();

        match address {
            0x2000..=0x3EFF => todo!(),
            _ => panic!("Trying to write to invalid address: 0x{:x}", address),
        };

        self.increment_vram_address();
    }

    fn read_buffered_data(&mut self, address: u16) -> u8 {
        let buffered = self.vram_buffer;

        self.vram_buffer = match address & 0x3FFF {
            0x0000..=0x1FFF => self.mapper.borrow().read_chr(address),
            0x2000..=0x3EFF => todo!(),
            0x3F00..=0x3FFF => todo!(),
            _ => 0,
        };

        buffered
    }

    fn increment_vram_address(&mut self) {
        let offset = self.ctrl.get_vram_increment_value();
        self.addr.increment(offset);
    }
}
