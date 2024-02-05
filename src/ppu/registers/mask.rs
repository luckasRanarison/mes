use crate::utils::BitFlag;

#[allow(unused)]
enum MaskFlag {
    G0,
    M0,
    M1,
    B0,
    S,
    R,
    G1,
    B1,
}

#[derive(Debug, Default)]
pub struct MaskRegister(u8);
// BGRs bMmG
// |||| ||||
// |||| |||+- Greyscale (0: normal color, 1: produce a greyscale display)
// |||| ||+-- 1: Show background in leftmost 8 pixels of screen, 0: Hide
// |||| |+--- 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
// |||| +---- 1: Show background
// |||+------ 1: Show sprites
// ||+------- Emphasize red (green on PAL/Dendy)
// |+-------- Emphasize green (red on PAL/Dendy)
// +--------- Emphasize blue

impl MaskRegister {
    pub fn write(&mut self, value: u8) {
        self.0 = value;
    }

    pub fn is_rendering(&self) -> bool {
        self.show_background() || self.show_sprites()
    }

    pub fn show_background_leftmost(&self) -> bool {
        self.contains(MaskFlag::M0)
    }

    pub fn show_sprites_leftmost(&self) -> bool {
        self.contains(MaskFlag::M1)
    }

    pub fn show_background(&self) -> bool {
        self.contains(MaskFlag::B0)
    }

    pub fn show_sprites(&self) -> bool {
        self.contains(MaskFlag::S)
    }

    fn contains(&self, flag: MaskFlag) -> bool {
        self.0.contains(flag as u8)
    }
}
