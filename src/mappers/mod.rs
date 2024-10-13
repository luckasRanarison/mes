mod mapper_000;
mod mapper_001;
mod mapper_002;
mod mapper_003;
mod mapper_004;

use self::{
    mapper_000::NRom, mapper_001::SxRom, mapper_002::UxRom, mapper_003::CnRom, mapper_004::TxRom,
};

use crate::{
    cartridge::{create_cartridge_mock, Cartridge, Mirroring},
    error::Error,
    utils::Reset,
};

use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub trait Mapper: Debug + Reset {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
    fn get_mirroring(&self) -> Mirroring;
}

#[derive(Debug, Clone)]
pub struct MapperChip(Rc<RefCell<dyn Mapper>>);

impl MapperChip {
    fn new<M: Mapper + 'static>(mapper: M) -> Self {
        Self(Rc::new(RefCell::new(mapper)))
    }

    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Cartridge::try_from_bytes(bytes).and_then(MapperChip::try_from)
    }
}

impl TryFrom<Cartridge> for MapperChip {
    type Error = Error;

    fn try_from(value: Cartridge) -> Result<Self, Self::Error> {
        match value.header.mapper {
            0 => Ok(Self::new(NRom::new(value))),
            1 => Ok(Self::new(SxRom::new(value))),
            2 => Ok(Self::new(UxRom::new(value))),
            3 => Ok(Self::new(CnRom::new(value))),
            4 => Ok(Self::new(TxRom::new(value))),
            id => Err(Error::UnsupportedMapper(id)),
        }
    }
}

impl Mapper for MapperChip {
    fn read(&self, address: u16) -> u8 {
        self.0.borrow().read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.0.borrow_mut().write(address, value)
    }

    fn get_mirroring(&self) -> Mirroring {
        self.0.borrow().get_mirroring()
    }
}

impl Reset for MapperChip {
    fn reset(&mut self) {
        self.0.borrow_mut().reset()
    }
}

#[allow(unused)]
pub fn create_mapper_mock() -> MapperChip {
    let cartridge = create_cartridge_mock();
    let mapper = NRom::new(cartridge);
    MapperChip::new(mapper)
}
