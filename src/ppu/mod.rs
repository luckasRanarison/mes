// https://www.nesdev.org/wiki/PPU_programmer_reference
// https://www.nesdev.org/wiki/PPU_rendering
// https://www.nesdev.org/wiki/PPU_sprite_evaluation

mod frame;
mod palette;
mod registers;

use crate::{
    bus::{Bus, PpuBus},
    mappers::MapperRef,
    ppu::{frame::Frame, palette::NES_PALETTE, registers::*},
    utils::{BitFlag, BitPlane, Clock},
};

const PRIMARY_OAM_SIZE: usize = 256;
const SECONDARY_OAM_SIZE: usize = 32;

#[derive(Debug)]
pub struct Ppu {
    bus: PpuBus,
    primary_oam: [u8; PRIMARY_OAM_SIZE],
    secondary_oam: [u8; SECONDARY_OAM_SIZE],
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
    frame: Frame,
    odd_frame: bool,
    bg_address: u16,
    bg_pattern_id: u8,
    bg_palette_id: u8,
    bg_pattern: BitPlane<u8>,
    bg_pattern_shift: BitPlane<u16>,
    bg_palette_shift: BitPlane<u16>,
    oam_buffer: u8,
    primary_oam_index: u8,
    secondary_oam_index: u8,
    oam_index_overflow: bool,
    sp_buffer: [u8; 4],
    sp_address: u16,
    sp_pattern_shift: [BitPlane<u8>; 8],
    sp_attribute_shift: [u8; 8],
    sp_offset_shift: [u8; 8],
}

impl Ppu {
    pub fn new(mapper: MapperRef) -> Self {
        Self {
            bus: PpuBus::new(mapper),
            primary_oam: [0; PRIMARY_OAM_SIZE],
            secondary_oam: [0; SECONDARY_OAM_SIZE],
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
            frame: Frame::default(),
            odd_frame: false,
            bg_address: 0,
            bg_pattern_id: 0,
            bg_palette_id: 0,
            bg_pattern: BitPlane::default(),
            bg_pattern_shift: BitPlane::default(),
            bg_palette_shift: BitPlane::default(),
            oam_buffer: 0,
            primary_oam_index: 0,
            secondary_oam_index: 0,
            oam_index_overflow: false,
            sp_buffer: [0; 4],
            sp_address: 0,
            sp_pattern_shift: [BitPlane::default(); 8],
            sp_attribute_shift: [0; 8],
            sp_offset_shift: [0; 8],
        }
    }
}

impl Ppu {
    pub fn read_status(&mut self) -> u8 {
        let mask = 0b1110_0000;
        let status = (self.status.read() & mask) | (self.vram_buffer & !mask);
        self.latch = false;
        self.status.clear_vblank();
        status
    }

    pub fn read_oam_data(&self) -> u8 {
        let address = self.oam_addr as usize;
        self.primary_oam[address]
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
        self.primary_oam[address as usize] = value;
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
        self.bg_pattern_shift.low =
            (self.bg_pattern_shift.low & 0xFF00) | self.bg_pattern.low as u16;
        self.bg_pattern_shift.high =
            (self.bg_pattern_shift.high & 0xFF00) | self.bg_pattern.high as u16;
        self.bg_palette_shift.low =
            (self.bg_palette_shift.low & 0xFF00) | (self.bg_palette_id as u16 & 0b01) * 0xFF;
        self.bg_palette_shift.high =
            (self.bg_palette_shift.high & 0xFF00) | (self.bg_palette_id as u16 & 0b10) * 0xFF;
    }

