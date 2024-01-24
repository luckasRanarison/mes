pub mod mapper_0;

use crate::{cartridge::Cartridge, mappers::mapper_0::Nrom};
use std::{cell::RefCell, fmt::Debug, rc::Rc};

pub type MapperRef = Rc<RefCell<dyn Mapper>>;

fn new_mapper_ref<M>(mapper: M) -> MapperRef
where
    M: Mapper + 'static,
{
    Rc::new(RefCell::new(mapper))
}

pub trait Mapper: Debug {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
}

pub fn get_mapper(cartridge: Cartridge) -> Option<MapperRef> {
    match cartridge.header.mapper {
        0 => Some(new_mapper_ref(Nrom::new(cartridge))),
        _ => None,
    }
}
