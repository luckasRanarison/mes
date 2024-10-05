use crate::utils::BitFlag;

use super::timer::Timer;

#[derive(Debug, Default)]
pub struct Sweep {
    enabled: bool,
    period: u8,
    negate: bool,
    shift: u8,
    counter: u8,
    reload: bool,
    negate_value: u8,
}

impl Sweep {
    pub fn new(negate_value: u8) -> Self {
        Self {
            negate_value,
            ..Default::default()
        }
    }

    pub fn write(&mut self, value: u8) {
        self.enabled = value.contains(7);
        self.period = value.get_range(4..7);
        self.negate = value.contains(3);
        self.shift = value.get_range(0..3);
        self.reload = true;
    }

    pub fn update_period(&mut self, timer: &mut Timer) {
        if self.counter == 0 && self.enabled && self.shift > 0 && timer.period >= 8 {
            let period = self.target_period(timer);

            // set the period if not "muting"
            if period <= 0x7FF {
                timer.period = period;
            }
        }

        if self.counter == 0 || self.reload {
            self.counter = self.period;
            self.reload = false;
        } else {
            self.counter -= 1;
        }
    }

    pub fn target_period(&self, timer: &Timer) -> u16 {
        let period = timer.period;
        let sweep_value = period >> self.shift;

        match self.negate {
            true => period - sweep_value - self.negate_value as u16,
            false => period + sweep_value,
        }
    }
}
