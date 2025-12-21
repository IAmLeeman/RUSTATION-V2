
mod cpu;
mod memory;
mod bios;


use memory::ram::Ram;
use bios::bios::BIOS;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use std::time::Duration;

fn main() -> Result<(), String> {

    println!("Rustation V2 Booting");

    let mut memory = Ram::new();
    memory.write(0x1234, 42);
    println!("Memory at 0x1234: {}", memory.read(0x1234));

    let bios = BIOS::load("SCPH7501.BIN").expect("failed to load BIOS");
    println!("SCPH7501.BIN loaded, size:{} bytes", bios.data.len());

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window= video_subsystem.window("RUSTATION", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit {..} = event {
                break 'running;
            }
        }
        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())

}
