mod address;
mod control;
mod mask;
mod status;

pub use address::AddressRegiser;
pub use control::ControlRegister;
pub use mask::MaskRegister;
pub use status::{StatusFlag, StatusRegister};
