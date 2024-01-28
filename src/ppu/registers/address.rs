#[derive(Debug, Default, Clone, Copy)]
pub struct AddressRegiser {
    value: u16,
}

impl AddressRegiser {
    pub fn get(&self) -> u16 {
        self.value
    }

    pub fn set_coarse_x(&mut self, value: u8) {
        self.value |= value as u16;
    }

    pub fn get_coarse_x(&self) -> u8 {
        (self.value & 0b11111) as u8
    }

    pub fn set_coarse_y(&mut self, value: u8) {
        self.value |= (value as u16) << 5;
    }

    pub fn get_coarse_y(&self) -> u8 {
        ((self.value >> 5) & 0b11111) as u8
    }

    pub fn set_nametable(&mut self, value: u8) {
        self.value |= (value as u16) << 10;
    }

    pub fn get_nametable(&self) -> u8 {
        (self.value >> 10 & 0b11) as u8
    }

    pub fn set_fine_y(&mut self, value: u8) {
        self.value |= (value as u16) << 12;
    }

    pub fn get_fine_y(&self) -> u8 {
        (self.value >> 12 & 0b111) as u8
    }

    pub fn set_high_byte(&mut self, value: u8) {
        self.value |= (value as u16) << 8;
        self.value &= !(1 << 15);
    }

    pub fn set_low_byte(&mut self, value: u8) {
        self.value &= 0b1111_1111_0000_0000;
        self.value |= value as u16;
    }

    pub fn increment(&mut self, value: u8) {
        self.value = self.value.wrapping_add(value as u16);
    }
}

#[cfg(test)]
mod tests {
    use super::AddressRegiser;

    #[test]
    fn test_address_register() {
        let mut t = AddressRegiser::default();

        t.set_nametable(0b10);
        assert_eq!(0b10, t.get_nametable());
        t.set_coarse_x(0b01111);
        assert_eq!(0b01111, t.get_coarse_x());
        t.set_coarse_y(0b01011);
        assert_eq!(0b01011, t.get_coarse_y());
        t.set_high_byte(0b101011);
        assert_eq!(0b0010_1011_0110_1111, t.get());
        t.set_low_byte(0b0000_0011);
        assert_eq!(0b0010_1011_0000_0011, t.get());
    }
}
