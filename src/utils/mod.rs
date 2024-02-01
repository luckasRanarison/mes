pub trait BitFlag<T> {
    fn get(&self, flag: T) -> T;
    fn contains(&self, flag: T) -> bool;
    fn set(&mut self, flag: T);
    fn clear(&mut self, flag: T);
    fn update(&mut self, flag: T, cond: bool);
}

impl BitFlag<u8> for u8 {
    fn get(&self, flag: u8) -> u8 {
        self >> flag & 1
    }

    fn contains(&self, flag: u8) -> bool {
        self.get(flag) == 1
    }

    fn set(&mut self, flag: u8) {
        *self |= 1 << flag;
    }

    fn clear(&mut self, flag: u8) {
        *self &= !(1 << flag);
    }

    fn update(&mut self, flag: u8, cond: bool) {
        if cond {
            self.set(flag);
        } else {
            self.clear(flag);
        }
    }
}

#[derive(Debug, Default)]
pub struct BitPlane<T> {
    pub low: T,
    pub high: T,
}

pub trait Clock {
    fn tick(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::BitFlag;

    #[test]
    fn test_register() {
        let mut bitflag = 0b1001_0000;

        bitflag.set(5);
        bitflag.clear(4);

        assert_eq!(bitflag, 0b1010_0000);
    }
}
