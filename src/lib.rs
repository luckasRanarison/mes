mod bus;
mod cartridge;
mod controller;
mod cpu;
mod mappers;
mod ppu;
mod utils;

#[cfg(feature = "wasm")]
mod wasm;

pub mod error;

use bus::MainBus;
use cartridge::Cartridge;
use cpu::Cpu;
use error::Error;
use mappers::get_mapper;

#[derive(Debug)]
pub struct Nes {
    cpu: Cpu,
}

impl Nes {
    pub fn new(bytes: &[u8]) -> Result<Nes, Error> {
        let cartridge = Cartridge::try_from_bytes(bytes)?;
        let mapper_id = cartridge.header.mapper;
        let mapper = get_mapper(cartridge).ok_or(Error::UnsupportedMapper(mapper_id))?;
        let bus = MainBus::new(mapper);
        let cpu = Cpu::new(bus);

        Ok(Self { cpu })
    }

    pub fn step_frame(&mut self) {
        while !self.cpu.bus().is_vblank() {
            self.cpu.step();
        }
    }

    pub fn step_vblank(&mut self) {
        while self.cpu.bus().is_vblank() {
            self.cpu.step();
        }
    }

    pub fn get_frame_buffer(&self) -> &[u8] {
        self.cpu.bus().get_frame_buffer()
    }

    pub fn set_palette(&mut self, palette: &[u8]) {
        self.cpu.bus_mut().set_palette(palette);
    }

    pub fn set_controller_state(&mut self, id: usize, state: u8) {
        self.cpu.bus_mut().set_controller_state(id, state);
    }
}
