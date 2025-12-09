pub mod opcodes;

pub struct CPU{
    pub pc: u32,                // Program counter
    pub regs: [u32; 32],
}

impl CPU {
    pub fn new() -> Self {
        Cpu {
            pc: 0,
            regs: [0, 32],
        }
    }

    pub fn step(&mut self, memory: &impl MemoryDevice) {
        let opcode = memory.read32(self.pc as usize);
    }
}