mod mapper_000;

use crate::{
    cartridge::{create_cartridge_mock, Cartridge, Mirroring},
    error::Error,
    mappers::mapper_000::NRom,
    utils::Reset,
};
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub type MapperRef = Rc<RefCell<dyn Mapper>>;

fn create_ref<M>(mapper: M) -> MapperRef
where
    M: Mapper + 'static,
{
    Rc::new(RefCell::new(mapper))
}

pub trait Mapper: Debug + Reset {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
    fn get_mirroring(&self) -> Mirroring;
}

fn get_mapper(cartridge: Cartridge) -> Option<MapperRef> {
    match cartridge.header.mapper {
        0 => Some(create_ref(NRom::new(cartridge))),
        _ => None,
    }
}

pub fn get_mapper_from_bytes(bytes: &[u8]) -> Result<MapperRef, Error> {
    let cartridge = Cartridge::try_from_bytes(bytes)?;
    let mapper_id = cartridge.header.mapper;
    get_mapper(cartridge).ok_or(Error::UnsupportedMapper(mapper_id))
}

#[allow(unused)]
pub fn create_mapper_mock() -> MapperRef {
    let cartridge = create_cartridge_mock();
    get_mapper(cartridge).unwrap()
}

impl Mapper for MapperRef {
    fn read(&self, address: u16) -> u8 {
        self.borrow().read(address)
    }

    fn write(&mut self, address: u16, value: u8) {
        self.borrow_mut().write(address, value)
    }

    fn get_mirroring(&self) -> Mirroring {
        self.borrow().get_mirroring()
    }
}

impl Reset for MapperRef {
    fn reset(&mut self) {
        self.borrow_mut().reset();
    }
}
