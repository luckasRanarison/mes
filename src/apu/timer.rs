use crate::utils::Clock;

#[derive(Debug, Default)]
pub struct Timer {
    pub period: u16,
    pub counter: u16,
}

impl Clock for Timer {
    fn tick(&mut self) {
        if self.counter == 0 {
            self.counter = self.period;
        } else {
            self.counter -= 1;
        }
    }
}

impl Timer {
    pub fn is_zero(&self) -> bool {
        self.counter == 0
    }

    pub fn set_period_hi(&mut self, value: u8) {
        self.period = (self.period & 0x00FF) | ((value as u16) << 8);
    }

    pub fn set_period_lo(&mut self, value: u8) {
        self.period = (self.period & 0xFF00) | value as u16;
    }
}
