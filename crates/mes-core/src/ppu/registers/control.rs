use crate::utils::BitFlag;

#[allow(unused)]
#[rustfmt::skip]
mod control_flag {
    pub const N0: u8 = 0;
    pub const N1: u8 = 1;
    pub const I : u8 = 2;
    pub const S : u8 = 3;
    pub const B : u8 = 4;
    pub const H : u8 = 5;
    pub const P : u8 = 6;
    pub const V : u8 = 7;
}

/// VPHB SINN
/// |||| ||||
/// |||| ||++- Base nametable address
/// |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
/// |||| |+--- VRAM address increment per CPU read/write of PPUDATA
/// |||| |     (0: add 1, going across; 1: add 32, going down)
/// |||| +---- Sprite pattern table address for 8x8 sprites
/// ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
/// |||+------ Background pattern table address (0: $0000; 1: $1000)
/// ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels – see PPU OAM#Byte 1)
/// |+-------- PPU master/slave select
/// |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
/// +--------- Generate an NMI at the start of the
///            vertical blanking interval (0: off; 1: on)
#[derive(Debug, Default)]
pub struct ControlRegister(u8);

impl ControlRegister {
    pub fn write(&mut self, value: u8) {
        self.0 = value;
    }

    pub fn get_nametable_bits(&self) -> u8 {
        self.0.get(control_flag::N1) * 2 + self.0.get(control_flag::N0)
    }

    pub fn get_vram_increment_value(&self) -> u8 {
        match self.0.contains(control_flag::I) {
            true => 32,
            false => 1,
        }
    }

    pub fn get_sprite_pattern_table_address(&self) -> u16 {
        match self.0.contains(control_flag::S) {
            true => 0x1000,
            false => 0x0000,
        }
    }

    pub fn get_background_pattern_table_address(&self) -> u16 {
        match self.0.get(control_flag::B) == 1 {
            true => 0x1000,
            false => 0x0000,
        }
    }

    pub fn get_sprite_height(&self) -> u8 {
        match self.0.contains(control_flag::H) {
            true => 16,
            false => 8,
        }
    }

    pub fn generate_nmi(&self) -> bool {
        self.0.contains(control_flag::V)
    }
}
