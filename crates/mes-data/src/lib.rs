use mes_core::{cartridge::Header, error::Error};
use types::SerializableHeader;

pub mod types;

pub fn serialize_rom_header(bytes: &[u8]) -> Result<String, Error> {
    let base_header = Header::try_from_bytes(bytes)?;
    let ser_header = SerializableHeader::from(base_header);
    let json_header = serde_json::to_string(&ser_header).unwrap();
    Ok(json_header)
}
