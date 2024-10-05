use crate::utils::BitFlag;

#[derive(Debug, Default)]
pub struct Sweep {
    enabled: bool,
    period: u8,
    negate: bool,
    shift: u8,
    counter: u8,
    reload: bool,
}

impl Sweep {
    pub fn write(&mut self, value: u8) {
        self.enabled = value.contains(7);
        self.period = value.get_range(4..7);
        self.negate = value.contains(3);
        self.shift = value.get_range(0..3);
        self.reload = true;
    }
}
