// https://www.masswerk.at/6502/6502_instruction_set.html

mod address;
mod opcodes;
mod register;

pub mod interrupt;

use address::{Address, AddressMode};
use opcodes::{Asm, OPCODES};

use crate::{
    apu::Apu,
    bus::{Bus, DmaState, MainBus},
    cpu::{
        interrupt::{Interrupt, INTERRUPT_LATENCY},
        register::{status_flag, CpuRegister, StatusRegister},
    },
    utils::{BitFlag, Clock, Reset},
};

use std::{
    cell::RefCell,
    fmt,
    ops::{BitAnd, BitOr, BitXor},
    rc::Rc,
};

const STACK_START: u16 = 0x100;

pub struct Cpu {
    pc: u16,
    ac: u8,
    x: u8,
    y: u8,
    sr: StatusRegister,
    sp: u8,
    cycle: u64,
    dma: Option<DmaState>,
    interrupt: Option<Interrupt>,
    pub(crate) bus: MainBus,
    pub(crate) apu: Rc<RefCell<Apu>>,
}

impl Cpu {
    pub fn new(bus: MainBus) -> Self {
        Self {
            pc: 0x00,
            ac: 0x00,
            x: 0x00,
            y: 0x00,
            sr: StatusRegister::default(),
            sp: 0x00,
            apu: bus.apu.clone(),
            bus,
            cycle: 0,
            dma: None,
            interrupt: Some(Interrupt::Reset),
        }
    }

    pub fn step(&mut self) {
        let cycles = self.cycle();
        let mut apu = self.apu.borrow_mut();

        for _ in 0..cycles {
            self.cycle += 1;
            self.bus.tick();
            apu.tick();

            if let Some(cycles) = apu.take_dmc_cycles() {
                self.cycle += cycles as u64;
            }
        }
    }

    pub fn cycle(&mut self) -> u8 {
        self.interrupt = self
            .interrupt
            .or_else(|| self.bus.poll_interrupt())
            .or_else(|| self.apu.borrow().poll_irq());

        if let Some(interrupt) = self.interrupt.take() {
            if self.handle_interrupt(interrupt) {
                return INTERRUPT_LATENCY;
            }
        }

        if self.dma.is_some() && self.apu.borrow().incoming_dma() {
            return 1; // DMC DMA is taken causing an extra alignment cycle
        }

        if let Some(address) = self.bus.poll_dma() {
            self.dma = Some(DmaState::new(address));

            if self.cycle % 2 == 1 {
                return 1;
            }
        }

        if let Some(dma) = self.dma.as_mut() {
            let end = self.bus.dma_cycle(dma);

            if end {
                self.dma.take();
            }

            return if end { 2 } else { 1 };
        }

        let opcode = self.bus.read_u8(self.pc);

        self.increment_pc(1);
        self.execute(opcode)
    }

