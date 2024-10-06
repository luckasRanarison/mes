#[derive(Debug, Default)]
pub struct Sequencer {
    steps: usize,
    current: usize,
}

impl Sequencer {
    pub fn new(steps: usize) -> Self {
        Self { steps, current: 0 }
    }

    pub fn step(&mut self) {
        self.current = (self.current + 1) % self.steps;
    }

    pub fn index(&self) -> usize {
        self.current
    }

    pub fn reset(&mut self) {
        self.current = 0;
    }
}
