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

#[derive(Debug)]
pub enum StatusFlag {
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

impl StatusRegister {
    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn get(&self, flag: StatusFlag) -> u8 {
        self.0.get(flag as u8)
    }

    pub fn contains(&self, flag: StatusFlag) -> bool {
        self.0.contains(flag as u8)
    }

    pub fn assign(&mut self, value: u8) {
        self.0 = value
    }

    pub fn update(&mut self, flag: StatusFlag, cond: bool) {
        self.0.update(flag as u8, cond)
    }

    pub fn set(&mut self, flag: StatusFlag) {
        self.update(flag, true)
    }

    pub fn clear(&mut self, flag: StatusFlag) {
        self.update(flag, false)
    }

    pub fn update_zero(&mut self, value: u8) {
        self.update(StatusFlag::Z, value == 0);
    }

    pub fn update_negative(&mut self, value: u8) {
        self.update(StatusFlag::N, value >> 7 == 1);
    }
}
