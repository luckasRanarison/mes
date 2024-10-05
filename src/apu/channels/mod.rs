mod noise;
mod pulse;

pub use noise::Noise;
pub use pulse::Pulse;

pub trait Channel {
    fn write(&mut self, address: u16, value: u8);
    fn sample(&self) -> u8;
    fn active(&self) -> bool;
    fn is_mute(&self) -> bool;
    fn set_enabled(&mut self, value: bool);

    fn get_sample(&self) -> u8 {
        match self.is_mute() {
            true => 0,
            false => self.sample(),
        }
    }
}
