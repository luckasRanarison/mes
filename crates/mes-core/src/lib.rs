pub mod apu;
pub mod bus;
pub mod cartridge;
pub mod controller;
pub mod cpu;
pub mod error;
pub mod mappers;
pub mod ppu;
pub mod utils;

use std::cell::Ref;

use bus::MainBus;
use cpu::Cpu;
use error::Error;
use mappers::MapperChip;
use utils::Reset;

#[derive(Debug)]
pub struct Nes {
    cpu: Cpu,
}

impl Nes {
    pub fn new(bytes: &[u8]) -> Result<Self, Error> {
        let mapper = MapperChip::try_from_bytes(bytes)?;
        let bus = MainBus::new(mapper);
        let cpu = Cpu::new(bus);

        Ok(Self { cpu })
    }

    pub fn with_mapper(mapper: MapperChip) -> Self {
        let bus = MainBus::new(mapper);
        let cpu = Cpu::new(bus);

        Self { cpu }
    }

    pub fn set_cartridge(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let mapper = MapperChip::try_from_bytes(bytes)?;
        self.cpu.bus.set_mapper(mapper);

        Ok(())
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn step_frame(&mut self) {
        while !self.cpu.bus.ppu.is_vblank() {
            self.cpu.step();
        }
    }

    pub fn step_vblank(&mut self) {
        while self.cpu.bus.ppu.is_vblank() {
            self.cpu.step();
        }
    }

    pub fn get_audio_buffer(&self) -> Ref<[f32]> {
        Ref::map(self.cpu.apu.borrow(), |apu| apu.get_buffer())
    }

    pub fn clear_audio_buffer(&mut self) {
        self.cpu.apu.borrow_mut().clear_buffer();
    }

    pub fn get_frame_buffer(&self) -> &[u8] {
        self.cpu.bus.ppu.get_frame_buffer()
    }

    pub fn set_controller_state(&mut self, id: usize, state: u8) {
        self.cpu.bus.controller.set_state(id, state);
    }
}