    pub fn execute(&mut self, opcode: u8) -> u8 {
        let opcode = OPCODES[opcode as usize];
        let adr_mode = opcode.adr_mode;
        let (address, crossed_boundary) = self.get_address(adr_mode);
        let prev_pc = self.pc;

        match opcode.asm {
            Asm::LDA => self.lda(address),
            Asm::LDX => self.ldx(address),
            Asm::LDY => self.ldy(address),
            Asm::STA => self.sta(address),
            Asm::STX => self.stx(address),
            Asm::STY => self.sty(address),
            Asm::TAX => self.tax(),
            Asm::TAY => self.tay(),
            Asm::TSX => self.tsx(),
            Asm::TXA => self.txa(),
            Asm::TXS => self.txs(),
            Asm::TYA => self.tya(),
            Asm::PHA => self.pha(),
            Asm::PHP => self.php(),
            Asm::PLA => self.pla(),
            Asm::PLP => self.plp(),
            Asm::DEC => self.dec(address),
            Asm::DEX => self.dex(),
            Asm::DEY => self.dey(),
            Asm::INC => self.inc(address),
            Asm::INX => self.inx(),
            Asm::INY => self.iny(),
            Asm::ADC => self.adc(address),
            Asm::SBC => self.sbc(address),
            Asm::AND => self.and(address),
            Asm::EOR => self.eor(address),
            Asm::ORA => self.ora(address),
            Asm::ASL => self.asl(address),
            Asm::LSR => self.lsr(address),
            Asm::ROL => self.rol(address),
            Asm::ROR => self.ror(address),
            Asm::CLC => self.clc(),
            Asm::CLD => self.cld(),
            Asm::CLI => self.cli(),
            Asm::CLV => self.clv(),
            Asm::SEC => self.sec(),
            Asm::SED => self.sed(),
            Asm::SEI => self.sei(),
            Asm::CMP => self.cmp(address),
            Asm::CPX => self.cpx(address),
            Asm::CPY => self.cpy(address),
            Asm::BCC => self.bcc(address),
            Asm::BCS => self.bcs(address),
            Asm::BVC => self.bvc(address),
            Asm::BVS => self.bvs(address),
            Asm::BEQ => self.beq(address),
            Asm::BMI => self.bmi(address),
            Asm::BNE => self.bne(address),
            Asm::BPL => self.bpl(address),
            Asm::JMP => self.jmp(address),
            Asm::JSR => self.jsr(address),
            Asm::RTS => self.rts(),
            Asm::BRK => self.brk(),
            Asm::RTI => self.rti(),
            Asm::BIT => self.bit(address),
            Asm::NOP => self.nop(),
            Asm::ALR => self.alr(address),
            Asm::ARR => self.arr(address),
            Asm::ANC => self.anc(address),
            Asm::DCP => self.dcp(address),
            Asm::ISB => self.isb(address),
            Asm::LAS => self.las(address),
            Asm::LAX => self.lax(address),
            Asm::RLA => self.rla(address),
            Asm::RRA => self.rra(address),
            Asm::SAX => self.sax(address),
            Asm::SBX => self.sbx(address),
            Asm::SLO => self.slo(address),
            Asm::SRE => self.sre(address),
            asm => panic!("Unstable opcode: {asm:?}"),
        }

        let jumped = prev_pc != self.pc;
        let total_cycles = opcode.total_cycles(crossed_boundary, jumped);

        if !jumped {
            self.increment_pc(opcode.len() - 1);
        }

        total_cycles
    }

    fn increment_pc(&mut self, value: u8) {
        self.pc = self.pc.wrapping_add(value as u16);
    }

    fn get_effective_address(&self, address: u16, offset: u8) -> (u16, bool) {
        let [low, high] = address.to_le_bytes();
        let low = low as u16 + offset as u16;
        let high = (high as u16) << 8;
        let crossed = (low >> 8) > 0;
        let address = high.wrapping_add(low);
        (address, crossed)
    }

    fn get_address(&mut self, adr_mode: AddressMode) -> (Address, bool) {
        match adr_mode {
            AddressMode::Implied(reg) => (Address::Register(reg), false),
            AddressMode::Immediate => (Address::Memory(self.pc), false),
            AddressMode::Absolute => {
                let address = self.bus.read_u16(self.pc);
                (Address::Memory(address), false)
            }
            AddressMode::ZeroPage => {
                let address = self.bus.read_u8(self.pc) as u16;
                (Address::Memory(address), false)
            }
            AddressMode::AbsoluteX => {
                let address = self.bus.read_u16(self.pc);
                let (address, crossed) = self.get_effective_address(address, self.x);
                (Address::Memory(address), crossed)
            }
            AddressMode::AbsoluteY => {
                let address = self.bus.read_u16(self.pc);
                let (address, crossed) = self.get_effective_address(address, self.y);
                (Address::Memory(address), crossed)
            }
            AddressMode::ZeroPageX => {
                let address = self.bus.read_u8(self.pc).wrapping_add(self.x);
                (Address::Memory(address as u16), false)
            }
            AddressMode::ZeroPageY => {
                let address = self.bus.read_u8(self.pc).wrapping_add(self.y);
                (Address::Memory(address as u16), false)
            }
            AddressMode::Indirect => {
                let lookup_adr = self.bus.read_u16(self.pc);
                let address = match lookup_adr & 0xFF == 0xFF {
                    true => u16::from_le_bytes([
                        self.bus.read_u8(lookup_adr),
                        self.bus.read_u8(lookup_adr & 0xFF00),
                    ]),
                    false => self.bus.read_u16(lookup_adr),
                };
                (Address::Memory(address), false)
            }
            AddressMode::IndirectX => {
                let lookup_adr = self.bus.read_u8(self.pc).wrapping_add(self.x);
                let low = self.bus.read_u8(lookup_adr as u16);
                let high = self.bus.read_u8(lookup_adr.wrapping_add(1) as u16);
                let address = u16::from_le_bytes([low, high]);
                (Address::Memory(address), false)
            }
            AddressMode::IndirectY => {
                let lookup_adr = self.bus.read_u8(self.pc);
                let low = self.bus.read_u8(lookup_adr as u16);
                let high = self.bus.read_u8(lookup_adr.wrapping_add(1) as u16);
                let address = u16::from_le_bytes([low, high]);
                let (address, crossed) = self.get_effective_address(address, self.y);
                (Address::Memory(address), crossed)
            }
            AddressMode::Relative => {
                let offset = self.bus.read_u8(self.pc);
                let pc = self.pc.wrapping_add(1); // page boundary is relative to the next instruction start
                let address = pc.wrapping_add_signed(offset as i8 as i16);
                let crossed = pc & 0xFF00 != address & 0xFF00;
                (Address::Memory(address), crossed)
            }
        }
    }

