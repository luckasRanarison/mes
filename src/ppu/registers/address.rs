#[derive(Debug, Default)]
pub struct AddressRegiser {
    high: u8,
    low: u8,
}

impl AddressRegiser {
    pub fn get(&self) -> u16 {
        u16::from_be_bytes([self.high, self.low])
    }

    pub fn write(&mut self, value: u8, latch: &mut bool) {
        if *latch {
            self.low = value;
            self.set(self.get() & 0x3FFF);
        } else {
            self.high = value;
        }

        *latch = !*latch;
    }

    pub fn increment(&mut self, value: u8) {
        let result = self.get().wrapping_add(value as u16);
        self.set(result & 0x3FFF);
    }

    fn set(&mut self, value: u16) {
        [self.high, self.low] = value.to_be_bytes();
    }
}

#[cfg(test)]
mod tests {
    use super::AddressRegiser;

    #[test]
    fn test_address_register() {
        let mut register = AddressRegiser::default();
        let mut latch = false;

        register.write(0x20, &mut latch);
        assert_eq!(register.high, 0x20);

        register.write(0xC0, &mut latch);
        assert_eq!(register.low, 0xC0);
        assert_eq!(register.get(), 0x20C0);

        register.write(0x40, &mut latch);
        assert_eq!(register.high, 0x40);

        register.write(0x10, &mut latch);
        register.increment(5);
        assert_eq!(register.get(), 0x0015);
        assert_eq!(latch, false);
    }
}
