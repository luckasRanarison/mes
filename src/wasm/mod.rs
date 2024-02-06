use crate::Nes as NesEngine;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Nes {
    engine: NesEngine,
}

#[wasm_bindgen]
impl Nes {
    #[wasm_bindgen(constructor)]
    pub fn new(bytes: &[u8]) -> Result<Nes, JsError> {
        Ok(Self {
            engine: NesEngine::new(bytes)?,
        })
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

    #[wasm_bindgen(js_name = "setControllerState")]
    pub fn set_controller_state(&mut self, id: usize, state: u8) {
        self.engine.set_controller_state(id, state);
    }
}
