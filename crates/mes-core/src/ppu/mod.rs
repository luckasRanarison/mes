// https://www.nesdev.org/wiki/PPU_programmer_reference

mod internals;
mod registers;

use crate::{
    bus::{Bus, PpuBus},
    mappers::MapperChip,
    ppu::{internals::*, registers::*},
    utils::{BitFlag, Clock, Reset},
};

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;
pub const PALETTE_SIZE: usize = 192;
pub const FRAME_BUFFER_SIZE: usize = SCREEN_WIDTH * SCREEN_HEIGHT;

/// Generated from https://bisqwit.iki.fi/utils/nespalette.php
pub const COLOR_PALETTE: &[u8; PALETTE_SIZE] = include_bytes!("../../../../palette/nespalette.pal");

#[derive(Debug)]
pub struct Ppu {
    vram_buffer: u8,
    ctrl: ControlRegister,
    mask: MaskRegister,
    status: StatusRegister,
    t_addr: AddressRegister,
    v_addr: AddressRegister,
    fine_x: u8,
    latch: bool,
    cycle: u64,
    dot: u16,
    scanline: u16,
    odd_frame: bool,
    nmi: Option<bool>,
    oam: OamData,
    bg: BackgroundData,
    sprite: SpriteData,
    frame_buffer: [u8; FRAME_BUFFER_SIZE],
    pub(crate) bus: PpuBus,
}

