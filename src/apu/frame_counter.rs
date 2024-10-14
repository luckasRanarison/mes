// https://www.nesdev.org/wiki/APU_Frame_Counter

use crate::utils::{BitFlag, Clock};

mod status_flag {
    pub const I: u8 = 6;
    pub const M: u8 = 7;
}

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    FourSteps,
    FiveSteps,
}

pub trait ClockFrame {
    fn tick_frame(&mut self, frame: &Frame);
}

#[derive(Debug)]
pub enum Frame {
    Quarter,
    Half,
}

impl Frame {
    pub fn is_half(&self) -> bool {
        matches!(self, Frame::Half)
    }
}

#[derive(Debug, Default)]
pub struct FrameCounter {
    flags: u8,
    sequencer: u32,
    frame: Option<Frame>,
    interrupt: bool,
}

impl FrameCounter {
    pub fn write(&mut self, value: u8) {
        self.flags = value;
        self.interrupt = self.interrupt && !value.contains(status_flag::I);
        self.sequencer = 0; // FIXME: apply 3-4 cycle delay
    }

    pub fn take_frame(&mut self) -> Option<Frame> {
        self.frame.take()
    }

    pub fn irq(&self) -> bool {
        self.interrupt
    }

    pub fn clear_irq(&mut self) {
        self.interrupt = false;
    }

    fn sequencer_mode(&self) -> Mode {
        match self.flags.contains(status_flag::M) {
            true => Mode::FiveSteps,
            false => Mode::FourSteps,
        }
    }

    fn set_interrupt(&mut self) {
        self.interrupt = !self.flags.contains(status_flag::I);
    }
}

impl Clock for FrameCounter {
    fn tick(&mut self) {
        let mode = self.sequencer_mode();

        match (self.sequencer, mode) {
            (14913, _) => self.frame = Some(Frame::Half),
            (7457, _) | (22371, _) => self.frame = Some(Frame::Quarter),
            (29828, Mode::FourSteps) => self.set_interrupt(),
            (29829, Mode::FourSteps) => {
                self.set_interrupt();
                self.frame = Some(Frame::Half);
            }
            (29830, Mode::FourSteps) => {
                self.set_interrupt();
                self.sequencer = 0;
            }
            (37281, Mode::FiveSteps) => self.frame = Some(Frame::Half),
            (37282, Mode::FiveSteps) => self.sequencer = 0,
            _ => {}
        };

        self.sequencer += 1;
    }
}