    fn update_shifters(&mut self) {
        self.bg_pattern_shift.low <<= 1;
        self.bg_pattern_shift.high <<= 1;
        self.bg_palette_shift.low <<= 1;
        self.bg_palette_shift.high <<= 1;
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
            337 | 339 => self.bg_address = self.v_addr.get_nametable_address(),
            338 | 340 => {
                self.bg_pattern_id = self.bus.read_u8(self.bg_address);

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
                self.bg_address = self.v_addr.get_nametable_address();
            }
            2 => self.bg_pattern_id = self.bus.read_u8(self.bg_address),
            3 => self.bg_address = self.v_addr.get_attribute_address(),
            4 => {
                self.bg_palette_id = self.bus.read_u8(self.bg_address);

                if self.v_addr.get_coarse_y() & 2 > 0 {
                    self.bg_palette_id >>= 4;
                }
                if self.v_addr.get_coarse_x() & 2 > 0 {
                    self.bg_palette_id >>= 2;
                }

                self.bg_palette_id &= 0b11;
            }
            5 => {
                let nametable = self.ctrl.get_background_pattern_table_address();
                let fine_y = self.v_addr.get_fine_y() as u16;
                self.bg_address = nametable + (self.bg_pattern_id as u16) * 16 + fine_y + 0;
            }
            6 => self.bg_pattern.low = self.bus.read_u8(self.bg_address),
            7 => self.bg_address += 8,
            0 => {
                self.bg_pattern.high = self.bus.read_u8(self.bg_address);

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

    fn tick_sprite(&mut self) {
        if self.dot == 1 {
            self.secondary_oam_index = 0;
        }

        if self.dot == 65 {
            self.primary_oam_index = 0;
            self.secondary_oam_index = 0;
        }

        match self.dot {
            1..=64 if self.scanline != 261 => {
                if self.dot % 2 == 0 {
                    self.secondary_oam[self.secondary_oam_index as usize] = self.oam_buffer;
                    self.secondary_oam_index += 1;
                } else {
                    self.oam_buffer = 0xFF;
                }
            }
            65..=256 if self.scanline != 261 => {
                if self.dot % 2 == 1 {
                    self.oam_buffer = self.primary_oam[self.primary_oam_index as usize];
                } else {
                    self.evaluate_sprite();
                }
            }
            257..=320 => self.fetch_sprite(),
            _ => {}
        }
    }

    fn evaluate_sprite(&mut self) {
        if self.secondary_oam_index < 32 {
            let sprite_y = self.oam_buffer;
            let sprite_height = self.ctrl.get_sprite_height() as i16;
            let offset = self.scanline as i16 - sprite_y as i16;

            if offset >= 0 && offset < sprite_height {
                let i = self.secondary_oam_index as usize;
                let j = self.primary_oam_index as usize;

                self.secondary_oam[i..i + 4].copy_from_slice(&self.primary_oam[j..j + 4]);
                self.secondary_oam_index += 4;
            }
        } else {
            self.status.set_sprite_overflow();
        }

        let (index, overflow) = self.primary_oam_index.overflowing_add(4);
        self.primary_oam_index = index;
        self.oam_index_overflow |= overflow; // TODO: sprite overflow bug
    }

    fn fetch_sprite(&mut self) {
        if self.dot == 257 {
            self.secondary_oam_index = 0;
        }

        let cycle = (self.dot - 257) % 8;
        let index = (self.dot as usize - 257) / 8;
        let oam_value = self.secondary_oam[self.secondary_oam_index as usize];
        let y = self.sp_buffer[0];

        match cycle {
            0..=3 => self.sp_buffer[cycle as usize] = oam_value,
            4 if y != 0xFF => self.sp_address = self.get_sprite_pattern_address(),
            5 if y != 0xFF => self.sp_pattern_shift[index].low = self.bus.read_u8(self.sp_address),
            6 if y != 0xFF => self.sp_address += 8,
            7 if y != 0xFF => {
                self.sp_pattern_shift[index].high = self.bus.read_u8(self.sp_address);
                self.sp_attribute_shift[index] = self.sp_buffer[2];
                self.sp_offset_shift[index] = self.sp_buffer[3]; // X coordinate
            }
            _ => {}
        }

        if cycle < 4 && self.secondary_oam_index < 31 {
            self.secondary_oam_index += 1;
        }
    }

    fn get_sprite_pattern_address(&self) -> u16 {
        let sprite_y = self.sp_buffer[0];
        let tile = self.sp_buffer[1];
        let attribute = self.sp_buffer[2];
        let height = self.ctrl.get_sprite_height();
        let pattern_table = self.ctrl.get_sprite_pattern_table_address();
        let base_address = match height {
            8 => pattern_table + 16 * tile as u16,
            _ => 0x1000 * tile.get(0) as u16 + (16 * (tile >> 1) as u16),
        };
        let flip_vertical = attribute.contains(7);
        let y = self.scanline - sprite_y as u16;
        let address = base_address + y;

        if flip_vertical {
            7 - address
        } else {
            address
        }
    }

    fn render_pixel(&mut self) {
        let x = self.dot.saturating_sub(1) as usize; // one dot offset
        let y = self.scanline as usize;

        if x < 256 && y < 240 {
            let (bg_pixel, bg_palette) = self.get_background_pixel();
            let (sp_pixel, sp_palette, sp_priority) = self.get_sprite_pixel();
            let (pixel, palette) = match (bg_pixel, sp_pixel, sp_priority) {
                (_, 0, _) => (bg_pixel, bg_palette),
                (_, _, true) => (sp_pixel, sp_palette),
                _ => (bg_pixel, bg_pixel),
            };
            let palette_address = 0x3F00 + (4 * palette as u16 + pixel as u16);
            let palette_index = self.bus.read_u8(palette_address);
            let rgb = NES_PALETTE[palette_index as usize];

            self.frame.set_pixel(x, y, rgb);
        }
    }

    fn get_background_pixel(&self) -> (u8, u8) {
        let offset = 15 - self.fine_x as u16;
        let low_pixel = self.bg_pattern_shift.low.get(offset);
        let high_pixel = self.bg_pattern_shift.high.get(offset);
        let pixel = (high_pixel << 1) + low_pixel;

        let low_palette = self.bg_palette_shift.low.get(offset);
        let high_palette = self.bg_palette_shift.high.get(offset);
        let palette = (high_palette << 1) + low_palette;

        (pixel as u8, palette as u8)
    }

    fn get_sprite_pixel(&mut self) -> (u8, u8, bool) {
        for i in 0..8 {
            if self.sp_offset_shift[i] == 0 {
                let attribute = self.sp_attribute_shift[i];
                let sp_priority = !attribute.contains(5);
                let palette = (attribute & 0b11) * 4;
                let horizontal_flip = attribute.contains(6);
                let pixel_index = if horizontal_flip { 0 } else { 7 };
                let pixel_low = self.sp_pattern_shift[i].low.get(pixel_index);
                let pixel_high = self.sp_pattern_shift[i].high.get(pixel_index);
                let pixel = (pixel_high << 1) | pixel_low;

                if horizontal_flip {
                    self.sp_pattern_shift[i].low >>= 1;
                    self.sp_pattern_shift[i].high >>= 1;
                } else {
                    self.sp_pattern_shift[i].low <<= 1;
                    self.sp_pattern_shift[i].high <<= 1;
                }

                return (pixel, palette, sp_priority);
            } else {
                self.sp_offset_shift[i] -= 1;
            }
        }

        (0, 0, false)
    }
}

impl Clock for Ppu {
    fn tick(&mut self) {
        self.cycle += 1;

        match self.scanline {
            0..=239 | 261 => {
                if self.scanline == 261 && self.dot == 1 {
                    self.status.clear();
                }

                self.tick_sprite();
                self.tick_background();
            }
            241 if self.dot == 1 => {
                self.status.set_vblank();
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
        assert_eq!(ppu.primary_oam[0x0F], 0x20);
        assert_eq!(ppu.primary_oam[0x10], 0x21);
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
