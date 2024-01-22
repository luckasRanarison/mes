use std::fmt::Debug;

pub trait Bus
where
    Self: Debug,
{
    fn read_u8(&self, address: u16) -> u8;
    fn read_u16(&self, address: u16) -> u16;
    fn write_u8(&mut self, address: u16, value: u8);
}
