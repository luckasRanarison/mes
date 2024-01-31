mod palette;
mod registers;

use crate::{
    bus::{Bus, PpuBus},
    mappers::MapperRef,
    ppu::registers::*,
    utils::{BitFlag, Clock},
};

use self::palette::NES_PALETTE;

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
    odd_frame: bool,
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
            odd_frame: false,
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

    pub fn get_frame_buffer(&self) -> &[u8] {
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
            0..=239 | 261 => match self.dot {
                1..=256 | 321..=336 => match self.dot % 8 {
                    1 => {
                        // fetch NT address

                        if self.dot == 261 {
                            self.status.clear();
                        }
                    }
                    2 => {} // read NT
                    3 => {} // fetch AT address
                    4 => {} // read AT
                    5 => {} // Read BG lsbits address
                    6 => {} // Read BG lsbits
                    7 => {} // Read BG msbits address
                    0 => {
                        // Read BG msbits

                        if self.dot == 256 {
                            self.v_addr.scroll_y();
                        }

                        self.v_addr.scroll_x();
                    }
                    _ => {}
                },
                257 => self.v_addr.set_x(self.t_addr),
                280..=304 if self.scanline == 261 => self.v_addr.set_y(self.t_addr),
                337 | 339 => {} // fetch NT address
                338 | 340 => {
                    // read NT

                    if self.scanline == 261 && self.odd_frame {
                        self.dot = 0;
                    }
                }
                _ => {}
            },
            241 if self.dot == 1 => {
                self.status.update(StatusFlag::V, true);
                self.nmi = self.ctrl.generate_nmi().then_some(true);
            }
            _ => {}
        }

        self.dot += 1;

        if self.dot > 340 {
            self.dot = 0;
            self.scanline += 1;

            if self.scanline > 261 {
                self.scanline = 0;
                self.odd_frame = !self.odd_frame;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Ppu;
    use crate::{cartridge::create_cartridge_mock, get_mapper};

    #[test]
    fn test_ppu_oam_read_write() {
        let mapper = get_mapper(create_cartridge_mock()).unwrap();
        let mut ppu = Ppu::new(mapper);

        ppu.write_oam_address(0x0F);
        ppu.write_oam_data(0x20);
        ppu.write_oam_data(0x21);

        assert_eq!(ppu.oam_addr, 0x11);
        assert_eq!(ppu.oam_data[0x0F], 0x20);
        assert_eq!(ppu.oam_data[0x10], 0x21);
    }

    #[test]
    fn test_ppu_data_read_write() {
        let mapper = get_mapper(create_cartridge_mock()).unwrap();
        let mut ppu = Ppu::new(mapper);

        ppu.write_addr(0x20);
        ppu.write_addr(0x50);

        assert_eq!(ppu.v_addr.get(), 0x2050);
        assert_eq!(ppu.latch, false);

        ppu.write_data(0x45);

        assert_eq!(ppu.v_addr.get(), 0x2051);

        ppu.write_addr(0x20);
        ppu.write_addr(0x50);
        let buffer = ppu.read_data();
        let data = ppu.read_data();

        assert_eq!(ppu.v_addr.get(), 0x2052);
        assert_eq!(buffer, 0x00);
        assert_eq!(data, 0x45);
    }
}
