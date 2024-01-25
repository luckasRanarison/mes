mod address;
mod control;
mod mask;
mod scroll;
mod status;

pub use address::AddressRegiser;
pub use control::ControlRegister;
pub use mask::MaskRegister;
pub use scroll::ScrollRegister;
pub use status::{StatusFlag, StatusRegister};
