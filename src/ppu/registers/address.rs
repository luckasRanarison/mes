#[derive(Debug, Default, Clone, Copy)]
pub struct AddressRegister(u16);

impl AddressRegister {
    pub fn get(&self) -> u16 {
        self.0 & 0x3FFF
    }

    pub fn set_coarse_x(&mut self, value: u8) {
        self.0 &= !(0b11111);
        self.0 |= value as u16;
    }

    pub fn get_coarse_x(&self) -> u8 {
        (self.0 & 0b11111) as u8
    }

    pub fn set_coarse_y(&mut self, value: u8) {
        self.0 &= !(0b11111 << 5);
        self.0 |= (value as u16) << 5;
    }

    pub fn get_coarse_y(&self) -> u8 {
        ((self.0 >> 5) & 0b11111) as u8
    }

    pub fn set_nametable(&mut self, value: u8) {
        self.0 &= !(0b11 << 10);
        self.0 |= (value as u16) << 10;
    }

    pub fn get_nametable_address(&self) -> u16 {
        0x2000 | self.0 & 0xFFF
    }

    pub fn get_attribute_address(&self) -> u16 {
        let nametable = self.get_nametable() as u16;
        let coarse_y = self.get_coarse_y() as u16;
        let coarse_x = self.get_coarse_x() as u16;

        0x23C0 | (nametable << 10) | ((coarse_y >> 2) << 3) | (coarse_x >> 2)
    }

    pub fn set_fine_y(&mut self, value: u8) {
        self.0 &= !(0b111 << 12);
        self.0 |= (value as u16) << 12;
    }

    pub fn get_fine_y(&self) -> u8 {
        (self.0 >> 12 & 0b111) as u8
    }

    pub fn set_high_byte(&mut self, value: u8) {
        self.0 &= !(0b111111 << 8);
        self.0 |= (value as u16) << 8;
    }

    pub fn set_low_byte(&mut self, value: u8) {
        self.0 &= !0xFF;
        self.0 |= value as u16;
    }

    pub fn increment(&mut self, value: u8) {
        self.0 = self.0.wrapping_add(value as u16);
    }

    pub fn scroll_x(&mut self) {
        let coarse_x = self.get_coarse_x();

        if coarse_x == 31 {
            self.set_coarse_x(0);
            self.0 ^= 0x400;
        } else {
            self.set_coarse_x(coarse_x + 1);
        }
    }

    pub fn scroll_y(&mut self) {
        let fine_y = self.get_fine_y();

        if fine_y == 7 {
            self.set_fine_y(0);
            let coarse_y = self.get_coarse_y();
            match coarse_y {
                29 => {
                    self.set_coarse_y(0);
                    self.0 ^= 0x800;
                }
                31 => self.set_coarse_y(0),
                _ => self.set_coarse_y(coarse_y + 1),
            }
        } else {
            self.set_fine_y(fine_y + 1);
        }
    }

    pub fn set_x(&mut self, other: AddressRegister) {
        self.0 = (self.0 & !0x041F) | (other.0 & 0x041F);
    }

    pub fn set_y(&mut self, other: AddressRegister) {
        self.0 = (self.0 & !0x7BE0) | (other.0 & 0x7BE0);
    }

    fn get_nametable(&self) -> u8 {
        (self.0 >> 10 & 0b11) as u8
    }
}