impl Ppu {
    pub fn new(mapper: MapperChip) -> Self {
        Self {
            bus: PpuBus::new(mapper),
            ctrl: ControlRegister::default(),
            mask: MaskRegister::default(),
            status: StatusRegister::default(),
            t_addr: AddressRegister::default(),
            v_addr: AddressRegister::default(),
            vram_buffer: 0,
            fine_x: 0,
            latch: false,
            cycle: 0,
            dot: 0,
            scanline: 0,
            odd_frame: false,
            nmi: None,
            oam: OamData::default(),
            bg: BackgroundData::default(),
            sprite: SpriteData::default(),
            frame_buffer: [0; FRAME_BUFFER_SIZE],
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
        self.oam.primary[self.oam.address as usize]
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
        self.oam.address = value;
    }

    pub fn write_oam_data(&mut self, value: u8) {
        self.write_oam(self.oam.address, value);
        self.oam.address = self.oam.address.wrapping_add(1);
    }

    pub fn write_oam(&mut self, address: u8, value: u8) {
        self.oam.primary[address as usize] = value;
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
        self.frame_buffer.as_slice()
    }

    fn increment_vram_address(&mut self) {
        let offset = self.ctrl.get_vram_increment_value();
        self.v_addr.increment(offset);
    }

    fn tick_background(&mut self) {
        match self.dot {
            1..=256 | 321..=336 => {
                self.bg.update_shifters();
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
            337 | 339 => self.bg.address = self.v_addr.get_nametable_address(),
            338 | 340 => {
                self.bg.pattern_id = self.bus.read_u8(self.bg.address);

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
                self.bg.load_shifters();
                self.bg.address = self.v_addr.get_nametable_address();
            }
            2 => self.bg.pattern_id = self.bus.read_u8(self.bg.address),
            3 => self.bg.address = self.v_addr.get_attribute_address(),
            4 => {
                self.bg.palette_id = self.bus.read_u8(self.bg.address);

                if self.v_addr.get_coarse_y() & 2 > 0 {
                    self.bg.palette_id >>= 4;
                }
                if self.v_addr.get_coarse_x() & 2 > 0 {
                    self.bg.palette_id >>= 2;
                }

                self.bg.palette_id &= 0b11;
            }
            5 => {
                let nametable = self.ctrl.get_background_pattern_table_address();
                let fine_y = self.v_addr.get_fine_y() as u16;
                self.bg.address = nametable + (self.bg.pattern_id as u16) * 16 + fine_y;
            }
            6 => self.bg.pattern.low = self.bus.read_u8(self.bg.address),
            7 => self.bg.address += 8,
            _ => {
                self.bg.pattern.high = self.bus.read_u8(self.bg.address);

                if self.mask.is_rendering() {
                    if self.dot == 256 {
                        self.v_addr.scroll_y();
                    }

                    self.v_addr.scroll_x();
                }
            }
        }
    }

    fn tick_sprite(&mut self) {
        if self.dot == 1 {
            self.oam.secondary_index = 0;
        }

        if self.dot == 65 {
            self.oam.primary_index = 0;
            self.oam.secondary_index = 0;
            self.oam.index_overflow = false;
        }

        if self.dot >= 1 && self.dot < 256 {
            self.sprite.update_shifters();
        }

        match self.dot {
            1..=64 if self.scanline != 261 => {
                if self.dot % 2 == 0 {
                    self.oam.secondary[self.oam.secondary_index as usize] = self.oam.buffer;
                    self.oam.secondary_index += 1;
                } else {
                    self.oam.buffer = 0xFF;
                }
            }
            65..=256 if self.scanline != 261 => {
                if self.dot % 2 == 1 {
                    self.oam.buffer = self.oam.primary[self.oam.primary_index as usize];
                } else {
                    self.evaluate_sprite();
                }
            }
            257..=320 => self.fetch_sprite(),
            _ => {}
        }
    }

    // https://www.nesdev.org/wiki/PPU_sprite_evaluation
    fn evaluate_sprite(&mut self) {
        if !self.oam.index_overflow && self.oam.secondary_index < 32 {
            let sprite_y = self.oam.buffer;
            let sprite_height = self.ctrl.get_sprite_height() as i16;
            let offset = self.scanline as i16 - sprite_y as i16;

            if offset >= 0 && offset < sprite_height {
                let i = self.oam.secondary_index as usize;
                let j = self.oam.primary_index as usize;

                self.oam.secondary[i..i + 4].copy_from_slice(&self.oam.primary[j..j + 4]);
                self.oam.secondary_index += 4;

                if self.oam.primary_index == 0 {
                    self.sprite.zero_eval = true;
                }
            }
        } else {
            self.status.set_sprite_overflow();
        }

        let (index, overflow) = self.oam.primary_index.overflowing_add(4);
        self.oam.primary_index = index;
        self.oam.index_overflow |= overflow; // TODO: sprite overflow bug
    }

    fn fetch_sprite(&mut self) {
        if self.dot == 257 {
            self.oam.secondary_index = 0;
        }

        let cycle = (self.dot - 257) % 8;
        let index = (self.dot as usize - 257) / 8;
        let oam_value = self.oam.secondary[self.oam.secondary_index as usize];
        let y = self.sprite.buffer[0];

        match cycle {
            0..=3 => self.sprite.buffer[cycle as usize] = oam_value,
            4 if y != 0xFF => self.sprite.address = self.get_sprite_pattern_address(),
            5 if y != 0xFF => {
                self.sprite.pattern_shift[index].low = self.bus.read_u8(self.sprite.address)
            }
            6 if y != 0xFF => self.sprite.address += 8,
            7 if y != 0xFF => {
                let attribute = self.sprite.buffer[2];
                let sprite_x = self.sprite.buffer[3];
                self.sprite.attribute_shift[index] = attribute;
                self.sprite.offset_shift[index] = sprite_x;
                self.sprite.pattern_shift[index].high = self.bus.read_u8(self.sprite.address);

                if attribute.contains(6) {
                    self.sprite.horizontal_reverse(index);
                }
            }
            _ => {}
        }

        if cycle < 4 && self.oam.secondary_index < 31 {
            self.oam.secondary_index += 1;
        }
    }

    // https://www.nesdev.org/wiki/PPU_OAM#Byte_1
    fn get_sprite_pattern_address(&self) -> u16 {
        let sprite_y = self.sprite.buffer[0];
        let tile = self.sprite.buffer[1];
        let attribute = self.sprite.buffer[2];
        let height = self.ctrl.get_sprite_height();
        let pattern_table = self.ctrl.get_sprite_pattern_table_address();
        let flip_vertical = attribute.contains(7);
        let y = self.scanline - sprite_y as u16;
        let offset = if flip_vertical { 7 - (y % 8) } else { y % 8 };

        if height == 8 {
            pattern_table + 16 * tile as u16 + offset
        } else {
            0x1000 * tile.get(0) as u16
                + (tile as u16 & 0b1111_1110) * 16
                + (flip_vertical as u16 ^ (y / 8)) * 16
                + offset
        }
    }

    fn render_pixel(&mut self) {
        let x = self.dot.saturating_sub(1) as usize; // one dot offset
        let y = self.scanline as usize;

        if x < 256 && y < 240 {
            let (bg_pixel, bg_palette) = if self.mask.show_background() {
                self.get_background_pixel()
            } else {
                Default::default()
            };

            let (sp_pixel, sp_palette, sp_priority) = if self.mask.show_sprites() {
                self.get_sprite_pixel()
            } else {
                Default::default()
            };

            let (pixel, palette) = match (bg_pixel, sp_pixel) {
                (0, 0) => (0, 0),
                (0, _) => (sp_pixel, sp_palette),
                (_, 0) => (bg_pixel, bg_palette),
                _ => {
                    if self.sprite.zero_pixel
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
        let low_pixel = self.bg.pattern_shift.low.get(offset);
        let high_pixel = self.bg.pattern_shift.high.get(offset);
        let pixel = (high_pixel << 1) + low_pixel;
        let pixel = if pixel & 3 > 0 { pixel } else { 0 };

        let low_palette = self.bg.palette_shift.low.get(offset);
        let high_palette = self.bg.palette_shift.high.get(offset);
        let palette = (high_palette << 1) + low_palette;

        (pixel as u8, palette as u8)
    }

    fn get_sprite_pixel(&mut self) -> (u8, u8, bool) {
        for i in 0..8 {
            if self.sprite.offset_shift[i] == 0 {
                let attribute = self.sprite.attribute_shift[i];
                let sp_priority = !attribute.contains(5);
                let palette = (attribute & 0b11) + 4; // sprite palette offset
                let pixel_low = self.sprite.pattern_shift[i].low.get(7);
                let pixel_high = self.sprite.pattern_shift[i].high.get(7);
                let pixel = (pixel_high << 1) | pixel_low;

                if pixel != 0 {
                    if self.sprite.zero_eval && i == 0 {
                        self.sprite.zero_pixel = true;
                    }

                    return (pixel, palette, sp_priority);
                }
            }
        }

        (0, 0, false)
    }

    fn set_frame_pixel(&mut self, x: usize, y: usize, color: u8) {
        let color_index = 3 * color;
        let frame_index = y * 256 + x;

        self.frame_buffer[frame_index] = color_index;
    }
}

impl Clock for Ppu {
    // https://www.nesdev.org/wiki/PPU_rendering
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

            self.sprite.zero_eval = false;
            self.sprite.zero_pixel = false;

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
        self.vram_buffer = 0;
        self.ctrl.reset();
        self.mask.reset();
        self.status.reset();
        self.t_addr.reset();
        self.v_addr.reset();
        self.fine_x = 0;
        self.latch = false;
        self.cycle = 0;
        self.dot = 0;
        self.scanline = 0;
        self.odd_frame = false;
        self.nmi.take();
        self.oam.reset();
        self.sprite.reset();
        self.bg.reset();
        self.frame_buffer.fill(0);
    }
}

#[cfg(test)]
mod tests {
    use super::Ppu;
    use crate::mappers::MapperChip;

    #[test]
    fn test_ppu_oam_read_write() {
        let mapper = MapperChip::mock();
        let mut ppu = Ppu::new(mapper);

        ppu.write_oam_address(0x0F);
        ppu.write_oam_data(0x20);
        ppu.write_oam_data(0x21);

        assert_eq!(ppu.oam.address, 0x11);
        assert_eq!(ppu.oam.primary[0x0F], 0x20);
        assert_eq!(ppu.oam.primary[0x10], 0x21);
    }

    #[test]
    fn test_ppu_data_read_write() {
        let mapper = MapperChip::mock();
        let mut ppu = Ppu::new(mapper);

        ppu.write_addr(0x20);
        ppu.write_addr(0x50);

        assert_eq!(ppu.v_addr.get(), 0x2050);
        assert!(!ppu.latch);

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
