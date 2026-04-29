// RUSTATION //
// ULTRAVIOLENCE //
// 18/04/2026 //

mod cpu;
mod memory;
mod bios;


use memory::ram::Ram;
use bios::bios::BIOS;

extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
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

    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font("VCR_OSD_MONO.ttf", 24)?;

    let surface = font // Test 
        .render("RUSTATION IS LOADING...")
        .blended(Color::WHITE)
        .unwrap();

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator
        .create_texture_from_surface(&surface)
        .unwrap();

    let target = Rect::new(20, 20, surface.width(), surface.height());
    

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit {..} = event {
                break 'running;
            }
        }

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas.copy(&texture, None, Some(target)).unwrap();
        canvas.present();

        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())

}
