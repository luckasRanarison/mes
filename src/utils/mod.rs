#[derive(Default)]
pub struct Register {
    value: u8,
}

impl Register {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn get(&self, flag: u8) -> u8 {
        (self.value >> flag) & 1
    }

    pub fn contains(&self, flag: u8) -> bool {
        self.get(flag) == 1
    }

    pub fn assign(&mut self, value: u8) {
        self.value = value;
    }

    pub fn set(&mut self, flag: u8) {
        self.value |= 1 << flag;
    }

    pub fn clear(&mut self, flag: u8) {
        self.value &= !(1 << flag);
    }

    pub fn update(&mut self, flag: u8, cond: bool) {
        if cond {
            self.set(flag);
        } else {
            self.clear(flag);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Register;

    #[test]
    fn test_register() {
        let mut reg = Register::new(0b1001_0000);

        reg.set(5);
        reg.clear(4);

        assert_eq!(reg.value, 0b1010_0000);
    }
}
