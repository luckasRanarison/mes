const PRIMARY_OAM_SIZE: usize = 256;
const SECONDARY_OAM_SIZE: usize = 32;

#[derive(Debug)]
pub struct OamData {
    pub address: u8,
    pub buffer: u8,
    pub primary: [u8; PRIMARY_OAM_SIZE],
    pub secondary: [u8; SECONDARY_OAM_SIZE],
    pub primary_index: u8,
    pub secondary_index: u8,
    pub index_overflow: bool,
}

impl Default for OamData {
    fn default() -> Self {
        Self {
            address: 0,
            buffer: 0,
            primary: [0; PRIMARY_OAM_SIZE],
            secondary: [0; SECONDARY_OAM_SIZE],
            primary_index: 0,
            secondary_index: 0,
            index_overflow: false,
        }
    }
}
