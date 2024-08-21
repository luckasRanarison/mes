// https://www.nesdev.org/wiki/PPU_scrolling

/// yyy NN YYYYY XXXXX
/// ||| || ||||| +++++-- coarse X scroll
/// ||| || +++++-------- coarse Y scroll
/// ||| ++-------------- nametable select
/// +++----------------- fine Y scroll
#[derive(Debug, Default, Clone, Copy)]
pub struct AddressRegister(u16);

impl AddressRegister {
    pub fn get(&self) -> u16 {
        self.0 & 0x7FFF
    }

    pub fn set_coarse_x(&mut self, value: u8) {
        self.0 = (self.0 & !0x1F) | value as u16;
    }

    pub fn get_coarse_x(&self) -> u8 {
        (self.0 & 0x1F) as u8
    }

    pub fn set_coarse_y(&mut self, value: u8) {
        self.0 = (self.0 & !0x3E0) | ((value as u16) << 5);
    }

    pub fn get_coarse_y(&self) -> u8 {
        ((self.0 >> 5) & 0x1F) as u8
    }

    pub fn set_nametable(&mut self, value: u8) {
        self.0 = (self.0 & !0xC00) | ((value as u16) << 10);
    }

    pub fn get_nametable_address(&self) -> u16 {
        0x2000 | self.0 & 0xFFF
    }

    pub fn get_attribute_address(&self) -> u16 {
        0x23C0 | (self.0 & 0xC00) | ((self.0 >> 4) & 0x38) | ((self.0 >> 2) & 0x07)
    }

    pub fn set_fine_y(&mut self, value: u8) {
        self.0 = (self.0 & !0x7000) | ((value as u16) << 12);
    }

    pub fn get_fine_y(&self) -> u8 {
        ((self.0 >> 12) & 0b111) as u8
    }

    pub fn set_high_byte(&mut self, value: u8) {
        self.0 = (self.0 & 0x80FF) | ((value as u16) << 8);
    }

    pub fn set_low_byte(&mut self, value: u8) {
        self.0 = (self.0 & 0xFF00) | (value as u16);
    }

    pub fn increment(&mut self, value: u8) {
        self.0 = self.0.wrapping_add(value as u16);
    }

    pub fn scroll_x(&mut self) {
        let coarse_x = self.get_coarse_x();

        if coarse_x == 31 {
            self.set_coarse_x(0);
            self.0 ^= 0x400; // switch horizobtal nametable
        } else {
            self.set_coarse_x(coarse_x + 1);
        }
    }

    pub fn scroll_y(&mut self) {
        let fine_y = self.get_fine_y();

        if fine_y < 7 {
            self.set_fine_y(fine_y + 1);
        } else {
            self.set_fine_y(0);

            let coarse_y = self.get_coarse_y();

            if coarse_y == 29 {
                self.set_coarse_y(0);
                self.0 ^= 0x800; // switch vertical nametable
            } else if coarse_y == 31 {
                self.set_coarse_y(0);
            } else {
                self.set_coarse_y(coarse_y + 1);
            }
        }
    }

    pub fn set_x(&mut self, other: AddressRegister) {
        self.0 = (self.0 & !0x041F) | (other.0 & 0x041F);
    }

    pub fn set_y(&mut self, other: AddressRegister) {
        self.0 = (self.0 & !0x7BE0) | (other.0 & 0x7BE0);
    }
}

#[cfg(test)]
mod tests {
    use super::AddressRegister;

    #[test]
    fn test_loopy_register() {
        let mut t = AddressRegister::default();

        t.set_nametable(0b01);
        assert_eq!((t.0 >> 10) & 0b11, 0b01);

        t.set_coarse_x(0b0111_1101 >> 3);
        assert_eq!(t.get_coarse_x(), 0b01111);

        t.set_coarse_y(0b01011110 >> 3);
        t.set_fine_y(0b110);
        assert_eq!(t.get_coarse_y(), 0b01011);
        assert_eq!(t.get_fine_y(), 0b110);

        t.set_high_byte(0b111101);
        t.set_low_byte(0b11110000);
        assert_eq!(t.get(), 0b011110111110000);
    }

    #[test]
    fn test_register_scrolling() {
        let mut t = AddressRegister::default();

        t.set_coarse_x(30);
        t.scroll_x();
        assert_eq!(t.get_coarse_x(), 31);
        assert_eq!((t.0 >> 10) & 0b11, 0b00);

        t.scroll_x();
        assert_eq!(t.get_coarse_x(), 0);
        assert_eq!((t.0 >> 10) & 0b11, 0b01);

        t.set_fine_y(6);
        t.set_coarse_y(29);
        t.scroll_y();
        assert_eq!(t.get_fine_y(), 7);
        assert_eq!(t.get_coarse_y(), 29);
        assert_eq!((t.0 >> 10) & 0b11, 0b01);

        t.scroll_y();
        assert_eq!(t.get_fine_y(), 0);
        assert_eq!(t.get_coarse_y(), 0);
        assert_eq!((t.0 >> 10) & 0b11, 0b11);
    }
}