    fn read_address(&mut self, address: Address) -> u8 {
        match address {
            Address::Memory(adr) => self.bus.read_u8(adr),
            Address::Register(reg) => self.read_register(reg),
        }
    }

    fn write_address(&mut self, address: Address, value: u8) {
        match address {
            Address::Memory(adr) => self.bus.write_u8(adr, value),
            Address::Register(reg) => self.write_register(reg, value),
        }
    }

    fn read_register(&self, register: CpuRegister) -> u8 {
        match register {
            CpuRegister::AC => self.ac,
            CpuRegister::X => self.x,
            CpuRegister::Y => self.y,
            CpuRegister::SP => self.sp,
            _ => unreachable!(),
        }
    }

    fn write_register(&mut self, register: CpuRegister, value: u8) {
        match register {
            CpuRegister::AC => self.ac = value,
            CpuRegister::X => self.x = value,
            CpuRegister::Y => self.y = value,
            CpuRegister::SP => self.sp = value,
            _ => unreachable!(),
        };
    }

    fn push_stack_u8(&mut self, value: u8) {
        self.bus.write_u8(self.sp as u16 + STACK_START, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pull_stack_u8(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        self.bus.read_u8(self.sp as u16 + STACK_START)
    }

    fn push_stack_u16(&mut self, value: u16) {
        let [low, high] = value.to_le_bytes();
        self.push_stack_u8(high);
        self.push_stack_u8(low);
    }

    fn pull_stack_u16(&mut self) -> u16 {
        let low = self.pull_stack_u8();
        let high = self.pull_stack_u8();
        u16::from_le_bytes([low, high])
    }

    fn lda(&mut self, address: Address) {
        self.load(address, CpuRegister::AC);
    }

    fn ldx(&mut self, address: Address) {
        self.load(address, CpuRegister::X);
    }

    fn ldy(&mut self, address: Address) {
        self.load(address, CpuRegister::Y);
    }

    fn sta(&mut self, address: Address) {
        self.store(address, CpuRegister::AC);
    }

    fn stx(&mut self, address: Address) {
        self.store(address, CpuRegister::X);
    }

    fn sty(&mut self, address: Address) {
        self.store(address, CpuRegister::Y);
    }

    fn tax(&mut self) {
        self.transfer(CpuRegister::AC, CpuRegister::X, true);
    }

    fn tay(&mut self) {
        self.transfer(CpuRegister::AC, CpuRegister::Y, true);
    }

    fn tsx(&mut self) {
        self.transfer(CpuRegister::SP, CpuRegister::X, true);
    }

    fn txa(&mut self) {
        self.transfer(CpuRegister::X, CpuRegister::AC, true);
    }

    fn txs(&mut self) {
        self.transfer(CpuRegister::X, CpuRegister::SP, false);
    }

    fn tya(&mut self) {
        self.transfer(CpuRegister::Y, CpuRegister::AC, true);
    }

    fn pha(&mut self) {
        self.push_stack_u8(self.ac);
    }

    fn php(&mut self) {
        let mut status = self.sr.value();
        status.set(status_flag::B);
        self.push_stack_u8(status);
    }

    fn pla(&mut self) {
        let value = self.pull_stack_u8();
        self.sr.update_negative(value);
        self.sr.update_zero(value);
        self.ac = value;
    }

    fn plp(&mut self) {
        let status = (self.pull_stack_u8() & 0b1100_1111)
            | ((self.sr.get(status_flag::B)) << 4)
            | (self.sr.get(status_flag::__) << 5);
        self.sr.assign(status);
    }

    fn dec(&mut self, address: Address) {
        self.decrement(address);
    }

    fn dex(&mut self) {
        self.decrement(Address::Register(CpuRegister::X));
    }

    fn dey(&mut self) {
        self.decrement(Address::Register(CpuRegister::Y));
    }

    fn inc(&mut self, address: Address) {
        self.increment(address);
    }

    fn inx(&mut self) {
        self.increment(Address::Register(CpuRegister::X));
    }

    fn iny(&mut self) {
        self.increment(Address::Register(CpuRegister::Y));
    }

    fn adc(&mut self, address: Address) {
        let rhs = self.read_address(address);
        self.add_to_accumulator(rhs);
    }

    fn sbc(&mut self, address: Address) {
        let rhs = self.read_address(address);
        self.add_to_accumulator(!rhs);
    }

    fn and(&mut self, address: Address) {
        self.binary_op(address, u8::bitand);
    }

    fn eor(&mut self, address: Address) {
        self.binary_op(address, u8::bitxor);
    }

    fn ora(&mut self, address: Address) {
        self.binary_op(address, u8::bitor);
    }

    fn asl(&mut self, address: Address) {
        let operand = self.read_address(address);
        let result = operand.wrapping_shl(1);
        self.sr.update(status_flag::C, operand >> 7 == 1);
        self.sr.update_negative(result);
        self.sr.update_zero(result);
        self.write_address(address, result);
    }

    fn lsr(&mut self, address: Address) {
        let operand = self.read_address(address);
        let result = operand.wrapping_shr(1);
        self.sr.update(status_flag::C, operand << 7 == 128);
        self.sr.update_negative(result);
        self.sr.update_zero(result);
        self.write_address(address, result);
    }

    fn rol(&mut self, address: Address) {
        let operand = self.read_address(address);
        let carry_bit = self.sr.get(status_flag::C);
        let result = operand.wrapping_shl(1) | carry_bit;
        self.sr.update(status_flag::C, operand >> 7 == 1);
        self.sr.update_negative(result);
        self.sr.update_zero(result);
        self.write_address(address, result);
    }

    fn ror(&mut self, address: Address) {
        let operand = self.read_address(address);
        let carry_bit = self.sr.get(status_flag::C);
        let result = operand.wrapping_shr(1) | (carry_bit << 7);
        self.sr.update(status_flag::C, operand & 1 == 1);
        self.sr.update_negative(result);
        self.sr.update_zero(result);
        self.write_address(address, result);
    }

    fn clc(&mut self) {
        self.sr.clear(status_flag::C);
    }

    fn cld(&mut self) {
        self.sr.clear(status_flag::D);
    }

    fn cli(&mut self) {
        self.sr.clear(status_flag::I);
    }

    fn clv(&mut self) {
        self.sr.clear(status_flag::V);
    }

    fn sec(&mut self) {
        self.sr.set(status_flag::C);
    }

    fn sei(&mut self) {
        self.sr.set(status_flag::I);
    }

    fn sed(&mut self) {
        self.sr.set(status_flag::D);
    }

    fn cmp(&mut self, address: Address) {
        self.compare(address, CpuRegister::AC);
    }

    fn cpx(&mut self, address: Address) {
        self.compare(address, CpuRegister::X);
    }

    fn cpy(&mut self, address: Address) {
        self.compare(address, CpuRegister::Y);
    }

    fn bcc(&mut self, address: Address) {
        self.branch(!self.sr.contains(status_flag::C), address);
    }

    fn bcs(&mut self, address: Address) {
        self.branch(self.sr.contains(status_flag::C), address);
    }

    fn beq(&mut self, address: Address) {
        self.branch(self.sr.contains(status_flag::Z), address);
    }

    fn bmi(&mut self, address: Address) {
        self.branch(self.sr.contains(status_flag::N), address);
    }

    fn bne(&mut self, address: Address) {
        self.branch(!self.sr.contains(status_flag::Z), address);
    }

    fn bpl(&mut self, address: Address) {
        self.branch(!self.sr.contains(status_flag::N), address);
    }

    fn bvc(&mut self, address: Address) {
        self.branch(!self.sr.contains(status_flag::V), address);
    }

    fn bvs(&mut self, address: Address) {
        self.branch(self.sr.contains(status_flag::V), address);
    }

    fn jmp(&mut self, address: Address) {
        self.pc = address.to_memory_unchecked();
    }

    fn jsr(&mut self, address: Address) {
        self.push_stack_u16(self.pc + 1);
        self.pc = address.to_memory_unchecked();
    }

    fn rts(&mut self) {
        self.pc = self.pull_stack_u16() + 1;
    }

    fn brk(&mut self) {
        self.sr.set(status_flag::B);
        self.sr.set(status_flag::I);
        self.push_stack_u16(self.pc + 2);
        self.push_stack_u8(self.sr.value());
        self.pc = Interrupt::Irq.vector();
    }

    fn rti(&mut self) {
        self.plp();
        self.pc = self.pull_stack_u16();
    }

    fn bit(&mut self, address: Address) {
        let rhs = self.read_address(address);
        let result = self.ac & rhs;
        self.sr.update_zero(result);
        self.sr.update_negative(rhs);
        self.sr.update(status_flag::V, (rhs >> 6 & 1) == 1);
    }

    fn nop(&self) {}

    fn alr(&mut self, address: Address) {
        self.and(address);
        self.lsr(address);
    }

    fn arr(&mut self, address: Address) {
        self.and(address);
        self.ror(address);
    }

    fn anc(&mut self, address: Address) {
        self.and(address);
        self.sr.update(status_flag::C, self.ac.contains(7));
    }

    fn las(&mut self, address: Address) {
        self.lda(address);
        self.tsx();
    }

    fn lax(&mut self, address: Address) {
        self.lda(address);
        self.ldx(address);
    }

    fn sax(&mut self, address: Address) {
        let result = self.ac & self.x;
        self.write_address(address, result);
    }

    fn sbx(&mut self, address: Address) {
        let lhs = self.ac & self.x;
        let rhs = self.read_address(address);
        let (sum, c1) = lhs.overflowing_add(!rhs);
        let (sum, c2) = sum.overflowing_add(1);
        self.sr.update(status_flag::C, c1 || c2);
        self.sr.update_negative(sum);
        self.sr.update_zero(sum);
        self.x = sum;
    }

    fn dcp(&mut self, address: Address) {
        self.dec(address);
        self.cmp(address);
    }

    fn isb(&mut self, address: Address) {
        self.inc(address);
        self.sbc(address);
    }

    fn slo(&mut self, address: Address) {
        self.asl(address);
        self.ora(address);
    }

    fn rla(&mut self, address: Address) {
        self.rol(address);
        self.and(address);
    }

    fn sre(&mut self, address: Address) {
        self.lsr(address);
        self.eor(address);
    }

    fn rra(&mut self, address: Address) {
        self.ror(address);
        self.adc(address);
    }

    fn load(&mut self, address: Address, register: CpuRegister) {
        let value = self.read_address(address);
        self.sr.update_negative(value);
        self.sr.update_zero(value);
        self.write_register(register, value);
    }

    fn store(&mut self, address: Address, register: CpuRegister) {
        let value = self.read_register(register);
        self.write_address(address, value);
    }

    fn transfer(&mut self, src: CpuRegister, dst: CpuRegister, update_flags: bool) {
        let value = self.read_register(src);

        if update_flags {
            self.sr.update_negative(value);
            self.sr.update_zero(value);
        }

        self.write_register(dst, value);
    }

    fn decrement(&mut self, address: Address) {
        let result = self.read_address(address).wrapping_sub(1);
        self.sr.update_negative(result);
        self.sr.update_zero(result);
        self.write_address(address, result);
    }

    fn increment(&mut self, address: Address) {
        let result = self.read_address(address).wrapping_add(1);
        self.sr.update_negative(result);
        self.sr.update_zero(result);
        self.write_address(address, result);
    }

    fn add_to_accumulator(&mut self, value: u8) {
        let carry = self.sr.get(status_flag::C);
        let (sum, c1) = self.ac.overflowing_add(value);
        let (sum, c2) = sum.overflowing_add(carry);
        let signed_sum = (self.ac as i8 as i16) + (value as i8 as i16) + carry as i16;
        let overflow = !(-128..=127).contains(&signed_sum);
        self.sr.update(status_flag::C, c1 || c2);
        self.sr.update(status_flag::V, overflow);
        self.sr.update_negative(sum);
        self.sr.update_zero(sum);
        self.ac = sum;
    }

    fn binary_op<F>(&mut self, address: Address, f: F)
    where
        F: Fn(u8, u8) -> u8,
    {
        let rhs = self.read_address(address);
        let result = f(self.ac, rhs);
        self.sr.update_negative(result);
        self.sr.update_zero(result);
        self.ac = result;
    }

    fn compare(&mut self, address: Address, register: CpuRegister) {
        let lhs = self.read_register(register);
        let rhs = self.read_address(address);
        let (sum, c1) = lhs.overflowing_add(!rhs);
        let (sum, c2) = sum.overflowing_add(1);
        self.sr.update(status_flag::C, c1 || c2);
        self.sr.update_negative(sum);
        self.sr.update_zero(sum);
    }

    fn branch(&mut self, predicate: bool, address: Address) {
        if predicate {
            self.pc = address.to_memory_unchecked();
        }
    }

    fn handle_interrupt(&mut self, interrupt: Interrupt) -> bool {
        if interrupt == Interrupt::Irq && self.sr.contains(status_flag::I) {
            return false;
        }

        let mut status = self.sr.value();
        status.clear(status_flag::B);
        status.set(status_flag::__);
        self.push_stack_u16(self.pc);
        self.push_stack_u8(status);
        self.sr.set(status_flag::I);
        self.pc = self.bus.read_u16(interrupt.vector());

        true
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cpu {{")?;
        writeln!(f, "  pc: 0x{:x},", self.pc)?;
        writeln!(f, "  ac: 0x{:x},", self.ac)?;
        writeln!(f, "  x: 0x{:x},", self.x)?;
        writeln!(f, "  y: 0x{:x},", self.y)?;
        writeln!(
            f,
            "  sr: {{ N: {}, V: {}, _: {}, B: {}, D: {}, I: {}, Z: {}, C: {} }},",
            self.sr.get(status_flag::N),
            self.sr.get(status_flag::V),
            self.sr.get(status_flag::__),
            self.sr.get(status_flag::B),
            self.sr.get(status_flag::D),
            self.sr.get(status_flag::I),
            self.sr.get(status_flag::Z),
            self.sr.get(status_flag::C)
        )?;
        writeln!(f, "  sp: 0x{:x},", self.sp)?;
        writeln!(f, "  cycle: {}", self.cycle)?;
        writeln!(f, "}}")
    }
}

impl Reset for Cpu {
    fn reset(&mut self) {
        self.pc = 0x00;
        self.ac = 0x00;
        self.x = 0x00;
        self.y = 0x00;
        self.sr = StatusRegister::default();
        self.sp = 0x00;
        self.bus.reset();
        self.cycle = 0;
        self.dma = None;
        self.interrupt = Some(Interrupt::Reset);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        bus::{Bus, MainBus},
        cpu::Cpu,
        mappers::MapperChip,
        utils::test::{LogLine, NESTEST_LOG, NESTEST_ROM},
    };

    #[test]
    fn test_cpu_nestest() {
        let mapper = MapperChip::try_from_bytes(NESTEST_ROM).unwrap();
        let bus = MainBus::new(mapper);
        let mut cpu = Cpu::new(bus);

        cpu.step(); // reset interrupt

        assert_eq!(cpu.pc, 0xC004);
        assert_eq!(cpu.sp, 0xFD);

        cpu.pc = 0xC000;
        cpu.sr.assign(0x24);

        for line in NESTEST_LOG.lines() {
            let parsed = LogLine::from_line(line).unwrap();
            let opcode = cpu.bus.read_u8(cpu.pc);

            println!("\nLine: {parsed:?}\nCPU: {cpu:?}");

            assert_eq!(parsed.opcode, opcode, "opcode");
            assert_eq!(parsed.pc, cpu.pc, "pc");
            assert_eq!(parsed.a, cpu.ac, "acc");
            assert_eq!(parsed.x, cpu.x, "x");
            assert_eq!(parsed.y, cpu.y, "y");
            assert_eq!(parsed.sp, cpu.sp, "sp");
            assert_eq!(parsed.sr, cpu.sr.value(), "sr");
            assert_eq!(parsed.cycle, cpu.cycle, "cycle");

            cpu.step();
        }
    }
}
