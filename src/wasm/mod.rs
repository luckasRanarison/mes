use crate::{mappers::create_mapper_mock, Nes as NesEngine};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Nes {
    engine: NesEngine,
}

#[wasm_bindgen]
impl Nes {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mapper = create_mapper_mock();
        let engine = NesEngine::with_mapper(mapper);

        Self { engine }
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.engine.reset();
    }

    #[wasm_bindgen(js_name = "setCartridge")]
    pub fn set_cartridge(&mut self, bytes: &[u8]) -> Result<(), JsError> {
        self.engine.set_cartridge(bytes)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "stepFrame")]
    pub fn step_frame(&mut self) {
        self.engine.step_frame();
    }

    #[wasm_bindgen(js_name = "stepVblank")]
    pub fn step_vblank(&mut self) {
        self.engine.step_vblank();
    }

    #[wasm_bindgen(js_name = "getFrameBufferPtr")]
    pub fn get_frame_buffer_ptr(&self) -> *const u8 {
        self.engine.get_frame_buffer().as_ptr()
    }

    #[wasm_bindgen(js_name = "setPalette")]
    pub fn set_palette(&mut self, palette: &[u8]) {
        self.engine.set_palette(palette);
    }

    #[wasm_bindgen(js_name = "setControllerState")]
    pub fn set_controller_state(&mut self, id: usize, state: u8) {
        self.engine.set_controller_state(id, state);
    }
}
