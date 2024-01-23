mod address;
mod opcodes;
mod register;

use address::{Address, AddressMode};
use opcodes::{Asm, OPCODE_MAP};

use crate::{
    bus::Bus,
    cpu::register::{CpuRegister, StatusRegister},
};

use std::{
    fmt,
    ops::{BitAnd, BitOr, BitXor},
};

use self::register::StatusFlag;

const STACK_START: u16 = 0x100;
const INITIAL_PC: u16 = 0xC000;

pub struct Cpu {
    pc: u16,
    ac: u8,
    x: u8,
    y: u8,
    sr: StatusRegister,
    sp: u8,
    bus: Box<dyn Bus>,
    cycle: usize,
}

impl Cpu {
    pub fn new<B>(bus: B) -> Self
    where
        B: Bus + 'static,
    {
        Self {
            pc: INITIAL_PC,
            ac: 0x00,
            x: 0x00,
            y: 0x00,
            sr: StatusRegister::default(),
            sp: 0xFF,
            bus: Box::new(bus),
            cycle: 0,
        }
    }

    pub fn step(&mut self, steps: usize) {
        for _ in 0..steps {
            let opcode = self.fetch();
            let cycles = self.execute(opcode);
            self.cycle = self.cycle.wrapping_add(cycles as usize);
        }
    }

    pub fn fetch(&mut self) -> u8 {
        let opcode = self.bus.read_u8(self.pc);
        self.increment_pc(1);
        opcode
    }

    pub fn execute(&mut self, opcode: u8) -> u8 {
        let Some(opcode) = OPCODE_MAP.get(&opcode) else {
            panic!("Invalid opcode: {:x}", opcode)
        };
        let adr_mode = opcode.adr_mode;
        let (address, crossed_boundary) = self.get_address(adr_mode);
        let total_cycle = opcode.cycle + opcode.get_additional_cycles(crossed_boundary);
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
        }

        if opcode.advance_counter() {
            self.increment_pc(opcode.len() - 1);
        }

        if opcode.is_branching() && (prev_pc != self.pc) {
            total_cycle + 1 + crossed_boundary as u8
        } else {
            total_cycle
        }
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

