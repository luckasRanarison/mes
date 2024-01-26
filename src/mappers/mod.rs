mod mapper_0;

use crate::{
    cartridge::{Cartridge, Mirroring},
    mappers::mapper_0::NRom,
};
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub type MapperRef = Rc<RefCell<dyn Mapper>>;

fn create_ref<M>(mapper: M) -> MapperRef
where
    M: Mapper + 'static,
{
    Rc::new(RefCell::new(mapper))
}

pub trait Mapper: Debug {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
    fn get_mirroring(&self) -> Mirroring;
}

pub fn get_mapper(cartridge: Cartridge) -> Option<MapperRef> {
    match cartridge.header.mapper {
        0 => Some(create_ref(NRom::new(cartridge))),
        _ => None,
    }
}

impl Mapper for Rc<RefCell<dyn Mapper>> {
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
