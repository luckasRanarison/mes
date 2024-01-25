use crate::utils::BitFlag;

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

impl MaskRegister {
    pub fn write(&mut self, value: u8) {
        self.0 = value;
    }

    pub fn is_grayscale(&self) -> bool {
        self.contains(MaskFlag::G0)
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
