mod frame_counter;
mod sweep;

use frame_counter::{Frame, FrameCounter};

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
    frame_counter: FrameCounter,
}

impl Clock for Apu {
    fn tick(&mut self) {
        self.frame_counter.tick();

        match self.frame_counter.take_frame() {
            Some(Frame::Quarter) => todo!(),
            Some(Frame::Half) => todo!(),
            None => {}
        }
    }
}

impl Apu {
    pub fn read_status(&mut self) -> u8 {
        let mut status = 0;

        status.update(status_flag::P1, todo!());
        status.update(status_flag::P2, todo!());
        status.update(status_flag::T, todo!());
        status.update(status_flag::N, todo!());
        status.update(status_flag::D, todo!());
        status.update(status_flag::F, self.frame_counter.irq());
        status.update(status_flag::I, todo!());

        self.frame_counter.clear_interrupt();

        status
    }

    pub fn write_status(&mut self, value: u8) {
        // TODO: enable/disable sound channels
    }

    pub fn write_frame_counter(&mut self, value: u8) {
        self.frame_counter.write(value);
    }

    pub fn poll_irq(&self) -> Option<Interrupt> {
        self.frame_counter.irq().then_some(Interrupt::Irq)
    }
}
