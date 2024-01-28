mod dma;
mod ppu;

use crate::{
    cartridge::{error::LoadError, Cartridge},
    cpu::interrupt::Interrupt,
    mappers::{get_mapper, Mapper, MapperRef},
    ppu::Ppu,
    utils::Clock,
};

use std::fmt::Debug;

pub use {dma::DmaState, ppu::PpuBus};

pub trait Bus: Debug + Clock {
    fn read_u8(&mut self, address: u16) -> u8;

    fn read_u16(&mut self, address: u16) -> u16 {
        let low = self.read_u8(address);
        let high = self.read_u8(address.wrapping_add(1));
        u16::from_le_bytes([low, high])
    }

    fn write_u8(&mut self, address: u16, value: u8);
}

#[derive(Debug)]
pub struct MainBus {
    ram: [u8; 2048],
    ppu: Ppu,
    mapper: MapperRef,
    dma_adr: Option<u8>,
    cycle: u64,
}

impl MainBus {
    pub fn new(cartridge: Cartridge) -> Result<Self, LoadError> {
        let mapper = get_mapper(cartridge).ok_or(LoadError::UnsupportedMapper)?;
        let ppu = Ppu::new(mapper.clone());
        let ram = [0; 2048];

        Ok(MainBus {
            ram,
            ppu,
            mapper,
            dma_adr: None,
            cycle: 0,
        })
    }

    pub fn poll_interrupt(&mut self) -> Option<Interrupt> {
        self.ppu.poll_nmi().then_some(Interrupt::Nmi)
    }

    pub fn poll_dma(&mut self) -> Option<u8> {
        self.dma_adr.take()
    }

    pub fn dma_cycle(&mut self, state: &mut DmaState) -> bool {
        if let Some(buffer) = state.buffer {
            let address = state.current_page;
            self.ppu.write_oam(address, buffer); // put
            state.current_page = address.wrapping_add(1);
            state.buffer.take();
        } else {
            let address = state.get_ram_address();
            let value = self.read_u8(address); // get
            state.buffer = Some(value);
        }

        state.current_page == 0x00
    }

    fn setup_oam_dma(&mut self, offset: u8) {
        self.dma_adr = Some(offset);
    }

    fn read_ram(&self, address: u16) -> u8 {
        self.ram[address as usize % 0x8000]
    }

    fn write_ram(&mut self, address: u16, value: u8) {
        self.ram[address as usize % 0x8000] = value;
    }
}

impl Bus for MainBus {
    fn read_u8(&mut self, address: u16) -> u8 {
        match address {
            0x0000..=0x1FFF => self.read_ram(address),
            0x2002 => self.ppu.read_status(),
            0x2004 => self.ppu.read_oam_data(),
            0x2007 => self.ppu.read_data(),
            0x2008..=0x3FFF => self.read_u8(address & 0x2007),
            0x4016 => 0, // TODO: APU
            0x4020..=0xFFFF => self.mapper.read(address),
            _ => panic!("Trying to read from write-only address: 0x{:x}", address),
        }
    }

    fn write_u8(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.write_ram(address, value),
            0x2000 if self.cycle >= 29_658 => self.ppu.write_ctrl(value),
            0x2001 if self.cycle >= 29_658 => self.ppu.write_mask(value),
            0x2003 => self.ppu.write_oam_address(value),
            0x2004 => self.ppu.write_oam_data(value),
            0x2005 if self.cycle >= 29_658 => self.ppu.write_scroll(value),
            0x2006 if self.cycle >= 29_658 => self.ppu.write_addr(value),
            0x2007 => self.ppu.write_data(value),
            0x2008..=0x3FFF => self.write_u8(address & 0x2007, value),
            0x4000..=0x4013 | 0x4015 => {}
            0x4014 => self.setup_oam_dma(value),
            0x4016 | 0x4017 => {}
            0x4020..=0xFFFF => self.mapper.write(address, value),
            _ => panic!("Trying to write to read-only address: 0x{:x}", address),
        }
    }
}

impl Clock for MainBus {
    fn tick(&mut self, cycles: u8) {
        self.cycle += cycles as u64;
        self.ppu.tick(cycles * 3);
    }
}
