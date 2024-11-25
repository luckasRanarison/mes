// https://www.nesdev.org/wiki/CPU_interrupts

#[cfg(feature = "json")]
use serde::Serialize;

pub const INTERRUPT_LATENCY: u8 = 7;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub enum Interrupt {
    Nmi,
    Reset,
    Irq,
}

impl Interrupt {
    pub fn vector(&self) -> u16 {
        match self {
            Interrupt::Nmi => 0xFFFA,
            Interrupt::Reset => 0xFFFC,
            Interrupt::Irq => 0xFFFE,
        }
    }
}
