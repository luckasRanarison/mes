mod envelope;
mod frame_counter;
mod length_counter;
mod pulse;
mod sequencer;
mod sweep;
mod timer;

use frame_counter::{ClockHalfFrame, ClockQuarterFrame, Frame, FrameCounter};
use pulse::Pulse;

use crate::{
    cpu::interrupt::Interrupt,
    utils::{BitFlag, Clock},
};

#[rustfmt::skip]
mod status_flag {
    pub const P1: u8 = 0;
    pub const P2: u8 = 1;
    pub const T : u8 = 2;
    pub const N : u8 = 3;
    pub const D : u8 = 4;
    pub const F : u8 = 6;
    pub const I : u8 = 7;
}

#[derive(Debug, Default)]
pub struct Apu {
    pulse1: Pulse,
    pulse2: Pulse,
    frame_counter: FrameCounter,
    odd_cycle: bool,
}

impl Clock for Apu {
    fn tick(&mut self) {
        self.frame_counter.tick();

        if self.odd_cycle {
            self.pulse1.tick_timer();
            self.pulse2.tick_timer();
        }

        self.odd_cycle = !self.odd_cycle;

        match self.frame_counter.take_frame() {
            Some(Frame::Quarter) => {
                self.pulse1.tick_quarter();
                self.pulse2.tick_quarter();
            }
            Some(Frame::Half) => {
                self.pulse1.tick_half();
                self.pulse2.tick_half();
            }
            None => {}
        }
    }
}

impl Apu {
    pub fn new() -> Self {
        Self {
            pulse1: Pulse::channel1(),
            pulse2: Pulse::channel2(),
            ..Default::default()
        }
    }

    pub fn read_status(&mut self) -> u8 {
        let mut status = 0;

        status.update(status_flag::P1, self.pulse1.active());
        status.update(status_flag::P2, self.pulse2.active());
        //status.update(status_flag::T, todo!());
        //status.update(status_flag::N, todo!());
        //status.update(status_flag::D, todo!());
        status.update(status_flag::F, self.frame_counter.irq());
        //status.update(status_flag::I, todo!());

        self.frame_counter.clear_interrupt();

        status
    }

    pub fn write_pulse1(&mut self, address: u16, value: u8) {
        self.pulse1.write(address, value);
    }

    pub fn write_pulse2(&mut self, address: u16, value: u8) {
        self.pulse2.write(address, value);
    }

    pub fn write_status(&mut self, value: u8) {
        self.pulse1.set_enabled(value.contains(status_flag::P1));
        self.pulse2.set_enabled(value.contains(status_flag::P2));
    }

    pub fn write_frame_counter(&mut self, value: u8) {
        self.frame_counter.write(value);
    }

    pub fn poll_irq(&self) -> Option<Interrupt> {
        self.frame_counter.irq().then_some(Interrupt::Irq)
    }
}
