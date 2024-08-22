use std::ops::{BitAnd, BitAndAssign, BitOrAssign, Not, Shl, Shr};

pub trait BitFlag<T> {
    fn get(&self, flag: T) -> T;
    fn contains(&self, flag: T) -> bool;
    fn set(&mut self, flag: T);
    fn clear(&mut self, flag: T);
    fn update(&mut self, flag: T, cond: bool);
}

impl<T> BitFlag<T> for T
where
    T: Clone
        + Copy
        + PartialEq
        + Shr<T, Output = T>
        + Shl<T, Output = T>
        + BitAnd<T, Output = T>
        + BitAndAssign<T>
        + BitOrAssign<T>
        + Not<Output = T>
        + From<u8>,
{
    fn get(&self, flag: T) -> T {
        *self >> flag & T::from(1)
    }

    fn contains(&self, flag: T) -> bool {
        self.get(flag) == T::from(1)
    }

    fn set(&mut self, flag: T) {
        *self |= T::from(1) << flag;
    }

    fn clear(&mut self, flag: T) {
        *self &= !(T::from(1) << flag);
    }

    fn update(&mut self, flag: T, cond: bool) {
        if cond {
            self.set(flag);
        } else {
            self.clear(flag);
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct BitPlane<T> {
    pub low: T,
    pub high: T,
}

pub trait Clock {
    fn tick(&mut self) {}
}

pub trait Reset {
    fn reset(&mut self);
}

impl<T> Reset for T
where
    T: Default,
{
    fn reset(&mut self) {
        *self = Self::default();
    }
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
