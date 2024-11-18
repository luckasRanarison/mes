// https://www.nesdev.org/wiki/CPU_interrupts

pub const INTERRUPT_LATENCY: u8 = 7;

#[derive(Debug, Clone, Copy, PartialEq)]
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