    fn get_address(&self, adr_mode: AddressMode) -> (Address, bool) {
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
                let address = self.bus.read_u16(self.bus.read_u16(self.pc));
                (Address::Memory(address), false)
            }
            AddressMode::IndirectX => {
                let lookup_adr = self.bus.read_u8(self.pc).wrapping_add(self.x);
                let address = self.bus.read_u16(lookup_adr as u16);
                (Address::Memory(address), false)
            }
            AddressMode::IndirectY => {
                let address = self.bus.read_u16(self.bus.read_u8(self.pc) as u16);
                let (address, crossed) = self.get_effective_address(address, self.y);
                (Address::Memory(address), crossed)
            }
            AddressMode::Relative => {
                let offset = self.bus.read_u8(self.pc);
                let (address, crossed) = self.get_effective_address(self.pc, offset);
                (Address::Memory(address), crossed)
            }
        }
    }

    fn read_address(&self, address: Address) -> u8 {
        match address {
            Address::Memory(adr) => self.bus.read_u8(adr),
            Address::Register(reg) => self.get_register(reg),
        }
    }

    fn write_address(&mut self, address: Address, value: u8) {
        match address {
            Address::Memory(adr) => self.bus.write_u8(adr, value),
            Address::Register(reg) => *self.get_register_mut(reg) = value,
        }
    }

    fn get_register(&self, register: CpuRegister) -> u8 {
        match register {
            CpuRegister::AC => self.ac,
            CpuRegister::X => self.x,
            CpuRegister::Y => self.y,
            CpuRegister::SP => self.sp,
            _ => unreachable!(),
        }
    }

    fn get_register_mut(&mut self, register: CpuRegister) -> &mut u8 {
        match register {
            CpuRegister::AC => &mut self.ac,
            CpuRegister::X => &mut self.x,
            CpuRegister::Y => &mut self.y,
            CpuRegister::SP => &mut self.sp,
            _ => unreachable!(),
        }
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
        self.transfert(CpuRegister::AC, CpuRegister::X);
    }

    fn tay(&mut self) {
        self.transfert(CpuRegister::AC, CpuRegister::Y);
    }

    fn tsx(&mut self) {
        self.transfert(CpuRegister::SP, CpuRegister::X);
    }

    fn txa(&mut self) {
        self.transfert(CpuRegister::X, CpuRegister::AC);
    }

    fn txs(&mut self) {
        self.transfert(CpuRegister::X, CpuRegister::SP);
    }

    fn tya(&mut self) {
        self.transfert(CpuRegister::Y, CpuRegister::AC);
    }

    fn pha(&mut self) {
        self.push_stack_u8(self.ac);
    }

    fn php(&mut self) {
        self.sr.set(StatusFlag::B);
        self.push_stack_u8(self.sr.value());
    }

    fn pla(&mut self) {
        let value = self.pull_stack_u8();
        self.sr.update_negative(value);
        self.sr.update_zero(value);
        self.ac = value;
    }

    fn plp(&mut self) {
        let status = (self.pull_stack_u8() & 0b1110_1111) | ((self.sr.get(StatusFlag::B)) << 4);
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
        self.sr.update(StatusFlag::C, operand >> 7 == 1);
        self.sr.update_negative(result);
        self.sr.update_zero(result);
        self.write_address(address, result);
    }

    fn lsr(&mut self, address: Address) {
        let operand = self.read_address(address);
        let result = operand.wrapping_shr(1);
        self.sr.update(StatusFlag::C, operand << 7 == 128);
        self.sr.update_negative(result);
        self.sr.update_zero(result);
        self.write_address(address, result);
    }

    fn rol(&mut self, address: Address) {
        let operand = self.read_address(address);
        let carry_bit = self.sr.get(StatusFlag::C);
        let result = operand.wrapping_shl(1) | carry_bit;
        self.sr.update(StatusFlag::C, operand >> 7 == 1);
        self.sr.update_negative(result);
        self.sr.update_zero(result);
        self.write_address(address, result);
    }

    fn ror(&mut self, address: Address) {
        let operand = self.read_address(address);
        let carry_bit = self.sr.get(StatusFlag::C);
        let result = operand.wrapping_shr(1) | (carry_bit << 7);
        self.sr.update(StatusFlag::C, operand & 1 == 1);
        self.sr.update_negative(result);
        self.sr.update_zero(result);
        self.write_address(address, result);
    }

    fn clc(&mut self) {
        self.sr.clear(StatusFlag::C);
    }

    fn cld(&mut self) {
        self.sr.clear(StatusFlag::D);
    }

    fn cli(&mut self) {
        self.sr.clear(StatusFlag::I);
    }

    fn clv(&mut self) {
        self.sr.clear(StatusFlag::V);
    }

    fn sec(&mut self) {
        self.sr.set(StatusFlag::C);
    }

    fn sei(&mut self) {
        self.sr.set(StatusFlag::I);
    }

    fn sed(&mut self) {
        self.sr.set(StatusFlag::D);
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
        self.branch(!self.sr.contains(StatusFlag::C), address);
    }

    fn bcs(&mut self, address: Address) {
        self.branch(self.sr.contains(StatusFlag::C), address);
    }

    fn beq(&mut self, address: Address) {
        self.branch(self.sr.contains(StatusFlag::Z), address);
    }

    fn bmi(&mut self, address: Address) {
        self.branch(self.sr.contains(StatusFlag::N), address);
    }

    fn bne(&mut self, address: Address) {
        self.branch(!self.sr.contains(StatusFlag::Z), address);
    }

    fn bpl(&mut self, address: Address) {
        self.branch(!self.sr.contains(StatusFlag::N), address);
    }

    fn bvc(&mut self, address: Address) {
        self.branch(!self.sr.contains(StatusFlag::V), address);
    }

    fn bvs(&mut self, address: Address) {
        self.branch(self.sr.contains(StatusFlag::V), address);
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
        self.sr.set(StatusFlag::B);
        self.sr.set(StatusFlag::I);
        self.push_stack_u16(self.pc + 2);
        self.push_stack_u8(self.sr.value());
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
        self.sr.update(StatusFlag::V, (rhs >> 6 & 1) == 1);
    }

    fn nop(&self) {}

    fn load(&mut self, address: Address, register: CpuRegister) {
        let value = self.read_address(address);
        self.sr.update_negative(value);
        self.sr.update_zero(value);
        *self.get_register_mut(register) = value;
    }

    fn store(&mut self, address: Address, register: CpuRegister) {
        let value = self.get_register(register);
        self.write_address(address, value);
    }

    fn transfert(&mut self, src: CpuRegister, dst: CpuRegister) {
        let value = self.get_register(src);
        self.sr.update_negative(value);
        self.sr.update_zero(value);
        *self.get_register_mut(dst) = value;
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
        let carry = self.sr.get(StatusFlag::C);
        let (sum, c1) = self.ac.overflowing_add(value);
        let (sum, c2) = sum.overflowing_add(carry);
        let signed_sum = (self.ac as i8 as i16) + (value as i8 as i16) + carry as i16;
        let overflow = !(-128..=127).contains(&signed_sum);
        self.sr.update(StatusFlag::C, c1 || c2);
        self.sr.update(StatusFlag::V, overflow);
        self.sr.update_negative(sum);
        self.sr.update_zero(sum);
        self.ac = sum;
    }

    fn binary_op(&mut self, address: Address, f: fn(u8, u8) -> u8) {
        let rhs = self.read_address(address);
        let result = f(self.ac, rhs);
        self.sr.update_negative(result);
        self.sr.update_zero(result);
        self.ac = result;
    }

    fn compare(&mut self, address: Address, register: CpuRegister) {
        let lhs = self.get_register(register);
        let rhs = self.read_address(address);
        let (sum, c1) = lhs.overflowing_add(!rhs);
        let (sum, c2) = sum.overflowing_add(1);
        self.sr.update(StatusFlag::C, c1 || c2);
        self.sr.update_negative(sum);
        self.sr.update_zero(sum);
    }

    fn branch(&mut self, predicate: bool, address: Address) {
        if predicate {
            self.pc = address.to_memory_unchecked();
        }
    }
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Cpu {{")?;
        writeln!(f, "  pc: 0x{:x}", self.pc)?;
        writeln!(f, "  ac: {}", self.ac)?;
        writeln!(f, "  x: {}", self.x)?;
        writeln!(f, "  y: {}", self.y)?;
        writeln!(
            f,
            "  sr: {{ N: {}, V: {}, _: {}, B: {}, D: {}, I: {}, Z: {}, C: {} }}",
            self.sr.get(StatusFlag::N),
            self.sr.get(StatusFlag::V),
            self.sr.get(StatusFlag::__),
            self.sr.get(StatusFlag::B),
            self.sr.get(StatusFlag::D),
            self.sr.get(StatusFlag::I),
            self.sr.get(StatusFlag::Z),
            self.sr.get(StatusFlag::C)
        )?;
        writeln!(f, "  sp: 0x{:x}", self.sp)?;
        writeln!(f, "}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INITIAL_PC: usize = 0xC000;

    #[derive(Debug)]
    struct BusMock {
        ram: [u8; 0xFFFF],
    }

    impl Default for BusMock {
        fn default() -> Self {
            Self { ram: [0; 0xFFFF] }
        }
    }

    impl Bus for BusMock {
        fn read_u8(&self, address: u16) -> u8 {
            self.ram[address as usize]
        }

        fn read_u16(&self, address: u16) -> u16 {
            let low = self.ram[address as usize];
            let high = self.ram[address as usize + 1];
            u16::from_le_bytes([low, high])
        }

        fn write_u8(&mut self, address: u16, value: u8) {
            self.ram[address as usize] = value;
        }
    }

    fn cpu_with_program(program: &[u8]) -> Cpu {
        let mut ram = [0; 0xFFFF];
        let start = INITIAL_PC;
        let end = INITIAL_PC + program.len();
        ram[start..end].copy_from_slice(program);

        Cpu::new(BusMock { ram })
    }

    #[test]
    fn test_stack() {
        let mut cpu = Cpu::new(BusMock::default());

        cpu.push_stack_u8(0x30);

        assert_eq!(cpu.sp, 0xFE);
        assert_eq!(cpu.bus.read_u8(0x1FF), 0x30);
        assert_eq!(cpu.pull_stack_u8(), 0x30);
        assert_eq!(cpu.sp, 0xFF);

        cpu.push_stack_u16(0x4530);

        assert_eq!(cpu.sp, 0xFD);
        assert_eq!(cpu.pull_stack_u16(), 0x4530);
        assert_eq!(cpu.sp, 0xFF);
    }

    #[test]
    fn test_immediate_addressing() {
        let mut cpu = cpu_with_program(&[0xA9, 0x30]);

        cpu.step(1);

        assert_eq!(cpu.ac, 0x30);
    }

    #[test]
    fn test_absolute_addressing() {
        let mut cpu = cpu_with_program(&[0xA9, 0x5F, 0x8D, 0x45, 0x32]);

        cpu.step(2);

        assert_eq!(cpu.bus.read_u8(0x3245), 0x5F);
    }

    #[test]
    fn test_zero_page_addressing() {
        let mut cpu = cpu_with_program(&[0xA9, 0x15, 0x85, 0x10]);

        cpu.step(2);

        assert_eq!(cpu.bus.read_u8(0x0010), 0x15);
    }

    #[test]
    fn test_indexed_absolute_addressing() {
        let mut cpu = cpu_with_program(&[0xA2, 0x50, 0x8E, 0x70, 0x20, 0xBD, 0x20, 0x20]);

        cpu.step(3);

        assert_eq!(cpu.ac, 0x50);
        assert_eq!(cpu.cycle, 10);

        let mut cpu = cpu_with_program(&[0xA2, 0x50, 0x8E, 0x10, 0x20, 0xBD, 0xC0, 0x1F]);

        cpu.step(3);

        assert_eq!(cpu.ac, 0x50);
        assert_eq!(cpu.cycle, 11);
    }

    #[test]
    fn test_indexed_zero_page_addressing() {
        let mut cpu = cpu_with_program(&[0xA2, 0x20, 0x86, 0x40, 0xB5, 0x20]);

        cpu.step(3);

        assert_eq!(cpu.ac, 0x20);

        let mut cpu = cpu_with_program(&[0xA9, 0x10, 0x85, 0x20, 0xA2, 0xFF, 0xB4, 0x21]);

        cpu.step(4);

        assert_eq!(cpu.y, 0x10);
    }

    #[test]
    fn test_indirect_addressing() {
        let mut cpu = cpu_with_program(&[
            0xA9, 0xC4, 0x8D, 0x82, 0xFF, 0xA9, 0x80, 0x8D, 0x83, 0xFF, 0x6C, 0x82, 0xFF,
        ]);

        cpu.step(5);

        assert_eq!(cpu.pc, 0x80C4);
    }

    #[test]
    fn test_pre_index_indirect_addressing() {
        let mut cpu = cpu_with_program(&[
            0xA9, 0xA5, 0x8D, 0x23, 0x30, 0xA9, 0x23, 0x85, 0x75, 0xA9, 0x30, 0x85, 0x76, 0xA2,
            0x05, 0xA1, 0x70,
        ]);

        cpu.step(8);

        assert_eq!(cpu.ac, 0xA5);
    }

    #[test]
    fn test_post_index_indirect_addressing() {
        let mut cpu = cpu_with_program(&[
            0xA9, 0x23, 0x8D, 0x53, 0x35, 0xA9, 0x43, 0x85, 0x70, 0xA9, 0x35, 0x85, 0x71, 0xA0,
            0x10, 0xB1, 0x70,
        ]);

        cpu.step(8);

        assert_eq!(cpu.ac, 0x23);
        assert_eq!(cpu.cycle, 23);
    }

    #[test]
    fn test_relative_addressing() {
        let mut cpu = cpu_with_program(&[0xD0, 0x02, 0xA9, 0x05, 0xA9, 0x10]);

        cpu.step(2);

        assert_eq!(cpu.ac, 0x10);
    }

    #[test]
    fn test_add_carry() {
        let mut cpu = cpu_with_program(&[0xA9, 0x70, 0x85, 0x10, 0x65, 0x10]);

        cpu.step(3);

        assert_eq!(cpu.ac, 0xE0);
        assert!(cpu.sr.contains(StatusFlag::N));
        assert!(cpu.sr.contains(StatusFlag::V));
        assert!(!cpu.sr.contains(StatusFlag::C));

        let mut cpu = cpu_with_program(&[0xA9, 0xFF, 0x85, 0x10, 0x65, 0x10]);

        cpu.step(3);

        assert_eq!(cpu.ac, 0xFE);
        assert!(cpu.sr.contains(StatusFlag::N));
        assert!(!cpu.sr.contains(StatusFlag::V));
        assert!(cpu.sr.contains(StatusFlag::C));
    }

    #[test]
    fn test_sub_carry() {
        let mut cpu = cpu_with_program(&[0xA9, 0x30, 0x85, 0x10, 0xE5, 0x10]);

        cpu.step(3);

        assert_eq!(cpu.ac, 0xFF);
        assert!(cpu.sr.contains(StatusFlag::N));
        assert!(!cpu.sr.contains(StatusFlag::V));

        let mut cpu = cpu_with_program(&[0x38, 0xA9, 0x30, 0x85, 0x10, 0xE5, 0x10]);

        cpu.step(4);

        assert_eq!(cpu.ac, 0x00);
        assert!(!cpu.sr.contains(StatusFlag::V));
        assert!(cpu.sr.contains(StatusFlag::Z));
        assert!(cpu.sr.contains(StatusFlag::C));
    }

    #[test]
    fn test_comparaison() {
        let mut cpu = cpu_with_program(&[0xA9, 0x30, 0x85, 0x10, 0xC5, 0x10]);

        cpu.step(3);

        assert!(cpu.sr.contains(StatusFlag::Z));
        assert!(cpu.sr.contains(StatusFlag::C));
    }

    #[test]
    fn test_subroutines() {
        let mut cpu = cpu_with_program(&[0x20, 0x05, 0xC0, 0x85, 0x10, 0xA9, 0x10, 0x60]);

        cpu.step(4);

        assert_eq!(cpu.ac, 0x10);
        assert_eq!(cpu.bus.read_u8(0x0010), 0x10);
    }
}
