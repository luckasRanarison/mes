use mes_core::cartridge::{Header, Mirroring};
use serde::Serialize;

#[derive(Serialize)]
pub enum SerializableMirroring {
    Vertical,
    Horizontal,
    OneScreen,
    FourScreen,
}

impl From<Mirroring> for SerializableMirroring {
    fn from(value: Mirroring) -> Self {
        match value {
            Mirroring::Vertical => Self::Vertical,
            Mirroring::Horizontal => Self::Horizontal,
            Mirroring::OneScreen => Self::OneScreen,
            Mirroring::FourScreen => Self::FourScreen,
        }
    }
}

#[derive(Serialize)]
pub struct SerializableHeader {
    pub prg_rom_pages: u8,
    pub chr_rom_pages: u8,
    pub prg_ram_pages: u8,
    pub mirroring: SerializableMirroring,
    pub battery: bool,
    pub trainer: bool,
    pub mapper: u8,
}

impl From<Header> for SerializableHeader {
    fn from(value: Header) -> Self {
        Self {
            prg_rom_pages: value.prg_rom_pages,
            chr_rom_pages: value.chr_rom_pages,
            prg_ram_pages: value.prg_ram_pages,
            mirroring: value.mirroring.into(),
            battery: value.battery,
            trainer: value.trainer,
            mapper: value.mapper,
        }
    }
}
