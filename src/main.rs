
mod cpu;
mod memory;

use memory::ram::Ram;

fn main(){
    println!("Rustation V2 Booting");

    let mut memory = Ram::new();
    memory.write(0x1234, 42);
    println!("Memory at 0x1234: {}", memory.read(0x1234))
}
