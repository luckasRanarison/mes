const WIDTH: usize = 256;
const HEIGHT: usize = 240;

#[derive(Debug, Default)]
pub struct Frame {
    buffer: Vec<u8>,
}

impl Frame {
    pub fn into_buffer(self) -> Vec<u8> {
        self.buffer
    }
}
