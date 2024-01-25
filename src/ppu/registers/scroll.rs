#[derive(Debug, Default)]
pub struct ScrollRegister {
    x: u8,
    y: u8,
}

impl ScrollRegister {
    pub fn write(&mut self, value: u8, latch: &mut bool) {
        if *latch {
            self.y = value;
        } else {
            self.x = value;
        }

        *latch = !*latch;
    }

    pub fn get_x(&self) -> u8 {
        self.x
    }

    pub fn get_y(&self) -> u8 {
        self.y
    }
}
