// https://www.masswerk.at/6502/6502_instruction_set.html#registers

use crate::utils::BitFlag;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CpuRegister {
    PC,
    AC,
    X,
    Y,
    SR,
    SP,
}

#[rustfmt::skip]
pub mod status_flag {
    pub const C:  u8 = 0;
    pub const Z:  u8 = 1;
    pub const I:  u8 = 2;
    pub const D:  u8 = 3;
    pub const B:  u8 = 4;
    pub const __: u8 = 5;
    pub const V:  u8 = 6;
    pub const N:  u8 = 7;
}

pub struct StatusRegister(u8);

impl Default for StatusRegister {
    fn default() -> Self {
        Self(0b0010_0000)
    }
}

impl StatusRegister {
    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn get(&self, flag: u8) -> u8 {
        self.0.get(flag)
    }

    pub fn contains(&self, flag: u8) -> bool {
        self.0.contains(flag)
    }

    pub fn assign(&mut self, value: u8) {
        self.0 = value
    }

    pub fn update(&mut self, flag: u8, cond: bool) {
        self.0.update(flag, cond)
    }

    pub fn set(&mut self, flag: u8) {
        self.0.update(flag, true)
    }

    pub fn clear(&mut self, flag: u8) {
        self.0.update(flag, false)
    }

    pub fn update_zero(&mut self, value: u8) {
        self.0.update(status_flag::Z, value == 0);
    }

    pub fn update_negative(&mut self, value: u8) {
        self.0.update(status_flag::N, value >> 7 == 1);
    }
}
