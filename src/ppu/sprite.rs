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
            base_address + (16 * self.tile as u16)
        } else {
            let bank = 0x1000 * self.tile.get(0) as u16;
            bank + 32 * (self.tile >> 1) as u16
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

#[cfg(test)]
mod tests {
    use super::Sprite;

    #[test]
    fn test_sprite_tile_address() {
        let mut sprite = Sprite::default();

        sprite.tile = 0b0000_0011;
        assert_eq!(sprite.get_tile_address(16, 0x0000), 0x1020);
        assert_eq!(sprite.get_tile_address(8, 0x0000), 0x30);
        assert_eq!(sprite.get_tile_address(8, 0x1000), 0x1030);

        sprite.tile = 0b1111_1110;
        assert_eq!(sprite.get_tile_address(16, 0x0000), 0x0FE0);
        assert_eq!(sprite.get_tile_address(8, 0x0000), 0x0FE0);
        assert_eq!(sprite.get_tile_address(8, 0x1000), 0x1FE0);

        sprite.tile = 0b1111_1111;
        assert_eq!(sprite.get_tile_address(16, 0x0000), 0x1FE0);
    }
}
