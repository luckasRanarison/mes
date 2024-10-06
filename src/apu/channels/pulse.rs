// https://www.nesdev.org/wiki/APU_Pulse

use crate::{
    apu::{
        envelope::Envelope,
        frame_counter::{ClockFrame, Frame},
        length_counter::LengthCounter,
        sequencer::Sequencer,
        sweep::Sweep,
        timer::Timer,
    },
    utils::{BitFlag, Clock},
};

use super::Channel;

const WAVEFORMS: [[u8; 8]; 4] = [
    [0, 1, 0, 0, 0, 0, 0, 0],
    [0, 1, 1, 0, 0, 0, 0, 0],
    [0, 1, 1, 1, 1, 0, 0, 0],
    [1, 0, 0, 1, 1, 1, 1, 1],
];

#[derive(Debug, Default)]
pub struct Pulse {
    duty_mode: u8,
    length_counter: LengthCounter,
    sweep: Sweep,
    timer: Timer,
    sequencer: Sequencer,
    envelope: Envelope,
}

impl Pulse {
    pub fn channel1() -> Self {
        Self {
            sweep: Sweep::new(1),
            sequencer: Sequencer::new(8),
            ..Default::default()
        }
    }

    pub fn channel2() -> Self {
        Self {
            sweep: Sweep::new(0),
            sequencer: Sequencer::new(8),
            ..Default::default()
        }
    }
}

impl Channel for Pulse {
    fn write_register(&mut self, address: u16, value: u8) {
        match address % 4 {
            0 => {
                self.duty_mode = value.get_range(6..8);
                self.length_counter.set_halt(value.contains(5));
                self.envelope.write(value);
            }
            1 => self.sweep.write(value),
            2 => self.timer.set_period_lo(value),
            _ => {
                self.timer.set_period_hi(value.get_range(0..3));
                self.length_counter.set_length(value.get_range(3..8));
                self.sequencer.reset();
                self.envelope.restart();
            }
        }
    }

    fn raw_sample(&self) -> u8 {
        let duty = self.duty_mode as usize;
        let seq = self.sequencer.index();
        WAVEFORMS[duty][seq] * self.envelope.volume()
    }

    fn is_active(&self) -> bool {
        self.length_counter.is_active()
    }

    fn set_enabled(&mut self, value: bool) {
        self.length_counter.set_enabled(value);
    }

    fn is_mute(&self) -> bool {
        !self.length_counter.is_active()
            || self.sweep.target_period(&self.timer) > 0x7FF
            || self.timer.period < 8
    }
}

impl Clock for Pulse {
    fn tick(&mut self) {
        self.timer.tick();

        if self.timer.is_zero() {
            self.sequencer.step();
        }
    }
}

impl ClockFrame for Pulse {
    fn tick_frame(&mut self, frame: &Frame) {
        self.envelope.tick();

        if frame.is_half() {
            self.length_counter.tick();
            self.sweep.update_period(&mut self.timer);
        }
    }
}
