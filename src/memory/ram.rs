
pub struct Ram { 
    data: [u8; 0x10000], // Here we have an example 64kb of memory - I know massive.
}

impl Ram {
    pub fn new() -> Self {
        Ram { data: [0; 0x10000]}
        }

    pub fn read(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn write(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }
}
