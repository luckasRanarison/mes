// https://www.nesdev.org/wiki/PPU_programmer_reference
// https://www.nesdev.org/wiki/PPU_rendering
// https://www.nesdev.org/wiki/PPU_sprite_evaluation

mod registers;

use crate::{
    bus::{Bus, PpuBus},
    mappers::MapperRef,
    ppu::registers::*,
    utils::{BitFlag, BitPlane, Clock, Reset},
};

const SCREEN_WIDTH: usize = 256;
const SCREEN_HEIGHT: usize = 240;
const PRIMARY_OAM_SIZE: usize = 256;
const SECONDARY_OAM_SIZE: usize = 32;

// Generated from https://bisqwit.iki.fi/utils/nespalette.php
const NES_PALETTE: &[u8] = include_bytes!("../../palette/nespalette.pal");

#[derive(Debug)]
pub struct Ppu {
    pub(crate) bus: PpuBus,
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
    frame_buffer: Vec<u8>,
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
    sprite_zero_eval: bool,
    sprite_zero_pixel: bool,
    palette: Vec<u8>,
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
            frame_buffer: vec![0; SCREEN_WIDTH * SCREEN_HEIGHT * 4],
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
            sprite_zero_eval: false,
            sprite_zero_pixel: false,
            palette: NES_PALETTE.to_vec(),
        }
    }
}

impl Ppu {
    pub fn read_buffer(&self) -> u8 {
        self.vram_buffer
    }

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
        &self.frame_buffer
    }

    pub fn set_palette(&mut self, palette: &[u8]) {
        self.palette = palette.to_vec();
    }

    fn increment_vram_address(&mut self) {
        let offset = self.ctrl.get_vram_increment_value();
        self.v_addr.increment(offset);
    }

    fn load_background_shifters(&mut self) {
        self.bg_pattern_shift.low |= self.bg_pattern.low as u16;
        self.bg_pattern_shift.high |= self.bg_pattern.high as u16;
        self.bg_palette_shift.low |= (self.bg_palette_id as u16 & 0b01) * 0xFF;
        self.bg_palette_shift.high |= (self.bg_palette_id as u16 & 0b10) * 0xFF;
    }

    fn update_background_shifters(&mut self) {
        self.bg_pattern_shift.low <<= 1;
        self.bg_pattern_shift.high <<= 1;
        self.bg_palette_shift.low <<= 1;
        self.bg_palette_shift.high <<= 1;
    }

    fn update_sprite_shifters(&mut self) {
        for i in 0..8 {
            if self.sp_offset_shift[i] == 0 {
                self.sp_pattern_shift[i].low <<= 1;
                self.sp_pattern_shift[i].high <<= 1;
            } else {
                self.sp_offset_shift[i] -= 1;
            }
        }
    }

    fn tick_background(&mut self) {
        match self.dot {
            1..=256 | 321..=336 => {
                self.update_background_shifters();
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
                self.load_background_shifters();
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
                self.bg_address = nametable + (self.bg_pattern_id as u16) * 16 + fine_y;
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
            self.oam_index_overflow = false;
        }

        if self.dot >= 1 && self.dot < 256 {
            self.update_sprite_shifters();
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
        if !self.oam_index_overflow && self.secondary_oam_index < 32 {
            let sprite_y = self.oam_buffer;
            let sprite_height = self.ctrl.get_sprite_height() as i16;
            let offset = self.scanline as i16 - sprite_y as i16;

            if offset >= 0 && offset < sprite_height {
                let i = self.secondary_oam_index as usize;
                let j = self.primary_oam_index as usize;

                self.secondary_oam[i..i + 4].copy_from_slice(&self.primary_oam[j..j + 4]);
                self.secondary_oam_index += 4;

                if self.primary_oam_index == 0 {
                    self.sprite_zero_eval = true;
                }
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
                let attribute = self.sp_buffer[2];
                let sprite_x = self.sp_buffer[3];
                self.sp_attribute_shift[index] = attribute;
                self.sp_offset_shift[index] = sprite_x;
                self.sp_pattern_shift[index].high = self.bus.read_u8(self.sp_address);

                if attribute.contains(6) {
                    self.reverse_sprite_pattern_bits(index); // reverse horizontally
                }
            }
            _ => {}
        }

        if cycle < 4 && self.secondary_oam_index < 31 {
            self.secondary_oam_index += 1;
        }
    }

    fn reverse_sprite_pattern_bits(&mut self, index: usize) {
        self.sp_pattern_shift[index].low = self.sp_pattern_shift[index].low.reverse_bits();
        self.sp_pattern_shift[index].high = self.sp_pattern_shift[index].high.reverse_bits();
    }

    // https://www.nesdev.org/wiki/PPU_OAM#Byte_1
    fn get_sprite_pattern_address(&self) -> u16 {
        let sprite_y = self.sp_buffer[0];
        let tile = self.sp_buffer[1];
        let attribute = self.sp_buffer[2];
        let height = self.ctrl.get_sprite_height();
        let pattern_table = self.ctrl.get_sprite_pattern_table_address();
        let flip_vertical = attribute.contains(7);
        let y = self.scanline - sprite_y as u16;
        let base_address = match height {
            8 => pattern_table + 16 * tile as u16,
            _ => 0x1000 * tile.get(0) as u16 + ((tile as u16 & 0b1111_1110) + (y / 8)) * 16,
        };

        if flip_vertical {
            base_address + 7 - (y % 8)
        } else {
            base_address + (y % 8)
        }
    }

    fn render_pixel(&mut self) {
        let x = self.dot.saturating_sub(1) as usize; // one dot offset
        let y = self.scanline as usize;

        if x < 256 && y < 240 {
            let (bg_pixel, bg_palette) = self
                .mask
                .show_background()
                .then(|| self.get_background_pixel())
                .unwrap_or_default();

            let (sp_pixel, sp_palette, sp_priority) = self
                .mask
                .show_sprites()
                .then(|| self.get_sprite_pixel())
                .unwrap_or_default();

            let (pixel, palette) = match (bg_pixel, sp_pixel) {
                (0, 0) => (0, 0),
                (0, _) => (sp_pixel, sp_palette),
                (_, 0) => (bg_pixel, bg_palette),
                _ => {
                    if self.sprite_zero_pixel
                        && (self.dot > 8
                            || !(self.mask.show_background_leftmost()
                                || self.mask.show_sprites_leftmost()))
                        && self.dot < 256
                    {
                        self.status.set_sprite_zero_hit();
                    }

                    if sp_priority {
                        (sp_pixel, sp_palette)
                    } else {
                        (bg_pixel, bg_palette)
                    }
                }
            };

            let color_address = 0x3F00 + (4 * palette as u16 + pixel as u16);
            let color = self.bus.read_u8(color_address);

            self.set_frame_pixel(x, y, color);
        }
    }

    fn get_background_pixel(&self) -> (u8, u8) {
        let offset = 15 - self.fine_x as u16;
        let low_pixel = self.bg_pattern_shift.low.get(offset);
        let high_pixel = self.bg_pattern_shift.high.get(offset);
        let pixel = (high_pixel << 1) + low_pixel;
        let pixel = if pixel & 3 > 0 { pixel } else { 0 };

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
                let palette = (attribute & 0b11) + 4; // sprite palette offset
                let pixel_low = self.sp_pattern_shift[i].low.get(7);
                let pixel_high = self.sp_pattern_shift[i].high.get(7);
                let pixel = (pixel_high << 1) | pixel_low;

                if pixel != 0 {
                    if self.sprite_zero_eval && i == 0 {
                        self.sprite_zero_pixel = true;
                    }

                    return (pixel, palette, sp_priority);
                }
            }
        }

        (0, 0, false)
    }

    fn set_frame_pixel(&mut self, x: usize, y: usize, color: u8) {
        let color_index = 3 * color as usize;
        let frame_index = (y * 256 + x) * 4;

        self.frame_buffer[frame_index] = self.palette[color_index];
        self.frame_buffer[frame_index + 1] = self.palette[color_index + 1];
        self.frame_buffer[frame_index + 2] = self.palette[color_index + 2];
        self.frame_buffer[frame_index + 3] = 255;
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

        if self.mask.is_rendering() {
            self.render_pixel();
        }

        self.dot += 1;

        if self.dot > 340 {
            self.dot %= 341;
            self.scanline += 1;
            self.sprite_zero_eval = false;
            self.sprite_zero_pixel = false;

            if self.scanline > 261 {
                self.scanline = 0;
                self.odd_frame = !self.odd_frame;
            }
        }
    }
}

