use mes_core::{mappers::MapperChip, ppu, Nes as NesCore};
use wasm_bindgen::{prelude::*, Clamped};
use web_sys::{js_sys::Float32Array, ImageData};

#[wasm_bindgen]
pub struct Nes {
    engine: NesCore,
    frame: Vec<u8>,
    palette: [u8; ppu::PALETTE_SIZE],
}

impl Default for Nes {
    fn default() -> Self {
        let mapper = MapperChip::mock();
        let engine = NesCore::with_mapper(mapper);
        let frame = vec![0; ppu::FRAME_BUFFER_SIZE * 4];
        let palette = *ppu::COLOR_PALETTE;

        Self {
            engine,
            frame,
            palette,
        }
    }
}

#[wasm_bindgen]
impl Nes {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self::default()
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.engine.reset();
    }

    #[wasm_bindgen(js_name = "setCartridge")]
    pub fn set_cartridge(&mut self, bytes: &[u8]) -> Result<(), JsError> {
        Ok(self.engine.set_cartridge(bytes)?)
    }

    #[wasm_bindgen(js_name = "stepFrame")]
    pub fn step_frame(&mut self) {
        self.engine.step_frame();
    }

    #[wasm_bindgen(js_name = "stepVblank")]
    pub fn step_vblank(&mut self) {
        self.engine.step_vblank();
    }

    #[wasm_bindgen(js_name = "getAudioBuffer")]
    pub fn get_audio_buffer(&self) -> Float32Array {
        let buffer = self.engine.get_audio_buffer();
        unsafe { Float32Array::view(&buffer) }
    }

    #[wasm_bindgen(js_name = "clearAudioBuffer")]
    pub fn clear_audio_buffer(&mut self) {
        self.engine.clear_audio_buffer();
    }

    #[wasm_bindgen(js_name = "updateImageData")]
    pub fn update_image_data(&mut self) -> Result<ImageData, JsValue> {
        let frame = self.update_frame();

        ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(frame),
            ppu::SCREEN_WIDTH as u32,
            ppu::SCREEN_HEIGHT as u32,
        )
    }

    #[wasm_bindgen(js_name = "setPalette")]
    pub fn set_palette(&mut self, palette: &[u8]) -> Result<(), JsError> {
        if let Ok(palette) = palette.try_into() {
            self.palette = palette;
            Ok(())
        } else {
            Err(JsError::new("Invalid NES color palette"))
        }
    }

    #[wasm_bindgen(js_name = "setControllerState")]
    pub fn set_controller_state(&mut self, id: usize, state: u8) {
        self.engine.set_controller_state(id, state);
    }

    fn update_frame(&mut self) -> &[u8] {
        let frame_buffer = self.engine.get_frame_buffer();

        for (i, pixel) in frame_buffer.iter().enumerate() {
            let color_index = *pixel as usize;
            self.frame[i * 4] = self.palette[color_index];
            self.frame[i * 4 + 1] = self.palette[color_index + 1];
            self.frame[i * 4 + 2] = self.palette[color_index + 2];
            self.frame[i * 4 + 3] = 255;
        }

        &self.frame
    }
}
