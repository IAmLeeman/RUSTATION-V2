const BIOS_SIZE: usize = 0x800000; // 512kb size of bios, SCPH701.BIN in this case
const BIOS_START: usize = 0x000000; // FROM NOTHING

//use crate::memory::ram::Ram;

use std::fs;

pub struct BIOS {

    pub data: Vec<u8>,
}

impl BIOS{

    pub fn load(path: &str) -> Result<Self, String> {
        let bytes = fs::read(path)
            .map_err(|e| format!("Failed to load BIOS: {}", e))?;
        Ok(BIOS {data: bytes})
    }
    
    pub fn read(&self, addr: usize) -> u8 {
        self.data[addr]
    }
    
}