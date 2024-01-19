use std::{
    fmt,
    ops::{Deref, DerefMut},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Register {
    PC,
    AC,
    X,
    Y,
    SR,
    SP,
}

#[derive(Debug, PartialEq)]
enum StatusFlag {
    C,
    Z,
    I,
    D,
    B,
    __,
    V,
    N,
}

pub struct StatusRegister(u8);

impl Default for StatusRegister {
    fn default() -> Self {
        Self(0b0010_0000)
    }
}

impl Deref for StatusRegister {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for StatusRegister {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl StatusRegister {
    pub fn get_carry(&self) -> u8 {
        self.get_flag(StatusFlag::C)
    }

    pub fn get_zero(&self) -> u8 {
        self.get_flag(StatusFlag::Z)
    }

    pub fn get_interrupt(&self) -> u8 {
        self.get_flag(StatusFlag::I)
    }

    pub fn get_deciaml(&self) -> u8 {
        self.get_flag(StatusFlag::D)
    }

    pub fn get_break(&self) -> u8 {
        self.get_flag(StatusFlag::B)
    }

    pub fn get_overflow(&self) -> u8 {
        self.get_flag(StatusFlag::V)
    }

    pub fn get_negative(&self) -> u8 {
        self.get_flag(StatusFlag::N)
    }

    pub fn update_carry(&mut self, cond: bool) -> &mut Self {
        self.update_flag(StatusFlag::C, cond)
    }

    pub fn update_zero(&mut self, value: u8) -> &mut Self {
        self.update_flag(StatusFlag::Z, value == 0)
    }

    pub fn update_negative(&mut self, value: u8) -> &mut Self {
        self.update_flag(StatusFlag::N, (value >> 7 & 1) == 1)
    }

    pub fn update_overflow(&mut self, cond: bool) -> &mut Self {
        self.update_flag(StatusFlag::V, cond)
    }

    pub fn update_break(&mut self, cond: bool) -> &mut Self {
        self.update_flag(StatusFlag::B, cond)
    }

    pub fn update_interrupt(&mut self, cond: bool) -> &mut Self {
        self.update_flag(StatusFlag::I, cond)
    }

    pub fn update_decimal(&mut self, cond: bool) -> &mut Self {
        self.update_flag(StatusFlag::D, cond)
    }

    fn get_flag(&self, flag: StatusFlag) -> u8 {
        (self.0 >> flag as u8) & 1
    }

    fn update_flag(&mut self, flag: StatusFlag, cond: bool) -> &mut Self {
        self.0 = match cond {
            true => self.0 | (1 << flag as u8),
            false => self.0 & !(1 << flag as u8),
        };
        self
    }
}

impl fmt::Debug for StatusRegister {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{{ N: {}, V: {}, _: 1, B: {}, D: {}, I: {}, Z: {}, C: {} }}",
            self.get_negative(),
            self.get_overflow(),
            self.get_break(),
            self.get_deciaml(),
            self.get_interrupt(),
            self.get_zero(),
            self.get_carry(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::StatusRegister;

    #[test]
    fn test_status_register() {
        let mut sr = StatusRegister::default();

        sr.update_zero(0);
        assert_eq!(sr.get_zero(), 1);

        let value = 0b1001_0000;
        sr.update_negative(value).update_zero(value);

        assert_eq!(sr.get_negative(), 1);
        assert_eq!(sr.get_zero(), 0);

        sr.update_carry(true);

        assert_eq!(sr.get_carry(), 1);
        assert_eq!(sr.0, 0b1010_0001);
    }
}
