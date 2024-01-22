#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Register {
    PC,
    AC,
    X,
    Y,
    SR,
    SP,
}

#[derive(Debug, Default)]
pub struct StatusRegister {
    pub c: bool,
    pub z: bool,
    pub i: bool,
    pub d: bool,
    pub b: bool,
    pub __: bool,
    pub v: bool,
    pub n: bool,
}

impl StatusRegister {
    pub fn new() -> Self {
        Self {
            __: true,
            ..Default::default()
        }
    }

    pub fn update_z(&mut self, value: u8) {
        self.z = value == 0;
    }

    pub fn update_n(&mut self, value: u8) {
        self.n = value >> 7 == 1;
    }

    pub fn as_u8(&self) -> u8 {
        ((self.n as u8) << 7)
            + ((self.v as u8) << 6)
            + ((self.__ as u8) << 5)
            + ((self.b as u8) << 4)
            + ((self.d as u8) << 3)
            + ((self.i as u8) << 2)
            + ((self.z as u8) << 1)
            + (self.c as u8)
    }

    pub fn from_u8(value: u8) -> Self {
        Self {
            c: value == 1,
            z: value == 1 << 1,
            i: value == 1 << 2,
            d: value == 1 << 3,
            b: value == 1 << 4,
            __: value == 1 << 5,
            v: value == 1 << 6,
            n: value == 1 << 7,
        }
    }
}
