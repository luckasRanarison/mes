use crate::utils::BitFlag;

pub enum StatusFlag {
    // _,
    // _,
    // _,
    // _,
    // _,
    O = 5,
    S = 6,
    V = 7,
}

#[derive(Debug, Default)]
pub struct StatusRegister(u8);

impl StatusRegister {
    pub fn get_sprite_overflow(&self) -> bool {
        self.contains(StatusFlag::O)
    }

    pub fn get_sprite_zero_hit(&self) -> bool {
        self.contains(StatusFlag::S)
    }

    pub fn has_vblank_started(&self) -> bool {
        self.contains(StatusFlag::V)
    }

    pub fn update(&mut self, flag: StatusFlag, state: bool) {
        self.0.update(flag as u8, state);
    }

    fn contains(&self, flag: StatusFlag) -> bool {
        self.0.contains(flag as u8)
    }
}
