mod frame;
mod palette;
mod registers;

use crate::{
    bus::{Bus, PpuBus},
    mappers::MapperRef,
    ppu::{frame::Frame, registers::*},
    utils::Clock,
};

const SRAM_SIZE: usize = 256;

#[derive(Debug)]
pub struct Ppu {
    oam_data: [u8; SRAM_SIZE],
    vram_buffer: u8,
    oam_addr: u8,
    t_addr: AddressRegister,
    v_addr: AddressRegister,
    fine_x: u8,
    latch: bool,
    nmi: Option<bool>,
    cycle: u64,
    dot: u16,
    scanline: u16,
    tile_id: u8,
    tile_attr: u8,
    bg_tile_low: u8,
    bg_tile_high: u8,
    address: u16,
    frame: Frame,
    ctrl: ControlRegister,
    mask: MaskRegister,
    status: StatusRegister,
    bus: PpuBus,
}

impl Ppu {
    pub fn new(mapper: MapperRef) -> Self {
        Self {
            oam_data: [0; SRAM_SIZE],
            vram_buffer: 0,
            oam_addr: 0,
            t_addr: AddressRegister::default(),
            v_addr: AddressRegister::default(),
            fine_x: 0,
            latch: false,
            nmi: None,
            cycle: 0,
            dot: 0,
            scanline: 0,
            tile_id: 0,
            tile_attr: 0,
            bg_tile_low: 0,
            bg_tile_high: 0,
            address: 0,
            frame: Frame::default(),
            ctrl: ControlRegister::default(),
            mask: MaskRegister::default(),
            status: StatusRegister::default(),
            bus: PpuBus::new(mapper),
        }
    }
}

impl Ppu {
    pub fn read_status(&mut self) -> u8 {
        let mask = 0b1110_0000;
        let status = (self.status.read() & mask) | (self.vram_buffer & !mask);
        self.latch = false;
        self.status.update(StatusFlag::V, false);
        status
    }

    pub fn read_oam_data(&self) -> u8 {
        let address = self.oam_addr as usize;
        self.oam_data[address]
    }

    pub fn read_data(&mut self) -> u8 {
        let address = self.v_addr.get();
        let buffered = self.vram_buffer;
        let value = self.bus.read_u8(address);
        self.vram_buffer = value;
        self.increment_vram_address();

        match address & 0x3FFF {
            0x3F00..=0x3FFF => value,
            _ => buffered,
        }
    }

    pub fn write_ctrl(&mut self, value: u8) {
        let nmi_status = self.ctrl.generate_nmi();
        let vblank = self.status.is_vblank();
        self.ctrl.write(value);
        self.t_addr.set_nametable(self.ctrl.get_nametable_bits());

        if !nmi_status && self.ctrl.generate_nmi() && vblank {
            self.nmi = Some(true);
        }
    }

    pub fn write_mask(&mut self, value: u8) {
        self.mask.write(value);
    }

    pub fn write_oam_address(&mut self, value: u8) {
        self.oam_addr = value;
    }

    pub fn write_oam_data(&mut self, value: u8) {
        self.write_oam(self.oam_addr, value);
        self.oam_addr = self.oam_addr.wrapping_add(1);
    }

    pub fn write_oam(&mut self, address: u8, value: u8) {
        self.oam_data[address as usize] = value;
    }

    pub fn write_scroll(&mut self, value: u8) {
        if self.latch {
            self.t_addr.set_coarse_y(value >> 3);
            self.t_addr.set_fine_y(value & 0b111);
        } else {
            self.t_addr.set_coarse_x(value >> 3);
            self.fine_x = value & 0b111;
        }

        self.latch = !self.latch;
    }

    pub fn write_addr(&mut self, value: u8) {
        if self.latch {
            self.t_addr.set_low_byte(value);
            self.v_addr = self.t_addr;
        } else {
            self.t_addr.set_high_byte(value & 0b111111);
        }

        self.latch = !self.latch;
    }

    pub fn write_data(&mut self, value: u8) {
        let address = self.v_addr.get();
        self.bus.write_u8(address, value);
        self.increment_vram_address();
    }

    pub fn poll_nmi(&mut self) -> bool {
        self.nmi.take().is_some()
    }

    pub fn is_vblank(&self) -> bool {
        self.status.is_vblank()
    }

    pub fn get_frame_buffer(&self) -> Vec<u8> {
        todo!()
    }

    fn increment_vram_address(&mut self) {
        let offset = self.ctrl.get_vram_increment_value();
        self.v_addr.increment(offset);
    }
}

impl Clock for Ppu {
    fn tick(&mut self) {
        self.cycle += 1;

        match self.scanline {
            0..=239 => match self.dot {
                1..=255 | 321..=338 => match self.dot % 8 {
                    1 => self.address = self.v_addr.get_nametable_address(),
                    2 => self.tile_id = self.bus.read_u8(self.address),
                    3 => self.address = self.v_addr.get_attribute_address(),
                    4 => {
                        self.tile_attr = self.bus.read_u8(self.address);

                        if self.v_addr.get_coarse_y() & 0x02 != 0 {
                            self.tile_attr >>= 4;
                        }
                        if self.v_addr.get_coarse_x() & 0x02 != 0 {
                            self.tile_attr >>= 2;
                        }

                        self.tile_attr &= 0x02;
                    }
                    5 => {
                        self.address = self.ctrl.get_base_nametable_address()
                            + (self.tile_id as u16) * 16
                            + self.v_addr.get_fine_y() as u16
                            + 0
                    }
                    6 => self.bg_tile_low = self.bus.read_u8(self.address),
                    7 => {
                        self.address = self.ctrl.get_base_nametable_address()
                            + (self.tile_id as u16) * 16
                            + self.v_addr.get_fine_y() as u16
                            + 8
                    }
                    0 => {
                        self.bg_tile_high = self.bus.read_u8(self.address);
                        self.v_addr.scroll_x();
                    }
                    _ => {}
                },
                256 => {
                    self.bg_tile_high = self.bus.read_u8(self.address);
                    self.v_addr.scroll_y();
                }
                257 => self.v_addr.set_x(self.t_addr),
                339 => self.address = self.v_addr.get_nametable_address(),
                340 => self.tile_id = self.bus.read_u8(self.address),
                _ => {}
            },
            240 if self.dot == 1 => {
                self.status.update(StatusFlag::V, true);

                if self.ctrl.generate_nmi() {
                    self.nmi = Some(true)
                }
            }
            261 => match self.dot {
                1 => self.status.update(StatusFlag::V, false),
                280..=304 => self.v_addr.set_y(self.t_addr),
                _ => {}
            },
            _ => {}
        }

        self.dot += 1;

        if self.dot == 341 {
            self.dot = 0;
            self.scanline = (self.scanline + 1) % 261;
        }
    }
}
