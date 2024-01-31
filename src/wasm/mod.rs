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

    #[wasm_bindgen(js_name = "stepFrame")]
    pub fn step_frame(&mut self) {
        for _ in 0..29780 {
            self.cpu.step();
        }
    }

    #[wasm_bindgen(js_name = "getFrameBuffer")]
    pub fn get_frame_buffer(&mut self) -> *const u8 {
        self.cpu.bus_mut().get_frame_buffer().as_ptr()
    }
}
