use crate::utils::BitFlag;

#[allow(unused)]
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
// VPHB SINN
// |||| ||||
// |||| ||++- Base nametable address
// |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
// |||| |+--- VRAM address increment per CPU read/write of PPUDATA
// |||| |     (0: add 1, going across; 1: add 32, going down)
// |||| +---- Sprite pattern table address for 8x8 sprites
// ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
// |||+------ Background pattern table address (0: $0000; 1: $1000)
// ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels – see PPU OAM#Byte 1)
// |+-------- PPU master/slave select
// |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
// +--------- Generate an NMI at the start of the
//            vertical blanking interval (0: off; 1: on)

impl ControlRegister {
    pub fn write(&mut self, value: u8) {
        self.0 = value;
    }

    pub fn get_nametable_bits(&self) -> u8 {
        self.get(ControlFlag::N1) * 2 + self.get(ControlFlag::N0)
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

    pub fn get_sprite_height(&self) -> u8 {
        match self.contains(ControlFlag::H) {
            true => 16,
            false => 8,
        }
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
