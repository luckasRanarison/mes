// https://www.masswerk.at/6502/6502_instruction_set.html#layout

use super::{address::AddressMode, register::CpuRegister as Reg};

#[derive(Debug, Clone, Copy)]
pub struct Opcode {
    pub asm: Asm,
    pub cycle: u8,
    pub adr_mode: AddressMode,
}

impl Opcode {
    const fn new(asm: Asm, cycle: u8, adr_mode: AddressMode) -> Self {
        Self {
            asm,
            cycle,
            adr_mode,
        }
    }

    pub fn len(&self) -> u8 {
        match self.adr_mode {
            AddressMode::Implied(_) => 1,
            AddressMode::Absolute
            | AddressMode::AbsoluteX
            | AddressMode::AbsoluteY
            | AddressMode::Indirect => 3,
            _ => 2,
        }
    }

    pub fn total_cycles(&self, crossed_boundary: bool, jumped: bool) -> u8 {
        self.cycle
            + self.boundary_cross_cycle(crossed_boundary)
            + self.jump_cycles(jumped, crossed_boundary)
    }

    fn jump_cycles(&self, jumped: bool, crossed_boundary: bool) -> u8 {
        match jumped {
            true if self.is_branching() => 1 + crossed_boundary as u8,
            _ => 0,
        }
    }

    fn boundary_cross_cycle(&self, crossed_boundary: bool) -> u8 {
        match self.adr_mode {
            AddressMode::AbsoluteX | AddressMode::AbsoluteY | AddressMode::IndirectY
                if self.asm != Asm::STA && crossed_boundary && self.cycle < 6 =>
            {
                1
            }
            _ => 0,
        }
    }