impl Reset for Ppu {
    fn reset(&mut self) {
        self.bus.reset();
        self.primary_oam = [0; PRIMARY_OAM_SIZE];
        self.secondary_oam = [0; SECONDARY_OAM_SIZE];
        self.vram_buffer = 0;
        self.oam_addr = 0;
        self.ctrl = ControlRegister::default();
        self.mask = MaskRegister::default();
        self.status = StatusRegister::default();
        self.t_addr = AddressRegister::default();
        self.v_addr = AddressRegister::default();
        self.fine_x = 0;
        self.latch = false;
        self.nmi.take();
        self.cycle = 0;
        self.dot = 0;
        self.scanline = 0;
        self.frame_buffer = vec![0; SCREEN_WIDTH * SCREEN_HEIGHT * 4];
        self.odd_frame = false;
        self.bg_address = 0;
        self.bg_pattern_id = 0;
        self.bg_palette_id = 0;
        self.bg_pattern = BitPlane::default();
        self.bg_pattern_shift = BitPlane::default();
        self.bg_palette_shift = BitPlane::default();
        self.oam_buffer = 0;
        self.primary_oam_index = 0;
        self.secondary_oam_index = 0;
        self.oam_index_overflow = false;
        self.sp_buffer = [0; 4];
        self.sp_address = 0;
        self.sp_pattern_shift = [BitPlane::default(); 8];
        self.sp_attribute_shift = [0; 8];
        self.sp_offset_shift = [0; 8];
        self.sprite_zero_eval = false;
        self.sprite_zero_pixel = false;
    }
}

#[cfg(test)]
mod tests {
    use super::Ppu;
    use crate::mappers::create_mapper_mock;

    #[test]
    fn test_ppu_oam_read_write() {
        let mapper = create_mapper_mock();
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
        let mapper = create_mapper_mock();
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
