use core::{cell::RefCell, f32::consts::PI};

const SAMPLE_RATE: f32 = 44100.0;

#[derive(Debug)]
pub struct Filter {
    b0: f32,
    b1: f32,
    a1: f32,
    prev_x: f32,
    prev_y: f32,
}

impl Filter {
    pub fn low_pass(sample_rate: f32, cutoff_freq: f32) -> Self {
        let c = sample_rate / (cutoff_freq * PI);
        let a0 = 1.0 / (1.0 + c);

        Self {
            b0: a0,
            b1: a0,
            a1: (1.0 - c) * a0,
            prev_x: 0.0,
            prev_y: 0.0,
        }
    }

    pub fn high_pass(sample_rate: f32, cutoff_freq: f32) -> Self {
        let c = sample_rate / (cutoff_freq * PI);
        let a0 = 1.0 / (1.0 + c);

        Self {
            b0: c * a0,
            b1: -c * a0,
            a1: (1.0 - c) * a0,
            prev_x: 0.0,
            prev_y: 0.0,
        }
    }

    pub fn process(&mut self, x: f32) -> f32 {
        let y = self.b0 * x + self.b1 * self.prev_x - self.a1 * self.prev_y;

        self.prev_x = x;
        self.prev_y = y;

        y
    }
}

#[derive(Debug)]
pub struct FilterChain(RefCell<[Filter; 3]>);

// https://www.nesdev.org/wiki/APU_Mixer
impl Default for FilterChain {
    fn default() -> Self {
        Self(RefCell::new([
            Filter::high_pass(SAMPLE_RATE, 90.0),
            Filter::high_pass(SAMPLE_RATE, 440.0),
            Filter::low_pass(SAMPLE_RATE, 14000.0),
        ]))
    }
}

impl FilterChain {
    pub fn process(&self, sample: f32) -> f32 {
        self.0
            .borrow_mut()
            .iter_mut()
            .fold(sample, |acc, f| f.process(acc))
    }
}