    pub fn is_branching(&self) -> bool {
        matches!(
            self.asm,
            Asm::BCC | Asm::BCS | Asm::BVC | Asm::BVS | Asm::BEQ | Asm::BMI | Asm::BNE | Asm::BPL
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(clippy::upper_case_acronyms)]
#[rustfmt::skip]
pub enum Asm {
    LDA, LDX, LDY, // load
    STA, STX, STY, // store
    TAX, TAY, TSX, TXA, TXS, TYA, // transfert
    PHA, PHP, PLA, PLP, // push/pull
    DEC, DEX, DEY, // decrement
    INC, INX, INY, // increment
    ADC, SBC, // arithmetic
    AND, EOR, ORA, SAX, // logic
    ASL, LSR, ROL, ROR, // shift
    CLC, CLD, CLI, CLV, // clear
    SEC, SED, SEI, // set
    CMP, CPX, CPY, // compare
    BCC, BCS, BVC, BVS, BEQ, BMI, BNE, BPL, // branch
    JMP, JSR, RTS, // subroutines
    BRK, RTI, // interrupt
    ALR, ANC, ARR, // and + op
    LAS, LAX, // lda + op
    SBX, DCP, ISB, // increment/decrement + op
    SLO, RLA, SRE, RRA, // shift + op
    BIT, NOP, JAM, // misc
    TAS, SHA, SHX, SHY, // unstable!
    ANE, LXA, // highly unstable!!
}

use AddressMode as Adr;

pub const OPCODES: [Opcode; 256] = [
    Opcode::new(Asm::BRK, 7, Adr::Implied(Reg::PC)),
    Opcode::new(Asm::ORA, 6, Adr::IndirectX),
    Opcode::new(Asm::JAM, 0, Adr::Immediate),
    Opcode::new(Asm::SLO, 8, Adr::IndirectX),
    Opcode::new(Asm::NOP, 3, Adr::ZeroPage),
    Opcode::new(Asm::ORA, 3, Adr::ZeroPage),
    Opcode::new(Asm::ASL, 5, Adr::ZeroPage),
    Opcode::new(Asm::SLO, 5, Adr::ZeroPage),
    Opcode::new(Asm::PHP, 3, Adr::Implied(Reg::SR)),
    Opcode::new(Asm::ORA, 2, Adr::Immediate),
    Opcode::new(Asm::ASL, 2, Adr::Implied(Reg::AC)),
    Opcode::new(Asm::ANC, 2, Adr::Immediate),
    Opcode::new(Asm::NOP, 4, Adr::Absolute),
    Opcode::new(Asm::ORA, 4, Adr::Absolute),
    Opcode::new(Asm::ASL, 6, Adr::Absolute),
    Opcode::new(Asm::SLO, 6, Adr::Absolute),
    Opcode::new(Asm::BPL, 2, Adr::Relative),
    Opcode::new(Asm::ORA, 5, Adr::IndirectY),
    Opcode::new(Asm::JAM, 0, Adr::Immediate),
    Opcode::new(Asm::SLO, 8, Adr::IndirectY),
    Opcode::new(Asm::NOP, 4, Adr::ZeroPageX),
    Opcode::new(Asm::ORA, 4, Adr::ZeroPageX),
    Opcode::new(Asm::ASL, 6, Adr::ZeroPageX),
    Opcode::new(Asm::SLO, 6, Adr::ZeroPageX),
    Opcode::new(Asm::CLC, 2, Adr::Implied(Reg::SR)),
    Opcode::new(Asm::ORA, 4, Adr::AbsoluteY),
    Opcode::new(Asm::NOP, 2, Adr::Implied(Reg::PC)),
    Opcode::new(Asm::SLO, 7, Adr::AbsoluteY),
    Opcode::new(Asm::NOP, 4, Adr::AbsoluteX),
    Opcode::new(Asm::ORA, 4, Adr::AbsoluteX),
    Opcode::new(Asm::ASL, 7, Adr::AbsoluteX),
    Opcode::new(Asm::SLO, 7, Adr::AbsoluteX),
    Opcode::new(Asm::JSR, 6, Adr::Absolute),
    Opcode::new(Asm::AND, 6, Adr::IndirectX),
    Opcode::new(Asm::JAM, 0, Adr::Immediate),
    Opcode::new(Asm::RLA, 8, Adr::IndirectX),
    Opcode::new(Asm::BIT, 3, Adr::ZeroPage),
    Opcode::new(Asm::AND, 3, Adr::ZeroPage),
    Opcode::new(Asm::ROL, 5, Adr::ZeroPage),
    Opcode::new(Asm::RLA, 5, Adr::ZeroPage),
    Opcode::new(Asm::PLP, 4, Adr::Implied(Reg::SR)),
    Opcode::new(Asm::AND, 2, Adr::Immediate),
    Opcode::new(Asm::ROL, 2, Adr::Implied(Reg::AC)),
    Opcode::new(Asm::ANC, 2, Adr::Immediate),
    Opcode::new(Asm::BIT, 4, Adr::Absolute),
    Opcode::new(Asm::AND, 4, Adr::Absolute),
    Opcode::new(Asm::ROL, 6, Adr::Absolute),
    Opcode::new(Asm::RLA, 6, Adr::Absolute),
    Opcode::new(Asm::BMI, 2, Adr::Relative),
    Opcode::new(Asm::AND, 5, Adr::IndirectY),
    Opcode::new(Asm::JAM, 0, Adr::Immediate),
    Opcode::new(Asm::RLA, 8, Adr::IndirectY),
    Opcode::new(Asm::NOP, 4, Adr::ZeroPageX),
    Opcode::new(Asm::AND, 4, Adr::ZeroPageX),
    Opcode::new(Asm::ROL, 6, Adr::ZeroPageX),
    Opcode::new(Asm::RLA, 6, Adr::ZeroPageX),
    Opcode::new(Asm::SEC, 2, Adr::Implied(Reg::SR)),
    Opcode::new(Asm::AND, 4, Adr::AbsoluteY),
    Opcode::new(Asm::NOP, 2, Adr::Implied(Reg::PC)),
    Opcode::new(Asm::RLA, 7, Adr::AbsoluteY),
    Opcode::new(Asm::NOP, 4, Adr::AbsoluteX),
    Opcode::new(Asm::AND, 4, Adr::AbsoluteX),
    Opcode::new(Asm::ROL, 7, Adr::AbsoluteX),
    Opcode::new(Asm::RLA, 7, Adr::AbsoluteX),
    Opcode::new(Asm::RTI, 6, Adr::Implied(Reg::PC)),
    Opcode::new(Asm::EOR, 6, Adr::IndirectX),
    Opcode::new(Asm::JAM, 0, Adr::Immediate),
    Opcode::new(Asm::SRE, 8, Adr::IndirectX),
    Opcode::new(Asm::NOP, 3, Adr::ZeroPage),
    Opcode::new(Asm::EOR, 3, Adr::ZeroPage),
    Opcode::new(Asm::LSR, 5, Adr::ZeroPage),
    Opcode::new(Asm::SRE, 5, Adr::ZeroPage),
    Opcode::new(Asm::PHA, 3, Adr::Implied(Reg::AC)),
    Opcode::new(Asm::EOR, 2, Adr::Immediate),
    Opcode::new(Asm::LSR, 2, Adr::Implied(Reg::AC)),
    Opcode::new(Asm::ALR, 2, Adr::Immediate),
    Opcode::new(Asm::JMP, 3, Adr::Absolute),
    Opcode::new(Asm::EOR, 4, Adr::Absolute),
    Opcode::new(Asm::LSR, 6, Adr::Absolute),
    Opcode::new(Asm::SRE, 6, Adr::Absolute),
    Opcode::new(Asm::BVC, 2, Adr::Relative),
    Opcode::new(Asm::EOR, 5, Adr::IndirectY),
    Opcode::new(Asm::JAM, 0, Adr::Immediate),
    Opcode::new(Asm::SRE, 8, Adr::IndirectY),
    Opcode::new(Asm::NOP, 4, Adr::ZeroPageX),
    Opcode::new(Asm::EOR, 4, Adr::ZeroPageX),
    Opcode::new(Asm::LSR, 6, Adr::ZeroPageX),
    Opcode::new(Asm::SRE, 6, Adr::ZeroPageX),
    Opcode::new(Asm::CLI, 2, Adr::Implied(Reg::SR)),
    Opcode::new(Asm::EOR, 4, Adr::AbsoluteY),
    Opcode::new(Asm::NOP, 2, Adr::Implied(Reg::PC)),
    Opcode::new(Asm::SRE, 7, Adr::AbsoluteY),
    Opcode::new(Asm::NOP, 4, Adr::AbsoluteX),
    Opcode::new(Asm::EOR, 4, Adr::AbsoluteX),
    Opcode::new(Asm::LSR, 7, Adr::AbsoluteX),
    Opcode::new(Asm::SRE, 7, Adr::AbsoluteX),
    Opcode::new(Asm::RTS, 6, Adr::Implied(Reg::PC)),
    Opcode::new(Asm::ADC, 6, Adr::IndirectX),
    Opcode::new(Asm::JAM, 0, Adr::Immediate),
    Opcode::new(Asm::RRA, 8, Adr::IndirectX),
    Opcode::new(Asm::NOP, 3, Adr::ZeroPage),
    Opcode::new(Asm::ADC, 3, Adr::ZeroPage),
    Opcode::new(Asm::ROR, 5, Adr::ZeroPage),
    Opcode::new(Asm::RRA, 5, Adr::ZeroPage),
    Opcode::new(Asm::PLA, 4, Adr::Implied(Reg::AC)),
    Opcode::new(Asm::ADC, 2, Adr::Immediate),
    Opcode::new(Asm::ROR, 2, Adr::Implied(Reg::AC)),
    Opcode::new(Asm::ARR, 2, Adr::Immediate),
    Opcode::new(Asm::JMP, 5, Adr::Indirect),
    Opcode::new(Asm::ADC, 4, Adr::Absolute),
    Opcode::new(Asm::ROR, 6, Adr::Absolute),
    Opcode::new(Asm::RRA, 6, Adr::Absolute),
    Opcode::new(Asm::BVS, 2, Adr::Relative),
    Opcode::new(Asm::ADC, 5, Adr::IndirectY),
    Opcode::new(Asm::JAM, 0, Adr::Immediate),
    Opcode::new(Asm::RRA, 8, Adr::IndirectY),
    Opcode::new(Asm::NOP, 4, Adr::ZeroPageX),
    Opcode::new(Asm::ADC, 4, Adr::ZeroPageX),
    Opcode::new(Asm::ROR, 6, Adr::ZeroPageX),
    Opcode::new(Asm::RRA, 6, Adr::ZeroPageX),
    Opcode::new(Asm::SEI, 2, Adr::Implied(Reg::SR)),
    Opcode::new(Asm::ADC, 4, Adr::AbsoluteY),
    Opcode::new(Asm::NOP, 2, Adr::Implied(Reg::PC)),
    Opcode::new(Asm::RRA, 7, Adr::AbsoluteY),
    Opcode::new(Asm::NOP, 4, Adr::AbsoluteX),
    Opcode::new(Asm::ADC, 4, Adr::AbsoluteX),
    Opcode::new(Asm::ROR, 7, Adr::AbsoluteX),
    Opcode::new(Asm::RRA, 7, Adr::AbsoluteX),
    Opcode::new(Asm::NOP, 2, Adr::Immediate),
    Opcode::new(Asm::STA, 6, Adr::IndirectX),
    Opcode::new(Asm::NOP, 2, Adr::Immediate),
    Opcode::new(Asm::SAX, 6, Adr::IndirectX),
    Opcode::new(Asm::STY, 3, Adr::ZeroPage),
    Opcode::new(Asm::STA, 3, Adr::ZeroPage),
    Opcode::new(Asm::STX, 3, Adr::ZeroPage),
    Opcode::new(Asm::SAX, 3, Adr::ZeroPage),
    Opcode::new(Asm::DEY, 2, Adr::Implied(Reg::Y)),
    Opcode::new(Asm::NOP, 2, Adr::Immediate),
    Opcode::new(Asm::TXA, 2, Adr::Implied(Reg::AC)),
    Opcode::new(Asm::ANE, 2, Adr::Immediate),
    Opcode::new(Asm::STY, 4, Adr::Absolute),
    Opcode::new(Asm::STA, 4, Adr::Absolute),
    Opcode::new(Asm::STX, 4, Adr::Absolute),
    Opcode::new(Asm::SAX, 4, Adr::Absolute),
    Opcode::new(Asm::BCC, 2, Adr::Relative),
    Opcode::new(Asm::STA, 6, Adr::IndirectY),
    Opcode::new(Asm::NOP, 2, Adr::Immediate),
    Opcode::new(Asm::SHA, 5, Adr::IndirectY),
    Opcode::new(Asm::STY, 4, Adr::ZeroPageX),
    Opcode::new(Asm::STA, 4, Adr::ZeroPageX),
    Opcode::new(Asm::STX, 4, Adr::ZeroPageY),
    Opcode::new(Asm::SAX, 4, Adr::ZeroPageY),
    Opcode::new(Asm::TYA, 2, Adr::Implied(Reg::AC)),
    Opcode::new(Asm::STA, 5, Adr::AbsoluteY),
    Opcode::new(Asm::TXS, 2, Adr::Implied(Reg::SP)),
    Opcode::new(Asm::TAS, 5, Adr::AbsoluteY),
    Opcode::new(Asm::SHY, 5, Adr::AbsoluteX),
    Opcode::new(Asm::STA, 5, Adr::AbsoluteX),
    Opcode::new(Asm::SHX, 5, Adr::AbsoluteY),
    Opcode::new(Asm::SHA, 5, Adr::AbsoluteY),
    Opcode::new(Asm::LDY, 2, Adr::Immediate),
    Opcode::new(Asm::LDA, 6, Adr::IndirectX),
    Opcode::new(Asm::LDX, 2, Adr::Immediate),
    Opcode::new(Asm::LAX, 6, Adr::IndirectX),
    Opcode::new(Asm::LDY, 3, Adr::ZeroPage),
    Opcode::new(Asm::LDA, 3, Adr::ZeroPage),
    Opcode::new(Asm::LDX, 3, Adr::ZeroPage),
    Opcode::new(Asm::LAX, 3, Adr::ZeroPage),
    Opcode::new(Asm::TAY, 2, Adr::Implied(Reg::Y)),
    Opcode::new(Asm::LDA, 2, Adr::Immediate),
    Opcode::new(Asm::TAX, 2, Adr::Implied(Reg::X)),
    Opcode::new(Asm::LXA, 2, Adr::Immediate),
    Opcode::new(Asm::LDY, 4, Adr::Absolute),
    Opcode::new(Asm::LDA, 4, Adr::Absolute),
    Opcode::new(Asm::LDX, 4, Adr::Absolute),
    Opcode::new(Asm::LAX, 4, Adr::Absolute),
    Opcode::new(Asm::BCS, 2, Adr::Relative),
    Opcode::new(Asm::LDA, 5, Adr::IndirectY),
    Opcode::new(Asm::JAM, 0, Adr::Immediate),
    Opcode::new(Asm::LAX, 5, Adr::IndirectY),
    Opcode::new(Asm::LDY, 4, Adr::ZeroPageX),
    Opcode::new(Asm::LDA, 4, Adr::ZeroPageX),
    Opcode::new(Asm::LDX, 4, Adr::ZeroPageY),
    Opcode::new(Asm::LAX, 4, Adr::ZeroPageY),
    Opcode::new(Asm::CLV, 2, Adr::Implied(Reg::SR)),
    Opcode::new(Asm::LDA, 4, Adr::AbsoluteY),
    Opcode::new(Asm::TSX, 2, Adr::Implied(Reg::X)),
    Opcode::new(Asm::LAS, 4, Adr::AbsoluteY),
    Opcode::new(Asm::LDY, 4, Adr::AbsoluteX),
    Opcode::new(Asm::LDA, 4, Adr::AbsoluteX),
    Opcode::new(Asm::LDX, 4, Adr::AbsoluteY),
    Opcode::new(Asm::LAX, 4, Adr::AbsoluteY),
    Opcode::new(Asm::CPY, 2, Adr::Immediate),
    Opcode::new(Asm::CMP, 6, Adr::IndirectX),
    Opcode::new(Asm::NOP, 2, Adr::Immediate),
    Opcode::new(Asm::DCP, 8, Adr::IndirectX),
    Opcode::new(Asm::CPY, 3, Adr::ZeroPage),
    Opcode::new(Asm::CMP, 3, Adr::ZeroPage),
    Opcode::new(Asm::DEC, 5, Adr::ZeroPage),
    Opcode::new(Asm::DCP, 5, Adr::ZeroPage),
    Opcode::new(Asm::INY, 2, Adr::Implied(Reg::Y)),
    Opcode::new(Asm::CMP, 2, Adr::Immediate),
    Opcode::new(Asm::DEX, 2, Adr::Implied(Reg::X)),
    Opcode::new(Asm::SBX, 2, Adr::Immediate),
    Opcode::new(Asm::CPY, 4, Adr::Absolute),
    Opcode::new(Asm::CMP, 4, Adr::Absolute),
    Opcode::new(Asm::DEC, 6, Adr::Absolute),
    Opcode::new(Asm::DCP, 6, Adr::Absolute),
    Opcode::new(Asm::BNE, 2, Adr::Relative),
    Opcode::new(Asm::CMP, 5, Adr::IndirectY),
    Opcode::new(Asm::JAM, 0, Adr::Immediate),
    Opcode::new(Asm::DCP, 8, Adr::IndirectY),
    Opcode::new(Asm::NOP, 4, Adr::ZeroPageX),
    Opcode::new(Asm::CMP, 4, Adr::ZeroPageX),
    Opcode::new(Asm::DEC, 6, Adr::ZeroPageX),
    Opcode::new(Asm::DCP, 6, Adr::ZeroPageX),
    Opcode::new(Asm::CLD, 2, Adr::Implied(Reg::SR)),
    Opcode::new(Asm::CMP, 4, Adr::AbsoluteY),
    Opcode::new(Asm::NOP, 2, Adr::Implied(Reg::PC)),
    Opcode::new(Asm::DCP, 7, Adr::AbsoluteY),
    Opcode::new(Asm::NOP, 4, Adr::AbsoluteX),
    Opcode::new(Asm::CMP, 4, Adr::AbsoluteX),
    Opcode::new(Asm::DEC, 7, Adr::AbsoluteX),
    Opcode::new(Asm::DCP, 7, Adr::AbsoluteX),
    Opcode::new(Asm::CPX, 2, Adr::Immediate),
    Opcode::new(Asm::SBC, 6, Adr::IndirectX),
    Opcode::new(Asm::NOP, 2, Adr::Immediate),
    Opcode::new(Asm::ISB, 8, Adr::IndirectX),
    Opcode::new(Asm::CPX, 3, Adr::ZeroPage),
    Opcode::new(Asm::SBC, 3, Adr::ZeroPage),
    Opcode::new(Asm::INC, 5, Adr::ZeroPage),
    Opcode::new(Asm::ISB, 5, Adr::ZeroPage),
    Opcode::new(Asm::INX, 2, Adr::Implied(Reg::X)),
    Opcode::new(Asm::SBC, 2, Adr::Immediate),
    Opcode::new(Asm::NOP, 2, Adr::Implied(Reg::PC)),
    Opcode::new(Asm::SBC, 2, Adr::Immediate),
    Opcode::new(Asm::CPX, 4, Adr::Absolute),
    Opcode::new(Asm::SBC, 4, Adr::Absolute),
    Opcode::new(Asm::INC, 6, Adr::Absolute),
    Opcode::new(Asm::ISB, 6, Adr::Absolute),
    Opcode::new(Asm::BEQ, 2, Adr::Relative),
    Opcode::new(Asm::SBC, 5, Adr::IndirectY),
    Opcode::new(Asm::JAM, 0, Adr::Immediate),
    Opcode::new(Asm::ISB, 8, Adr::IndirectY),
    Opcode::new(Asm::NOP, 4, Adr::ZeroPageX),
    Opcode::new(Asm::SBC, 4, Adr::ZeroPageX),
    Opcode::new(Asm::INC, 6, Adr::ZeroPageX),
    Opcode::new(Asm::ISB, 6, Adr::ZeroPageX),
    Opcode::new(Asm::SED, 2, Adr::Implied(Reg::SR)),
    Opcode::new(Asm::SBC, 4, Adr::AbsoluteY),
    Opcode::new(Asm::NOP, 2, Adr::Implied(Reg::PC)),
    Opcode::new(Asm::ISB, 7, Adr::AbsoluteY),
    Opcode::new(Asm::NOP, 4, Adr::AbsoluteX),
    Opcode::new(Asm::SBC, 4, Adr::AbsoluteX),
    Opcode::new(Asm::INC, 7, Adr::AbsoluteX),
    Opcode::new(Asm::ISB, 7, Adr::AbsoluteX),
];
