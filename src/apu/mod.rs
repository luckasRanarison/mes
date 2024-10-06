// https://www.nesdev.org/wiki/APU

mod channels;
mod envelope;
mod frame_counter;
mod length_counter;
mod sequencer;
mod sweep;
mod timer;

use channels::{Channel, Noise, Pulse, Triangle};
use frame_counter::{ClockFrame, FrameCounter};

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
    triangle: Triangle,
    noise: Noise,
    frame_counter: FrameCounter,
    cycle: u64,
    buffer: Vec<f32>,
}

impl Apu {
    pub fn new() -> Self {
        Self {
            pulse1: Pulse::channel1(),
            pulse2: Pulse::channel2(),
            triangle: Triangle::new(),
            noise: Noise::new(),
            ..Default::default()
        }
    }

    pub fn read_status(&mut self) -> u8 {
        let mut status = 0;

        status.update(status_flag::P1, self.pulse1.is_active());
        status.update(status_flag::P2, self.pulse2.is_active());
        status.update(status_flag::T, self.triangle.is_active());
        status.update(status_flag::N, self.noise.is_active());
        //status.update(status_flag::D, todo!());
        status.update(status_flag::F, self.frame_counter.irq());
        //status.update(status_flag::I, todo!());

        self.frame_counter.clear_interrupt();

        status
    }

    pub fn write_pulse1(&mut self, address: u16, value: u8) {
        self.pulse1.write_register(address, value);
    }

    pub fn write_pulse2(&mut self, address: u16, value: u8) {
        self.pulse2.write_register(address, value);
    }

    pub fn write_triangle(&mut self, address: u16, value: u8) {
        self.triangle.write_register(address, value);
    }

    pub fn write_noise(&mut self, address: u16, value: u8) {
        self.noise.write_register(address, value);
    }

    pub fn write_status(&mut self, value: u8) {
        self.pulse1.set_enabled(value.contains(status_flag::P1));
        self.pulse2.set_enabled(value.contains(status_flag::P2));
        self.triangle.set_enabled(value.contains(status_flag::T));
        self.noise.set_enabled(value.contains(status_flag::N));
    }

    pub fn write_frame_counter(&mut self, value: u8) {
        self.frame_counter.write(value);
    }

    pub fn poll_irq(&self) -> Option<Interrupt> {
        self.frame_counter.irq().then_some(Interrupt::Irq)
    }

    pub fn drain_buffer(&mut self) -> Vec<f32> {
        self.buffer.drain(..).collect()
    }

    // https://www.nesdev.org/wiki/APU_Mixer
    fn get_sample(&self) -> f32 {
        let p1 = self.pulse1.get_sample() as f32;
        let p2 = self.pulse1.get_sample() as f32;
        let t = self.triangle.get_sample() as f32;
        let n = self.noise.get_sample() as f32;
        let d = 0.; // TODO: DMC

        let pulse_out = 95.88 / ((8128.0 / (p1 + p2)) + 100.0);
        let tnd_out = 159.79 / ((1.0 / ((t / 8227.0) + (n / 12241.0) + (d / 22638.0))) + 100.0);
        let output = pulse_out + tnd_out; // 0.0 to 1.0

        output * 2.0 - 1.0 // -1 to 1
    }
}

impl Clock for Apu {
    fn tick(&mut self) {
        self.triangle.tick();

        if self.cycle % 2 == 1 {
            self.pulse1.tick();
            self.pulse2.tick();
            self.noise.tick();
        }

        self.frame_counter.tick();

        if let Some(frame) = self.frame_counter.take_frame() {
            self.pulse1.tick_frame(&frame);
            self.pulse2.tick_frame(&frame);
            self.triangle.tick_frame(&frame);
            self.noise.tick_frame(&frame);
        }

        // 44_100 / 60 == 735 samples/frame
        // 29970 (CPU cycle) / 735 == 40 cycles/frame
        if self.cycle % 40 == 0 {
            self.buffer.push(self.get_sample());
        }

        self.cycle += 1;
    }
}
