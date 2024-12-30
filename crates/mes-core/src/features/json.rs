use crate::{error::Error, rom::ines::Header};

pub fn serialize_rom_header(bytes: &[u8]) -> Result<String, Error> {
    let header = Header::try_from_bytes(bytes)?;
    let serialized = serde_json::to_string(&header).unwrap();
    Ok(serialized)
}
