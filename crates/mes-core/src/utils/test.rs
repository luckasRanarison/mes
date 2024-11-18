use std::{error::Error, fmt};

use super::BitFlag;

pub const NESTEST_ROM: &[u8] = include_bytes!("../../../../nes-test-roms/other/nestest.nes");
pub const NESTEST_LOG: &str = include_str!("../../../../nes-test-roms/other/nestest.log");

/// Represents parsed lines from nestest.log
pub struct LogLine {
    pub pc: u16,
    pub opcode: u8,
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub sr: u8,
    pub sp: u8,
    pub cycle: u64,
}

impl LogLine {
    pub fn from_line(line: &str) -> Result<Self, Box<dyn Error>> {
        Ok(LogLine {
            pc: u16::from_str_radix(&line[..4], 16)?,
            opcode: u8::from_str_radix(&line[6..8], 16)?,
            a: u8::from_str_radix(&line[50..52], 16)?,
            x: u8::from_str_radix(&line[55..57], 16)?,
            y: u8::from_str_radix(&line[60..62], 16)?,
            sr: u8::from_str_radix(&line[65..67], 16)?,
            sp: u8::from_str_radix(&line[71..73], 16)?,
            cycle: line[90..].parse()?,
        })
    }
}

impl fmt::Debug for LogLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "LogLine {{")?;
        writeln!(f, "  opcode: 0x{:x},", self.opcode)?;
        writeln!(f, "  pc: 0x{:x},", self.pc)?;
        writeln!(f, "  ac: 0x{:x},", self.a)?;
        writeln!(f, "  x: 0x{:x},", self.x)?;
        writeln!(f, "  y: 0x{:x},", self.y)?;
        writeln!(
            f,
            "  sr: {{ N: {}, V: {}, _: {}, B: {}, D: {}, I: {}, Z: {}, C: {} }},",
            self.sr.get(7),
            self.sr.get(6),
            self.sr.get(5),
            self.sr.get(6),
            self.sr.get(3),
            self.sr.get(2),
            self.sr.get(1),
            self.sr.get(0)
        )?;
        writeln!(f, "  sp: 0x{:x},", self.sp)?;
        writeln!(f, "  cycle: {}", self.cycle)?;
        writeln!(f, "}}")
    }
}
