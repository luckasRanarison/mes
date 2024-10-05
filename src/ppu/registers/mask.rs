use crate::utils::BitFlag;

#[allow(unused)]
#[rustfmt::skip]
mod mask_flag {
    pub const G0: u8 = 0;
    pub const M0: u8 = 1;
    pub const M1: u8 = 2;
    pub const B0: u8 = 3;
    pub const S:  u8 = 4;
    pub const R:  u8 = 5;
    pub const G1: u8 = 6;
    pub const B1: u8 = 7;
}

/// BGRs bMmG
/// |||| ||||
/// |||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
/// |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
/// |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
/// |||| +---- 1: Show background
/// |||+------ 1: Show sprites
/// ||+------- Emphasize red (green on PAL/Dendy)
/// |+-------- Emphasize green (red on PAL/Dendy)
/// +--------- Emphasize blue
#[derive(Debug, Default)]
pub struct MaskRegister(u8);

impl MaskRegister {
    pub fn write(&mut self, value: u8) {
        self.0 = value;
    }

    pub fn is_rendering(&self) -> bool {
        self.show_background() || self.show_sprites()
    }

    pub fn show_background_leftmost(&self) -> bool {
        self.0.contains(mask_flag::M0)
    }

    pub fn show_sprites_leftmost(&self) -> bool {
        self.0.contains(mask_flag::M1)
    }

    pub fn show_background(&self) -> bool {
        self.0.contains(mask_flag::B0)
    }

    pub fn show_sprites(&self) -> bool {
        self.0.contains(mask_flag::S)
    }
}
