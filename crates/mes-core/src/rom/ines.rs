#[cfg(feature = "json")]
use serde::Serialize;

use crate::{error::Error, utils::BitFlag};

use super::Mirroring;

pub const INES_ASCII: [u8; 4] = [0x4E, 0x45, 0x53, 0x1A];
pub const INES_HEADER_SIZE: usize = 16;
pub const TRAINER_SIZE: usize = 512;

pub fn is_ines_file(bytes: &[u8]) -> bool {
    bytes[0..4] == INES_ASCII
}

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub struct Header {
    pub prg_rom_pages: u8,
    pub chr_rom_pages: u8,
    pub prg_ram_pages: u8,
    pub mirroring: Mirroring,
    pub battery: bool,
    pub trainer: bool,
    pub mapper: u8,
}

impl Header {
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if !is_ines_file(bytes) {
            return Err(Error::UnsupportedFileFormat);
        }

        let prg_rom_pages = *bytes.get(4).ok_or(Error::eof("PRG ROM pages", 1))?;
        let chr_rom_pages = *bytes.get(5).ok_or(Error::eof("CHR ROM pages", 1))?;
        let flags_6 = bytes.get(6).ok_or(Error::eof("Flags 6", 1))?;
        let flags_7 = bytes.get(7).ok_or(Error::eof("Flags 7", 1))?;
        let prg_ram_pages = *bytes.get(8).ok_or(Error::eof("PRG RAM pages", 1))?;

        let battery = flags_6.contains(1);
        let trainer = flags_6.contains(2);
        let mapper = (flags_7 & 0xF0) | (flags_6 >> 4);
        let is_vertical_mirroring = flags_6.contains(0);
        let is_four_screen = flags_6.contains(3);

        let mirroring = match (is_four_screen, is_vertical_mirroring) {
            (true, _) => Mirroring::FourScreen,
            (false, true) => Mirroring::Vertical,
            (false, false) => Mirroring::Horizontal,
        };

        Ok(Self {
            prg_rom_pages,
            prg_ram_pages,
            chr_rom_pages,
            mirroring,
            battery,
            trainer,
            mapper,
        })
    }
}
