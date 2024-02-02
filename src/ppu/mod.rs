// https://www.nesdev.org/wiki/PPU_programmer_reference
// https://www.nesdev.org/wiki/PPU_rendering

mod frame;
mod palette;
mod registers;

use crate::{
    bus::{Bus, PpuBus},
    mappers::MapperRef,
    ppu::{frame::Frame, palette::NES_PALETTE, registers::*},
    utils::{BitPlane, Clock},
};

const OAM_SIZE: usize = 256;

#[derive(Debug)]
pub struct Ppu {
    oam_data: [u8; OAM_SIZE],
    vram_buffer: u8,
    oam_addr: u8,
    ctrl: ControlRegister,
    mask: MaskRegister,
    status: StatusRegister,
    t_addr: AddressRegister,
    v_addr: AddressRegister,
    fine_x: u8,
    latch: bool,
    nmi: Option<bool>,
    cycle: u64,
    dot: u16,
    scanline: u16,
    address: u16,
    tile_id: u8,
    tile_attribute: u8,
    bg_tile: BitPlane<u8>,
    bg_shifter: BitPlane<u16>,
    pal_shifter: BitPlane<u8>,
    odd_frame: bool,
    frame: Frame,
    bus: PpuBus,
}

impl Ppu {
    pub fn new(mapper: MapperRef) -> Self {
        Self {
            oam_data: [0; OAM_SIZE],
            vram_buffer: 0,
            oam_addr: 0,
            ctrl: ControlRegister::default(),
            mask: MaskRegister::default(),
            status: StatusRegister::default(),
            t_addr: AddressRegister::default(),
            v_addr: AddressRegister::default(),
            fine_x: 0,
            latch: false,
            nmi: None,
            cycle: 0,
            dot: 0,
            scanline: 0,
            address: 0,
            tile_id: 0,
            tile_attribute: 0,
            bg_tile: BitPlane::default(),
            bg_shifter: BitPlane::default(),
            pal_shifter: BitPlane::default(),
            odd_frame: false,
            frame: Frame::default(),
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
            self.t_addr.set_high_byte(value & 0x3F);
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
        self.frame.get_buffer()
    }

    fn increment_vram_address(&mut self) {
        let offset = self.ctrl.get_vram_increment_value();
        self.v_addr.increment(offset);
    }

    fn load_shifters(&mut self) {
        self.bg_shifter.low = (self.bg_shifter.low & 0xFF00) | self.bg_tile.low as u16;
        self.bg_shifter.high = (self.bg_shifter.high & 0xFF00) | self.bg_tile.high as u16;
        self.pal_shifter.low = self.tile_attribute & 1;
        self.pal_shifter.high = (self.tile_attribute >> 1) & 1;
    }

    fn update_shifters(&mut self) {
        self.bg_shifter.low <<= 1;
        self.bg_shifter.high <<= 1;
        self.pal_shifter.low = (self.pal_shifter.low << 1) | (self.tile_attribute & 1);
        self.pal_shifter.high = (self.pal_shifter.high << 1) | ((self.tile_attribute >> 1) & 1);
    }

    fn tick_background(&mut self) {
        match self.dot {
            1..=256 | 321..=336 => {
                self.update_shifters();
                self.on_background_dot()
            }
            257 => {
                if self.mask.is_rendering() {
                    self.v_addr.set_x(self.t_addr)
                }
            }
            280..=304 => {
                if self.scanline == 261 && self.mask.is_rendering() {
                    self.v_addr.set_y(self.t_addr)
                }
            }
            337 | 339 => self.address = self.v_addr.get_nametable_address(),
            338 | 340 => {
                self.tile_id = self.bus.read_u8(self.address);

                if self.dot == 340 && self.scanline == 261 && self.odd_frame {
                    self.dot += 1;
                }
            }
            _ => {} // garbage NT
        };
    }

    fn on_background_dot(&mut self) {
        match self.dot % 8 {
            1 => {
                self.load_shifters();
                self.address = self.v_addr.get_nametable_address();

                if self.dot == 261 {
                    self.status.clear();
                }
            }
            2 => self.tile_id = self.bus.read_u8(self.address),
            3 => self.address = self.v_addr.get_attribute_address(),
            4 => {
                self.tile_attribute = self.bus.read_u8(self.address);

                if self.v_addr.get_coarse_y() & 2 > 0 {
                    self.tile_attribute >>= 4;
                }
                if self.v_addr.get_coarse_x() & 2 > 0 {
                    self.tile_attribute >>= 2;
                }

                self.tile_attribute &= 0b11;
            }
            5 => {
                let nametable = self.ctrl.get_background_pattern_table_address();
                let fine_y = self.v_addr.get_fine_y() as u16;
                self.address = nametable + (self.tile_id as u16) * 16 + fine_y + 0;
            }
            6 => self.bg_tile.low = self.bus.read_u8(self.address),
            7 => self.address += 8,
            0 => {
                self.bg_tile.high = self.bus.read_u8(self.address);

                if self.mask.is_rendering() {
                    if self.dot == 256 {
                        self.v_addr.scroll_y();
                    }

                    self.v_addr.scroll_x();
                }
            }
            _ => unreachable!(),
        }
    }

    fn render_pixel(&mut self) {
        let x = self.dot as usize;
        let y = self.scanline as usize;

        if x < 256 && y < 240 {
            let mask = 0x8000 >> self.fine_x;
            let low_plane = self.bg_shifter.low;
            let low_pixel = if low_plane & mask > 0 { 1 } else { 0 };
            let high_plane = self.bg_shifter.high;
            let high_pixel = if high_plane & mask > 0 { 2 } else { 0 };
            let result = low_pixel | high_pixel;
            let palette = (self.pal_shifter.high & 1) << 2 | self.pal_shifter.low & 1;
            let palette_address = 0x3F00 + (4 * palette as u16 + result as u16);
            let palette_index = self.bus.read_u8(palette_address);
            let rgb = NES_PALETTE[palette_index as usize];

            self.frame.set_pixel(x.saturating_sub(1), y, rgb);
        }
    }
}

impl Clock for Ppu {
    fn tick(&mut self) {
        self.cycle += 1;

        match self.scanline {
            0..=239 | 261 => self.tick_background(),
            241 if self.dot == 1 => {
                self.status.update(StatusFlag::V, true);
                self.nmi = self.ctrl.generate_nmi().then_some(true);
            }
            _ => {}
        }

        // WIP
        if self.mask.is_rendering() {
            self.render_pixel();
        }

        self.dot += 1;

        if self.dot > 340 {
            self.dot %= 341;
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
    use crate::{cartridge::create_cartridge_mock, mappers::get_mapper};

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
