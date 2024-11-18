use crate::utils::BitPlane;

#[derive(Debug, Default)]
pub struct BackgroundData {
    pub address: u16,
    pub pattern_id: u8,
    pub palette_id: u8,
    pub pattern: BitPlane<u8>,
    pub pattern_shift: BitPlane<u16>,
    pub palette_shift: BitPlane<u16>,
}

impl BackgroundData {
    pub fn load_shifters(&mut self) {
        self.pattern_shift.low |= self.pattern.low as u16;
        self.pattern_shift.high |= self.pattern.high as u16;
        self.palette_shift.low |= (self.palette_id as u16 & 0b01) * 0xFF;
        self.palette_shift.high |= (self.palette_id as u16 & 0b10) * 0xFF;
    }

    pub fn update_shifters(&mut self) {
        self.pattern_shift.low <<= 1;
        self.pattern_shift.high <<= 1;
        self.palette_shift.low <<= 1;
        self.palette_shift.high <<= 1;
    }
}
