// https://www.nesdev.org/wiki/INES

pub mod cartridge;
pub mod fds;
pub mod ines;

#[cfg(feature = "json")]
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(Serialize))]
pub enum Mirroring {
    Vertical,
    Horizontal,
    OneScreen,
    FourScreen,
}
