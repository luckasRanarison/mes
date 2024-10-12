// https://www.nesdev.org/wiki/APU

mod channels;
mod frame_counter;

use channels::{Channel, Dmc, Noise, Pulse, Triangle};
use frame_counter::{ClockFrame, FrameCounter};

use crate::{
    cpu::interrupt::Interrupt,
    mappers::MapperChip,
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

const BUFFER_CAPACITY: usize = 735; // approxiamte sample/frame

#[derive(Debug)]
pub struct Apu {
    pulse1: Pulse,
    pulse2: Pulse,
    triangle: Triangle,
    noise: Noise,
    dmc: Dmc,
    frame_counter: FrameCounter,
    buffer: Box<[f32; BUFFER_CAPACITY]>,
    write_index: usize,
    cycle: u64,
}

impl Apu {
    pub fn new(mapper: MapperChip) -> Self {
        Self {
            pulse1: Pulse::channel1(),
            pulse2: Pulse::channel2(),
            triangle: Triangle::new(),
            noise: Noise::new(),
            dmc: Dmc::new(mapper),
            frame_counter: FrameCounter::default(),
            buffer: Box::new([0.0; BUFFER_CAPACITY]),
            write_index: 0,
            cycle: 0,
        }
    }

    pub fn read_status(&mut self) -> u8 {
        let status = (self.pulse1.is_active() as u8) << status_flag::P1
            | (self.pulse2.is_active() as u8) << status_flag::P2
            | (self.triangle.is_active() as u8) << status_flag::T
            | (self.noise.is_active() as u8) << status_flag::N
            | (self.dmc.is_active() as u8) << status_flag::D
            | (self.frame_counter.irq() as u8) << status_flag::F
            | (self.dmc.irq() as u8) << status_flag::I;

        self.frame_counter.clear_irq();

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

    pub fn write_dmc(&mut self, address: u16, value: u8) {
        self.dmc.write_register(address, value);
    }

    pub fn write_status(&mut self, value: u8) {
        self.pulse1.set_enabled(value.contains(status_flag::P1));
        self.pulse2.set_enabled(value.contains(status_flag::P2));
        self.triangle.set_enabled(value.contains(status_flag::T));
        self.noise.set_enabled(value.contains(status_flag::N));
        self.dmc.set_enabled(value.contains(status_flag::D));

        self.dmc.clear_irq();
    }

    pub fn write_frame_counter(&mut self, value: u8) {
        self.frame_counter.write(value);
    }

    pub fn poll_irq(&self) -> Option<Interrupt> {
        match self.frame_counter.irq() || self.dmc.irq() {
            true => Some(Interrupt::Irq),
            false => None,
        }
    }

    pub fn get_buffer(&self) -> &[f32] {
        &self.buffer[..self.write_index]
    }

    pub fn clear_buffer(&mut self) {
        while self.write_index > 0 {
            self.buffer[self.write_index - 1] = 0.0;
            self.write_index -= 1;
        }
    }

    pub fn take_dmc_cycles(&mut self) -> Option<u8> {
        self.dmc.take_dma_cycles()
    }

    pub fn incoming_dma(&self) -> bool {
        self.cycle % 2 == 1 && self.dmc.should_fetch()
    }

    // https://www.nesdev.org/wiki/APU_Mixer
    fn get_sample(&self) -> f32 {
        let p1 = self.pulse1.get_sample();
        let p2 = self.pulse2.get_sample();
        let t = self.triangle.get_sample();
        let n = self.noise.get_sample();
        let d = self.dmc.get_sample();

        let pulse_out = 95.88 / ((8128.0 / (p1 + p2)) + 100.0);
        let tnd_out = 159.79 / ((1.0 / ((t / 8227.0) + (n / 12241.0) + (d / 22638.0))) + 100.0);

        pulse_out + tnd_out // 0.0 to 1.0
    }
}

impl Clock for Apu {
    fn tick(&mut self) {
        self.triangle.tick();

        if self.cycle % 2 == 1 {
            self.pulse1.tick();
            self.pulse2.tick();
            self.noise.tick();
            self.dmc.tick();
        }

        self.frame_counter.tick();

        if let Some(frame) = self.frame_counter.take_frame() {
            self.pulse1.tick_frame(&frame);
            self.pulse2.tick_frame(&frame);
            self.triangle.tick_frame(&frame);
            self.noise.tick_frame(&frame);
        }

        // TODO: Find a better sampling strategy
        // 44_100 / 60 == 735.x samples/frame
        // 29780 (CPU cycles) / 735 == 40.x cycles/frame
        if self.cycle % 41 == 0 && self.write_index < BUFFER_CAPACITY {
            self.buffer[self.write_index] = self.get_sample();
            self.write_index += 1;
        }

        self.cycle += 1;
    }
}
