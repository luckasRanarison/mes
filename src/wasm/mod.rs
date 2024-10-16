use crate::{mappers::create_mapper_mock, Nes as NesCore};

use wasm_bindgen::prelude::*;
use web_sys::js_sys::{Float32Array, Uint8ClampedArray};

#[wasm_bindgen]
pub struct Nes(NesCore);

#[wasm_bindgen]
impl Nes {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mapper = create_mapper_mock();
        let core = NesCore::with_mapper(mapper);

        Self(core)
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.0.reset();
    }

    #[wasm_bindgen(js_name = "setCartridge")]
    pub fn set_cartridge(&mut self, bytes: &[u8]) -> Result<(), JsError> {
        self.0.set_cartridge(bytes)?;
        Ok(())
    }

    #[wasm_bindgen(js_name = "stepFrame")]
    pub fn step_frame(&mut self) {
        self.0.step_frame();
    }

    #[wasm_bindgen(js_name = "stepVblank")]
    pub fn step_vblank(&mut self) {
        self.0.step_vblank();
    }

    #[wasm_bindgen(js_name = "getAudioBuffer")]
    pub fn get_audio_buffer(&self) -> Float32Array {
        let buffer = self.0.get_audio_buffer();
        unsafe { Float32Array::view(&buffer) }
    }

    #[wasm_bindgen(js_name = "clearAudioBuffer")]
    pub fn clear_audio_buffer(&mut self) {
        self.0.clear_audio_buffer();
    }

    #[wasm_bindgen(js_name = "getFrameBuffer")]
    pub fn get_frame_buffer(&self) -> Uint8ClampedArray {
        let buffer = self.0.get_frame_buffer();
        unsafe { Uint8ClampedArray::view(buffer) }
    }

    #[wasm_bindgen(js_name = "setPalette")]
    pub fn set_palette(&mut self, palette: &[u8]) {
        if let Ok(palette) = palette.try_into() {
            self.0.set_palette(palette);
        }
    }

    #[wasm_bindgen(js_name = "setControllerState")]
    pub fn set_controller_state(&mut self, id: usize, state: u8) {
        self.0.set_controller_state(id, state);
    }
}
