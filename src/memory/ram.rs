
pub struct Ram { 
    data: [u8; 0x200000], // Here we have an example 2mb of memory - I know massive.
}

impl Ram {
    pub fn new() -> Self {
        Ram { data: [0; 0x200000]}
        }

    pub fn read(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn write(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }
}
