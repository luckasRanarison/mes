use crate::utils::BitPlane;

#[derive(Debug, Default)]
pub struct SpriteData {
    pub buffer: [u8; 4],
    pub address: u16,
    pub pattern_shift: [BitPlane<u8>; 8],
    pub attribute_shift: [u8; 8],
    pub offset_shift: [u8; 8],
    pub zero_eval: bool,
    pub zero_pixel: bool,
}

impl SpriteData {
    pub fn update_shifters(&mut self) {
        for i in 0..8 {
            if self.offset_shift[i] == 0 {
                self.pattern_shift[i].low <<= 1;
                self.pattern_shift[i].high <<= 1;
            } else {
                self.offset_shift[i] -= 1;
            }
        }
    }

    pub fn horizontal_reverse(&mut self, index: usize) {
        self.pattern_shift[index].low = self.pattern_shift[index].low.reverse_bits();
        self.pattern_shift[index].high = self.pattern_shift[index].high.reverse_bits();
    }
}
