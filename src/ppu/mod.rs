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
    t_addr: AddressRegiser,
    v_addr: AddressRegiser,
    fine_x: u8,
    latch: bool,
    nmi: Option<bool>,
    cycle: u64,
    temp_cycle: u16,
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
            t_addr: AddressRegiser::default(),
            v_addr: AddressRegiser::default(),
            fine_x: 0,
            latch: false,
            nmi: None,
            cycle: 0,
            temp_cycle: 0,
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
    fn tick(&mut self, cycles: u8) {
        self.cycle += cycles as u64;
    }
}
