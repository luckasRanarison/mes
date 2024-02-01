const WIDTH: usize = 256;
const HEIGHT: usize = 240;

#[derive(Debug)]
pub struct Frame {
    buffer: Vec<u8>,
}

impl Default for Frame {
    fn default() -> Self {
        Self {
            buffer: vec![0; WIDTH * HEIGHT * 4],
        }
    }
}

impl Frame {
    pub fn set_pixel(&mut self, x: usize, y: usize, rgb: (u8, u8, u8)) {
        let index = (y * 256 + x) * 4;

        self.buffer[index + 0] = rgb.0;
        self.buffer[index + 1] = rgb.1;
        self.buffer[index + 2] = rgb.2;
        self.buffer[index + 3] = 255;
    }

    pub fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }
}
