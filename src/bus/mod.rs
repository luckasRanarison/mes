mod nes;
mod ppu;

pub use {nes::NesBus, ppu::PpuBus};

use std::fmt::Debug;

pub trait Bus: Debug {
    fn read_u8(&mut self, address: u16) -> u8;

    fn read_u16(&mut self, address: u16) -> u16 {
        let low = self.read_u8(address);
        let high = self.read_u8(address.wrapping_add(1));
        u16::from_le_bytes([low, high])
    }

    fn write_u8(&mut self, address: u16, value: u8);
}
