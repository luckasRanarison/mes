use crate::{bus::MainBus, cartridge::Cartridge, cpu::Cpu, mappers::get_mapper};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Nes {
    cpu: Cpu,
}

#[wasm_bindgen]
impl Nes {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Self {
        let cartridge = Cartridge::try_from_bytes(bytes).unwrap();
        let mapper = get_mapper(cartridge).unwrap();
        let bus = MainBus::new(mapper);
        let cpu = Cpu::new(bus);

        Self { cpu }
    }

    pub fn step(&mut self) {
        self.cpu.step();
    }

    pub fn is_vblank(&self) -> bool {
        self.cpu.bus().is_vblank()
    }

    pub fn get_frame_buffer(&self) -> Vec<u8> {
        self.cpu.bus().get_frame_buffer()
    }
}
