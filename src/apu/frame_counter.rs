// https://www.nesdev.org/wiki/APU_Frame_Counter

use crate::utils::{BitFlag, Clock};

#[derive(Debug)]
enum Flag {
    //_,
    //_,
    //_,
    //_,
    //_,
    //_,
    I = 6,
    M = 7,
}

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    FourSteps,
    FiveSteps,
}

#[derive(Debug)]
pub enum Frame {
    Quarter,
    Half,
}

#[derive(Debug, Default)]
pub struct FrameCounter {
    flags: u8,
    sequencer: u32,
    frame: Option<Frame>,
    interrupt: bool,
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

impl FrameCounter {
    pub fn write(&mut self, value: u8) {
        self.flags = value;
        self.interrupt = !value.contains(Flag::I as u8);
        self.sequencer = 0; // FIXME: apply 3-4 cycle delay
    }

    pub fn take_frame(&mut self) -> Option<Frame> {
        self.frame.take()
    }

    pub fn irq(&self) -> bool {
        self.interrupt
    }

    pub fn clear_interrupt(&mut self) {
        self.interrupt = false;
    }

    fn sequencer_mode(&self) -> Mode {
        match self.flags.contains(Flag::M as u8) {
            true => Mode::FiveSteps,
            false => Mode::FourSteps,
        }
    }

    fn set_interrupt(&mut self) {
        if !self.flags.contains(Flag::I as u8) {
            self.interrupt = true;
        }
    }
}
