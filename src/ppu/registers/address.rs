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
        } else {
            self.high = value;
        }

        self.set(self.get() & 0x3FFF);
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
