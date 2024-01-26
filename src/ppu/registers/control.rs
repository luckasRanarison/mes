use crate::utils::BitFlag;

enum ControlFlag {
    N0,
    N1,
    I,
    S,
    B,
    H,
    P,
    V,
}

#[derive(Debug, Default)]
pub struct ControlRegister(u8);

impl ControlRegister {
    pub fn write(&mut self, value: u8) {
        self.0 = value;
    }

    pub fn get_base_nametable_address(&self) -> u16 {
        let value = self.get(ControlFlag::N1) * 2 + self.get(ControlFlag::N0);

        match value {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2C00,
            _ => unreachable!(),
        }
    }

    pub fn get_vram_increment_value(&self) -> u8 {
        match self.contains(ControlFlag::I) {
            true => 32,
            false => 1,
        }
    }

    pub fn get_sprite_pattern_table_address(&self) -> u16 {
        match self.contains(ControlFlag::S) {
            true => 0x1000,
            false => 0x0000,
        }
    }

    pub fn get_background_pattern_table_address(&self) -> u16 {
        match self.get(ControlFlag::B) == 1 {
            true => 0x1000,
            false => 0x0000,
        }
    }

    pub fn get_sprite_height(&self) -> u16 {
        match self.contains(ControlFlag::H) {
            true => 16,
            false => 8,
        }
    }

    pub fn get_master_slave_select(&self) -> u8 {
        self.get(ControlFlag::P)
    }

    pub fn generate_nmi(&self) -> bool {
        self.contains(ControlFlag::V)
    }

    fn get(&self, flag: ControlFlag) -> u8 {
        self.0.get(flag as u8)
    }

    fn contains(&self, flag: ControlFlag) -> bool {
        self.0.contains(flag as u8)
    }
}

#[cfg(test)]
mod tests {
    use super::ControlRegister;

    #[test]
    fn test_base_nametable_address() {
        let mut register = ControlRegister::default();

        register.write(0b0000_0000);
        assert_eq!(register.get_base_nametable_address(), 0x2000);

        register.write(0b0000_0001);
        assert_eq!(register.get_base_nametable_address(), 0x2400);

        register.write(0b0000_0010);
        assert_eq!(register.get_base_nametable_address(), 0x2800);

        register.write(0b0000_0011);
        assert_eq!(register.get_base_nametable_address(), 0x2C00);
    }
}
