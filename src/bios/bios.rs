const BIOS_SIZE: usize = 0x800000; // 512kb size of bios, SCPH701.BIN in this case
const BIOS_START: usize = 0x000000; // FROM NOTHING

mod memory;

use memory::ram::Ram;

pub struct BIOS {

    data: Vec<u8>,
}

impl BIOS{

    pub fn new(bios_bytes: &[u8]) -> Self {
        BIOS {
            data: bios_bytes.toVec(),
        }
    }
}