mod noise;
mod pulse;

pub use noise::Noise;
pub use pulse::Pulse;

pub trait Channel {
    fn write_register(&mut self, address: u16, value: u8);
    fn raw_sample(&self) -> u8;
    fn is_active(&self) -> bool;
    fn is_mute(&self) -> bool;
    fn set_enabled(&mut self, value: bool);

    fn get_sample(&self) -> u8 {
        match self.is_mute() {
            true => 0,
            false => self.raw_sample(),
        }
    }
}
