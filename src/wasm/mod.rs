use crate::{bus::MainBus, cartridge::Cartridge, cpu::Cpu, error::Error, mappers::get_mapper};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Nes {
    cpu: Cpu,
}

#[wasm_bindgen]
impl Nes {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Result<Nes, JsError> {
        let cartridge = Cartridge::try_from_bytes(bytes)?;
        let mapper_id = cartridge.header.mapper;
        let mapper = get_mapper(cartridge).ok_or(Error::UnsupportedMapper(mapper_id))?;
        let bus = MainBus::new(mapper);
        let cpu = Cpu::new(bus);

        Ok(Self { cpu })
    }

    #[wasm_bindgen(js_name = "stepFrame")]
    pub fn step_frame(&mut self) {
        while !self.cpu.bus().is_vblank() {
            self.cpu.step();
        }
        while self.cpu.bus().is_vblank() {
            self.cpu.step();
        }
    }

    #[wasm_bindgen(js_name = "getFrameBufferPtr")]
    pub fn get_frame_buffer_ptr(&self) -> *const u8 {
        self.cpu.bus().get_frame_buffer().as_ptr()
    }

    #[wasm_bindgen(js_name = "setControllerButton")]
    pub fn set_controller_button(&mut self, id: usize, button: u8) {
        self.cpu.bus_mut().set_controller_button(id, button);
    }
}

#[wasm_bindgen]
pub enum ControllerButton {
    Right,
    Left,
    Down,
    Up,
    Start,
    Select,
    B,
    A,
}
