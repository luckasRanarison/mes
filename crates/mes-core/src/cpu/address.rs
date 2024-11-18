use super::register::CpuRegister;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Address {
    Memory(u16),
    Register(CpuRegister),
}

impl Address {
    pub fn to_memory_unchecked(self) -> u16 {
        match self {
            Address::Memory(address) => address,
            Address::Register(_) => panic!("Not a memory address"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AddressMode {
    Implied(CpuRegister),
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
