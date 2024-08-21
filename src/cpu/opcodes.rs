// https://www.masswerk.at/6502/6502_instruction_set.html#layout

use super::{address::AddressMode, register::CpuRegister};

#[derive(Debug, Clone, Copy)]
pub struct Opcode {
    pub asm: Asm,
    pub adr_mode: AddressMode,
    pub cycle: u8,
}

impl Opcode {
    const fn new(asm: Asm, adr_mode: AddressMode, cycle: u8) -> Self {
        Self {
            asm,
            adr_mode,
            cycle,
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

use {AddressMode as Adr, CpuRegister as Reg};

pub const OPCODES: [Opcode; 256] = [
    Opcode::new(Asm::BRK, Adr::Implied(Reg::PC), 7),
    Opcode::new(Asm::ORA, Adr::IndirectX, 6),
    Opcode::new(Asm::JAM, Adr::Immediate, 0),
    Opcode::new(Asm::SLO, Adr::IndirectX, 8),
    Opcode::new(Asm::NOP, Adr::ZeroPage, 3),
    Opcode::new(Asm::ORA, Adr::ZeroPage, 3),
    Opcode::new(Asm::ASL, Adr::ZeroPage, 5),
    Opcode::new(Asm::SLO, Adr::ZeroPage, 5),
    Opcode::new(Asm::PHP, Adr::Implied(Reg::SR), 3),
    Opcode::new(Asm::ORA, Adr::Immediate, 2),
    Opcode::new(Asm::ASL, Adr::Implied(Reg::AC), 2),
    Opcode::new(Asm::ANC, Adr::Immediate, 2),
    Opcode::new(Asm::NOP, Adr::Absolute, 4),
    Opcode::new(Asm::ORA, Adr::Absolute, 4),
    Opcode::new(Asm::ASL, Adr::Absolute, 6),
    Opcode::new(Asm::SLO, Adr::Absolute, 6),
    Opcode::new(Asm::BPL, Adr::Relative, 2),
    Opcode::new(Asm::ORA, Adr::IndirectY, 5),
    Opcode::new(Asm::JAM, Adr::Immediate, 0),
    Opcode::new(Asm::SLO, Adr::IndirectY, 8),
    Opcode::new(Asm::NOP, Adr::ZeroPageX, 4),
    Opcode::new(Asm::ORA, Adr::ZeroPageX, 4),
    Opcode::new(Asm::ASL, Adr::ZeroPageX, 6),
    Opcode::new(Asm::SLO, Adr::ZeroPageX, 6),
    Opcode::new(Asm::CLC, Adr::Implied(Reg::SR), 2),
    Opcode::new(Asm::ORA, Adr::AbsoluteY, 4),
    Opcode::new(Asm::NOP, Adr::Implied(Reg::PC), 2),
    Opcode::new(Asm::SLO, Adr::AbsoluteY, 7),
    Opcode::new(Asm::NOP, Adr::AbsoluteX, 4),
    Opcode::new(Asm::ORA, Adr::AbsoluteX, 4),
    Opcode::new(Asm::ASL, Adr::AbsoluteX, 7),
    Opcode::new(Asm::SLO, Adr::AbsoluteX, 7),
    Opcode::new(Asm::JSR, Adr::Absolute, 6),
    Opcode::new(Asm::AND, Adr::IndirectX, 6),
    Opcode::new(Asm::JAM, Adr::Immediate, 0),
    Opcode::new(Asm::RLA, Adr::IndirectX, 8),
    Opcode::new(Asm::BIT, Adr::ZeroPage, 3),
    Opcode::new(Asm::AND, Adr::ZeroPage, 3),
    Opcode::new(Asm::ROL, Adr::ZeroPage, 5),
    Opcode::new(Asm::RLA, Adr::ZeroPage, 5),
    Opcode::new(Asm::PLP, Adr::Implied(Reg::SR), 4),
    Opcode::new(Asm::AND, Adr::Immediate, 2),
    Opcode::new(Asm::ROL, Adr::Implied(Reg::AC), 2),
    Opcode::new(Asm::ANC, Adr::Immediate, 2),
    Opcode::new(Asm::BIT, Adr::Absolute, 4),
    Opcode::new(Asm::AND, Adr::Absolute, 4),
    Opcode::new(Asm::ROL, Adr::Absolute, 6),
    Opcode::new(Asm::RLA, Adr::Absolute, 6),
    Opcode::new(Asm::BMI, Adr::Relative, 2),
    Opcode::new(Asm::AND, Adr::IndirectY, 5),
    Opcode::new(Asm::JAM, Adr::Immediate, 0),
    Opcode::new(Asm::RLA, Adr::IndirectY, 8),
    Opcode::new(Asm::NOP, Adr::ZeroPageX, 4),
    Opcode::new(Asm::AND, Adr::ZeroPageX, 4),
    Opcode::new(Asm::ROL, Adr::ZeroPageX, 6),
    Opcode::new(Asm::RLA, Adr::ZeroPageX, 6),
    Opcode::new(Asm::SEC, Adr::Implied(Reg::SR), 2),
    Opcode::new(Asm::AND, Adr::AbsoluteY, 4),
    Opcode::new(Asm::NOP, Adr::Implied(Reg::PC), 2),
    Opcode::new(Asm::RLA, Adr::AbsoluteY, 7),
    Opcode::new(Asm::NOP, Adr::AbsoluteX, 4),
    Opcode::new(Asm::AND, Adr::AbsoluteX, 4),
    Opcode::new(Asm::ROL, Adr::AbsoluteX, 7),
    Opcode::new(Asm::RLA, Adr::AbsoluteX, 7),
    Opcode::new(Asm::RTI, Adr::Implied(Reg::PC), 6),
    Opcode::new(Asm::EOR, Adr::IndirectX, 6),
    Opcode::new(Asm::JAM, Adr::Immediate, 0),
    Opcode::new(Asm::SRE, Adr::IndirectX, 8),
    Opcode::new(Asm::NOP, Adr::ZeroPage, 3),
    Opcode::new(Asm::EOR, Adr::ZeroPage, 3),
    Opcode::new(Asm::LSR, Adr::ZeroPage, 5),
    Opcode::new(Asm::SRE, Adr::ZeroPage, 5),
    Opcode::new(Asm::PHA, Adr::Implied(Reg::AC), 3),
    Opcode::new(Asm::EOR, Adr::Immediate, 2),
    Opcode::new(Asm::LSR, Adr::Implied(Reg::AC), 2),
    Opcode::new(Asm::ALR, Adr::Immediate, 2),
    Opcode::new(Asm::JMP, Adr::Absolute, 3),
    Opcode::new(Asm::EOR, Adr::Absolute, 4),
    Opcode::new(Asm::LSR, Adr::Absolute, 6),
    Opcode::new(Asm::SRE, Adr::Absolute, 6),
    Opcode::new(Asm::BVC, Adr::Relative, 2),
    Opcode::new(Asm::EOR, Adr::IndirectY, 5),
    Opcode::new(Asm::JAM, Adr::Immediate, 0),
    Opcode::new(Asm::SRE, Adr::IndirectY, 8),
    Opcode::new(Asm::NOP, Adr::ZeroPageX, 4),
    Opcode::new(Asm::EOR, Adr::ZeroPageX, 4),
    Opcode::new(Asm::LSR, Adr::ZeroPageX, 6),
    Opcode::new(Asm::SRE, Adr::ZeroPageX, 6),
    Opcode::new(Asm::CLI, Adr::Implied(Reg::SR), 2),
    Opcode::new(Asm::EOR, Adr::AbsoluteY, 4),
    Opcode::new(Asm::NOP, Adr::Implied(Reg::PC), 2),
    Opcode::new(Asm::SRE, Adr::AbsoluteY, 7),
    Opcode::new(Asm::NOP, Adr::AbsoluteX, 4),
    Opcode::new(Asm::EOR, Adr::AbsoluteX, 4),
    Opcode::new(Asm::LSR, Adr::AbsoluteX, 7),
    Opcode::new(Asm::SRE, Adr::AbsoluteX, 7),
    Opcode::new(Asm::RTS, Adr::Implied(Reg::PC), 6),
    Opcode::new(Asm::ADC, Adr::IndirectX, 6),
    Opcode::new(Asm::JAM, Adr::Immediate, 0),
    Opcode::new(Asm::RRA, Adr::IndirectX, 8),
    Opcode::new(Asm::NOP, Adr::ZeroPage, 3),
    Opcode::new(Asm::ADC, Adr::ZeroPage, 3),
    Opcode::new(Asm::ROR, Adr::ZeroPage, 5),
    Opcode::new(Asm::RRA, Adr::ZeroPage, 5),
    Opcode::new(Asm::PLA, Adr::Implied(Reg::AC), 4),
    Opcode::new(Asm::ADC, Adr::Immediate, 2),
    Opcode::new(Asm::ROR, Adr::Implied(Reg::AC), 2),
    Opcode::new(Asm::ARR, Adr::Immediate, 2),
    Opcode::new(Asm::JMP, Adr::Indirect, 5),
    Opcode::new(Asm::ADC, Adr::Absolute, 4),
    Opcode::new(Asm::ROR, Adr::Absolute, 6),
    Opcode::new(Asm::RRA, Adr::Absolute, 6),
    Opcode::new(Asm::BVS, Adr::Relative, 2),
    Opcode::new(Asm::ADC, Adr::IndirectY, 5),
    Opcode::new(Asm::JAM, Adr::Immediate, 0),
    Opcode::new(Asm::RRA, Adr::IndirectY, 8),
    Opcode::new(Asm::NOP, Adr::ZeroPageX, 4),
    Opcode::new(Asm::ADC, Adr::ZeroPageX, 4),
    Opcode::new(Asm::ROR, Adr::ZeroPageX, 6),
    Opcode::new(Asm::RRA, Adr::ZeroPageX, 6),
    Opcode::new(Asm::SEI, Adr::Implied(Reg::SR), 2),
    Opcode::new(Asm::ADC, Adr::AbsoluteY, 4),
    Opcode::new(Asm::NOP, Adr::Implied(Reg::PC), 2),
    Opcode::new(Asm::RRA, Adr::AbsoluteY, 7),
    Opcode::new(Asm::NOP, Adr::AbsoluteX, 4),
    Opcode::new(Asm::ADC, Adr::AbsoluteX, 4),
    Opcode::new(Asm::ROR, Adr::AbsoluteX, 7),
    Opcode::new(Asm::RRA, Adr::AbsoluteX, 7),
    Opcode::new(Asm::NOP, Adr::Immediate, 2),
    Opcode::new(Asm::STA, Adr::IndirectX, 6),
    Opcode::new(Asm::NOP, Adr::Immediate, 2),
    Opcode::new(Asm::SAX, Adr::IndirectX, 6),
    Opcode::new(Asm::STY, Adr::ZeroPage, 3),
    Opcode::new(Asm::STA, Adr::ZeroPage, 3),
    Opcode::new(Asm::STX, Adr::ZeroPage, 3),
    Opcode::new(Asm::SAX, Adr::ZeroPage, 3),
    Opcode::new(Asm::DEY, Adr::Implied(Reg::Y), 2),
    Opcode::new(Asm::NOP, Adr::Immediate, 2),
    Opcode::new(Asm::TXA, Adr::Implied(Reg::AC), 2),
    Opcode::new(Asm::ANE, Adr::Immediate, 2),
    Opcode::new(Asm::STY, Adr::Absolute, 4),
    Opcode::new(Asm::STA, Adr::Absolute, 4),
    Opcode::new(Asm::STX, Adr::Absolute, 4),
    Opcode::new(Asm::SAX, Adr::Absolute, 4),
    Opcode::new(Asm::BCC, Adr::Relative, 2),
    Opcode::new(Asm::STA, Adr::IndirectY, 6),
    Opcode::new(Asm::NOP, Adr::Immediate, 2),
    Opcode::new(Asm::SHA, Adr::IndirectY, 5),
    Opcode::new(Asm::STY, Adr::ZeroPageX, 4),
    Opcode::new(Asm::STA, Adr::ZeroPageX, 4),
    Opcode::new(Asm::STX, Adr::ZeroPageY, 4),
    Opcode::new(Asm::SAX, Adr::ZeroPageY, 4),
    Opcode::new(Asm::TYA, Adr::Implied(Reg::AC), 2),
    Opcode::new(Asm::STA, Adr::AbsoluteY, 5),
    Opcode::new(Asm::TXS, Adr::Implied(Reg::SP), 2),
    Opcode::new(Asm::TAS, Adr::AbsoluteY, 5),
    Opcode::new(Asm::SHY, Adr::AbsoluteX, 5),
    Opcode::new(Asm::STA, Adr::AbsoluteX, 5),
    Opcode::new(Asm::SHX, Adr::AbsoluteY, 5),
    Opcode::new(Asm::SHA, Adr::AbsoluteY, 5),
    Opcode::new(Asm::LDY, Adr::Immediate, 2),
    Opcode::new(Asm::LDA, Adr::IndirectX, 6),
    Opcode::new(Asm::LDX, Adr::Immediate, 2),
    Opcode::new(Asm::LAX, Adr::IndirectX, 6),
    Opcode::new(Asm::LDY, Adr::ZeroPage, 3),
    Opcode::new(Asm::LDA, Adr::ZeroPage, 3),
    Opcode::new(Asm::LDX, Adr::ZeroPage, 3),
    Opcode::new(Asm::LAX, Adr::ZeroPage, 3),
    Opcode::new(Asm::TAY, Adr::Implied(Reg::Y), 2),
    Opcode::new(Asm::LDA, Adr::Immediate, 2),
    Opcode::new(Asm::TAX, Adr::Implied(Reg::X), 2),
    Opcode::new(Asm::LXA, Adr::Immediate, 2),
    Opcode::new(Asm::LDY, Adr::Absolute, 4),
    Opcode::new(Asm::LDA, Adr::Absolute, 4),
    Opcode::new(Asm::LDX, Adr::Absolute, 4),
    Opcode::new(Asm::LAX, Adr::Absolute, 4),
    Opcode::new(Asm::BCS, Adr::Relative, 2),
    Opcode::new(Asm::LDA, Adr::IndirectY, 5),
    Opcode::new(Asm::JAM, Adr::Immediate, 0),
    Opcode::new(Asm::LAX, Adr::IndirectY, 5),
    Opcode::new(Asm::LDY, Adr::ZeroPageX, 4),
    Opcode::new(Asm::LDA, Adr::ZeroPageX, 4),
    Opcode::new(Asm::LDX, Adr::ZeroPageY, 4),
    Opcode::new(Asm::LAX, Adr::ZeroPageY, 4),
    Opcode::new(Asm::CLV, Adr::Implied(Reg::SR), 2),
    Opcode::new(Asm::LDA, Adr::AbsoluteY, 4),
    Opcode::new(Asm::TSX, Adr::Implied(Reg::X), 2),
    Opcode::new(Asm::LAS, Adr::AbsoluteY, 4),
    Opcode::new(Asm::LDY, Adr::AbsoluteX, 4),
    Opcode::new(Asm::LDA, Adr::AbsoluteX, 4),
    Opcode::new(Asm::LDX, Adr::AbsoluteY, 4),
    Opcode::new(Asm::LAX, Adr::AbsoluteY, 4),
    Opcode::new(Asm::CPY, Adr::Immediate, 2),
    Opcode::new(Asm::CMP, Adr::IndirectX, 6),
    Opcode::new(Asm::NOP, Adr::Immediate, 2),
    Opcode::new(Asm::DCP, Adr::IndirectX, 8),
    Opcode::new(Asm::CPY, Adr::ZeroPage, 3),
    Opcode::new(Asm::CMP, Adr::ZeroPage, 3),
    Opcode::new(Asm::DEC, Adr::ZeroPage, 5),
    Opcode::new(Asm::DCP, Adr::ZeroPage, 5),
    Opcode::new(Asm::INY, Adr::Implied(Reg::Y), 2),
    Opcode::new(Asm::CMP, Adr::Immediate, 2),
    Opcode::new(Asm::DEX, Adr::Implied(Reg::X), 2),
    Opcode::new(Asm::SBX, Adr::Immediate, 2),
    Opcode::new(Asm::CPY, Adr::Absolute, 4),
    Opcode::new(Asm::CMP, Adr::Absolute, 4),
    Opcode::new(Asm::DEC, Adr::Absolute, 6),
    Opcode::new(Asm::DCP, Adr::Absolute, 6),
    Opcode::new(Asm::BNE, Adr::Relative, 2),
    Opcode::new(Asm::CMP, Adr::IndirectY, 5),
    Opcode::new(Asm::JAM, Adr::Immediate, 0),
    Opcode::new(Asm::DCP, Adr::IndirectY, 8),
    Opcode::new(Asm::NOP, Adr::ZeroPageX, 4),
    Opcode::new(Asm::CMP, Adr::ZeroPageX, 4),
    Opcode::new(Asm::DEC, Adr::ZeroPageX, 6),
    Opcode::new(Asm::DCP, Adr::ZeroPageX, 6),
    Opcode::new(Asm::CLD, Adr::Implied(Reg::SR), 2),
    Opcode::new(Asm::CMP, Adr::AbsoluteY, 4),
    Opcode::new(Asm::NOP, Adr::Implied(Reg::PC), 2),
    Opcode::new(Asm::DCP, Adr::AbsoluteY, 7),
    Opcode::new(Asm::NOP, Adr::AbsoluteX, 4),
    Opcode::new(Asm::CMP, Adr::AbsoluteX, 4),
    Opcode::new(Asm::DEC, Adr::AbsoluteX, 7),
    Opcode::new(Asm::DCP, Adr::AbsoluteX, 7),
    Opcode::new(Asm::CPX, Adr::Immediate, 2),
    Opcode::new(Asm::SBC, Adr::IndirectX, 6),
    Opcode::new(Asm::NOP, Adr::Immediate, 2),
    Opcode::new(Asm::ISB, Adr::IndirectX, 8),
    Opcode::new(Asm::CPX, Adr::ZeroPage, 3),
    Opcode::new(Asm::SBC, Adr::ZeroPage, 3),
    Opcode::new(Asm::INC, Adr::ZeroPage, 5),
    Opcode::new(Asm::ISB, Adr::ZeroPage, 5),
    Opcode::new(Asm::INX, Adr::Implied(Reg::X), 2),
    Opcode::new(Asm::SBC, Adr::Immediate, 2),
    Opcode::new(Asm::NOP, Adr::Implied(Reg::PC), 2),
    Opcode::new(Asm::SBC, Adr::Immediate, 2),
    Opcode::new(Asm::CPX, Adr::Absolute, 4),
    Opcode::new(Asm::SBC, Adr::Absolute, 4),
    Opcode::new(Asm::INC, Adr::Absolute, 6),
    Opcode::new(Asm::ISB, Adr::Absolute, 6),
    Opcode::new(Asm::BEQ, Adr::Relative, 2),
    Opcode::new(Asm::SBC, Adr::IndirectY, 5),
    Opcode::new(Asm::JAM, Adr::Immediate, 0),
    Opcode::new(Asm::ISB, Adr::IndirectY, 8),
    Opcode::new(Asm::NOP, Adr::ZeroPageX, 4),
    Opcode::new(Asm::SBC, Adr::ZeroPageX, 4),
    Opcode::new(Asm::INC, Adr::ZeroPageX, 6),
    Opcode::new(Asm::ISB, Adr::ZeroPageX, 6),
    Opcode::new(Asm::SED, Adr::Implied(Reg::SR), 2),
    Opcode::new(Asm::SBC, Adr::AbsoluteY, 4),
    Opcode::new(Asm::NOP, Adr::Implied(Reg::PC), 2),
    Opcode::new(Asm::ISB, Adr::AbsoluteY, 7),
    Opcode::new(Asm::NOP, Adr::AbsoluteX, 4),
    Opcode::new(Asm::SBC, Adr::AbsoluteX, 4),
    Opcode::new(Asm::INC, Adr::AbsoluteX, 7),
    Opcode::new(Asm::ISB, Adr::AbsoluteX, 7),
];
