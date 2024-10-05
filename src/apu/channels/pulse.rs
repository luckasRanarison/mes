use crate::{
    apu::{
        envelope::Envelope,
        frame_counter::{ClockHalfFrame, ClockQuarterFrame},
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
    duty_cycle: u8,
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

    fn is_mute(&self) -> bool {
        !self.length_counter.active()
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

impl Channel for Pulse {
    fn write(&mut self, address: u16, value: u8) {
        match address % 4 {
            0 => {
                self.duty_cycle = value.get_range(6..8);
                self.length_counter.set_halt(value.contains(5));
                self.envelope.write(value);
            }
            1 => self.sweep.write(value),
            2 => self.timer.set_period_lo(value),
            _ => {
                self.timer.set_period_hi(value.get_range(0..2));
                self.length_counter.set_length(value.get_range(3..8));
            }
        }
    }

    fn sample(&self) -> u8 {
        match self.is_mute() {
            false => {
                let duty = self.duty_cycle as usize;
                let seq = self.sequencer.index();
                WAVEFORMS[duty][seq] * self.envelope.volume()
            }
            true => 0,
        }
    }

    fn active(&self) -> bool {
        self.length_counter.active()
    }

    fn set_enabled(&mut self, value: bool) {
        self.length_counter.set_enabled(value);
    }
}

impl ClockHalfFrame for Pulse {
    fn tick_half(&mut self) {
        self.length_counter.tick();
        self.sweep.update_period(&mut self.timer);
    }
}

impl ClockQuarterFrame for Pulse {
    fn tick_quarter(&mut self) {
        self.envelope.tick();
    }
}
