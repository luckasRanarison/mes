mod envelope;
mod length_counter;
mod sequencer;
mod sweep;
mod timer;

pub use envelope::Envelope;
pub use length_counter::LengthCounter;
pub use sequencer::Sequencer;
pub use sweep::Sweep;
pub use timer::Timer;

pub trait Channel {
    fn write_register(&mut self, address: u16, value: u8);
    fn raw_sample(&self) -> u8;
    fn is_active(&self) -> bool;
    fn is_mute(&self) -> bool;
    fn set_enabled(&mut self, value: bool);

    fn get_sample(&self) -> f32 {
        match self.is_mute() {
            true => 0.0,
            false => self.raw_sample() as f32,
        }
    }
}
