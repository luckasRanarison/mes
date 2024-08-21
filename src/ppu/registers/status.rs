use crate::utils::BitFlag;

enum StatusFlag {
    // __,
    // __,
    // __,
    // __,
    // __,
    O = 5,
    S = 6,
    V = 7,
}

/// VSO. ....
/// |||| ||||
/// |||+-++++- PPU open bus. Returns stale PPU bus contents.
/// ||+------- Sprite overflow. The intent was for this flag to be set
/// ||         whenever more than eight sprites appear on a scanline, but a
/// ||         hardware bug causes the actual behavior to be more complicated
/// ||         and generate false positives as well as false negatives; see
/// ||         PPU sprite evaluation. This flag is set during sprite
/// ||         evaluation and cleared at dot 1 (the second dot) of the
/// ||         pre-render line.
/// |+-------- Sprite 0 Hit.  Set when a nonzero pixel of sprite 0 overlaps
/// |          a nonzero background pixel; cleared at dot 1 of the pre-render
/// |          line.  Used for raster timing.
/// +--------- Vertical blank has started (0: not in vblank; 1: in vblank).
///            Set at dot 1 of line 241 (the line *after* the post-render
///            line); cleared after reading $2002 and at dot 1 of the
///            pre-render line.
#[derive(Debug, Default)]
pub struct StatusRegister(u8);

impl StatusRegister {
    pub fn read(&self) -> u8 {
        self.0
    }

    pub fn set_vblank(&mut self) {
        self.update(StatusFlag::V, true);
    }

    pub fn set_sprite_overflow(&mut self) {
        self.update(StatusFlag::O, true);
    }

    pub fn set_sprite_zero_hit(&mut self) {
        self.update(StatusFlag::S, true);
    }

    pub fn clear_vblank(&mut self) {
        self.update(StatusFlag::V, false);
    }

    pub fn is_vblank(&self) -> bool {
        self.contains(StatusFlag::V)
    }

    fn update(&mut self, flag: StatusFlag, state: bool) {
        self.0.update(flag as u8, state);
    }

    pub fn clear(&mut self) {
        self.0 &= 0b0001_1111;
    }

    fn contains(&self, flag: StatusFlag) -> bool {
        self.0.contains(flag as u8)
    }
}
