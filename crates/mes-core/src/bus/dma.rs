#[cfg(feature = "json")]
use serde::Serialize;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct DmaState {
    pub high_byte: u8,
    pub current_page: u8,
    pub buffer: Option<u8>,
}

impl DmaState {
    pub fn new(offset: u8) -> Self {
        Self {
            high_byte: offset,
            current_page: 0x00,
            buffer: None,
        }
    }

    pub fn get_ram_address(&self) -> u16 {
        u16::from_le_bytes([self.current_page, self.high_byte])
    }
}
