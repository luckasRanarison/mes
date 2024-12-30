use crate::{error::Error, utils::MemoryObserver};

use super::{
    ines::{Header, INES_HEADER_SIZE, TRAINER_SIZE},
    Mirroring,
};

pub const PRG_ROM_PAGE_SIZE: usize = 16384;
pub const PRG_RAM_SIZE: usize = 8192;
pub const CHR_ROM_PAGE_SIZE: usize = 8192;
pub const CHR_RAM_PAGE_SIZE: usize = 8192;

pub enum ChrPage {
    Index4(u8),
    Index8(u8),
}

pub enum PrgPage {
    Index16(u8),
    Index32(u8),
    Last16,
}

pub struct Cartridge {
    pub header: Header,
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub prg_ram: Vec<u8>,
    pub chr_ram: Vec<u8>,
    pub observer: Option<Box<dyn MemoryObserver>>,
}

impl std::fmt::Debug for Cartridge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.header)
    }
}

impl Cartridge {
    pub fn try_from_ines(bytes: &[u8]) -> Result<Self, Error> {
        let header = Header::try_from_bytes(bytes)?;

        let prg_rom_size = header.prg_rom_pages as usize * PRG_ROM_PAGE_SIZE;
        let chr_rom_size = header.chr_rom_pages as usize * CHR_ROM_PAGE_SIZE;
        let prg_rom_start = INES_HEADER_SIZE + if header.trainer { TRAINER_SIZE } else { 0 };
        let chr_rom_start = prg_rom_start + prg_rom_size;
        let prg_rom = bytes[prg_rom_start..prg_rom_start + prg_rom_size].to_vec();
        let chr_rom = bytes[chr_rom_start..chr_rom_start + chr_rom_size].to_vec();
        let prg_ram = vec![0_u8; PRG_RAM_SIZE];
        let chr_ram_size = (header.chr_rom_pages == 0) as usize * CHR_RAM_PAGE_SIZE;
        let chr_ram = vec![0_u8; chr_ram_size];

        Ok(Self {
            header,
            prg_rom,
            chr_rom,
            prg_ram,
            chr_ram,
            observer: None,
        })
    }

    pub fn write_prg_ram(&mut self, address: u16, value: u8) {
        self.prg_ram[address as usize & 0x1FFF] = value;

        if let Some(observer) = &mut self.observer {
            observer.observe(&self.prg_ram);
        }
    }

    pub fn write_chr_ram(&mut self, address: u16, value: u8, page: ChrPage) {
        if self.header.chr_rom_pages == 0 {
            let (page_start, mask) = match page {
                ChrPage::Index4(index) => (index as usize * (CHR_ROM_PAGE_SIZE / 2), 0x0FFF),
                ChrPage::Index8(index) => (index as usize * CHR_ROM_PAGE_SIZE, 0x1FFF),
            };

            self.chr_ram[page_start + (address as usize & mask)] = value;
        }
    }

    pub fn read_prg_rom(&self, address: u16, page: PrgPage) -> u8 {
        let (page_start, mask) = match page {
            PrgPage::Index16(index) => (index as usize * PRG_ROM_PAGE_SIZE, 0x3FFF),
            PrgPage::Index32(index) => (index as usize * PRG_ROM_PAGE_SIZE * 2, 0x7FFF),
            PrgPage::Last16 => (
                (self.header.prg_rom_pages as usize - 1) * PRG_ROM_PAGE_SIZE,
                0x3FFF,
            ),
        };

        self.prg_rom[page_start + (address as usize & mask)]
    }

    pub fn read_prg_ram(&self, address: u16) -> u8 {
        self.prg_ram[address as usize & 0x1FFF]
    }

    pub fn read_chr(&self, address: u16, page: ChrPage) -> u8 {
        let chr = match self.header.chr_rom_pages {
            0 => &self.chr_ram,
            _ => &self.chr_rom,
        };
        let (page_start, mask) = match page {
            ChrPage::Index4(index) => (index as usize * (CHR_ROM_PAGE_SIZE / 2), 0x0FFF),
            ChrPage::Index8(index) => (index as usize * CHR_ROM_PAGE_SIZE, 0x1FFF),
        };

        chr[page_start + (address as usize & mask)]
    }
}

impl Default for Cartridge {
    fn default() -> Self {
        Self {
            header: Header {
                prg_rom_pages: 1,
                chr_rom_pages: 1,
                prg_ram_pages: 0,
                mirroring: Mirroring::Vertical,
                battery: false,
                trainer: false,
                mapper: 0,
            },
            prg_rom: vec![0; PRG_ROM_PAGE_SIZE],
            chr_rom: vec![0; CHR_ROM_PAGE_SIZE],
            prg_ram: vec![],
            chr_ram: vec![],
            observer: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::test::NESTEST_ROM;

    use super::Cartridge;

    #[test]
    fn test_load_rom() {
        let rom = Cartridge::try_from_ines(&NESTEST_ROM);

        assert!(rom.is_ok());
    }
}