use super::register::Register;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Address {
    Memory(u16),
    Register(Register),
}

impl Address {
    pub fn memory_unchecked(&self) -> u16 {
        match self {
            Address::Memory(address) => *address,
            Address::Register(_) => panic!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AddressMode {
    Implied(Register),
    Immediate,
    Absolute,
    ZeroPage,
    AbsoluteX,
    AbsoluteY,
    ZeroPageX,
    ZeroPageY,
    Indirect,
    IndirectX,
    IndirectY,
    Relative,
}
