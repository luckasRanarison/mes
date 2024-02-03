use crate::utils::BitFlag;

#[derive(Debug, Default)]
pub struct Sprite {
    pub x: u8,
    pub y: u8,
    pub tile: u8,
    pub attribute: u8,
}

impl Sprite {
    pub fn get_palette(&self) -> u8 {
        let low = self.attribute.get(0);
        let high = self.attribute.get(1);
        (high << 1) | low
    }

    pub fn get_tile_address(&self, height: u8, base_address: u16) -> u16 {
        if height == 8 {
            base_address + (16 * base_address as u16)
        } else {
            let bank = 0x1000 * self.attribute.get(0) as u16;
            bank + 32 * (self.attribute >> 1) as u16
        }
    }

    pub fn has_background_priority(&self) -> bool {
        !self.attribute.contains(5)
    }

    pub fn is_flipped_horizontally(&self) -> bool {
        self.attribute.contains(6)
    }

    pub fn is_flipped_vertically(&self) -> bool {
        self.attribute.contains(7)
    }
}
