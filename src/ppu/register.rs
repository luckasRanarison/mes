#[derive(Debug, Default)]
pub struct ControlRegister(u8);

#[derive(Debug, Default)]
pub struct MaskRegister(u8);

#[derive(Debug, Default)]
pub struct StatusRegister(u8);

#[derive(Debug, Default)]
pub struct OamAddressRegister {}

#[derive(Debug, Default)]
pub struct OamDataRegister {}

#[derive(Debug, Default)]
pub struct ScrollRegister {
    low: u8,
    high: u8,
}

#[derive(Debug, Default)]
pub struct AddressRegiser {
    low: u8,
    high: u8,
}

#[derive(Debug, Default)]
pub struct DataRegister {}
