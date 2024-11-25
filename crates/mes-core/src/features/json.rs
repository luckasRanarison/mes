use serde::Serialize;

use crate::{cartridge::Header, error::Error, Nes};

trait ToJsonString {
    fn to_json_string(&self) -> String;
}

impl<T: Serialize> ToJsonString for T {
    fn to_json_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

pub fn serialize_rom_header(bytes: &[u8]) -> Result<String, Error> {
    Header::try_from_bytes(bytes).map(|header| header.to_json_string())
}

impl Nes {
    pub fn serialize_cpu(&self) -> String {
        self.cpu.to_json_string()
    }
}
