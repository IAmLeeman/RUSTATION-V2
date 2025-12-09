
mod cpu;
mod memory;
mod bios;


use memory::ram::Ram;
use bios::bios::BIOS;

fn main(){
    println!("Rustation V2 Booting");

    let mut memory = Ram::new();
    memory.write(0x1234, 42);
    println!("Memory at 0x1234: {}", memory.read(0x1234))
}
