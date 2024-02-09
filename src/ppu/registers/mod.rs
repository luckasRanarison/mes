mod address;
mod control;
mod mask;
mod status;

pub use address::AddressRegister;
pub use control::ControlRegister;
pub use mask::MaskRegister;
pub use status::{StatusFlag, StatusRegister};
