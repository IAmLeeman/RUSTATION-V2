// RUSTATION //
// ULTRAVIOLENCE //
// 02/07/2026 //

mod cpu;
mod memory;
mod bios;
mod gpu;


use memory::ram::Ram;
use bios::bios::BIOS;


extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use std::time::Duration;
use std::time::Instant;
use gpu::commands::draw_triangle; // Debug command to draw triangle

use crate::gpu::rasterizer::{Framebuffer, Vertex};




fn main() -> Result<(), String> {

    let start = Instant::now();
    let elapsed = start.elapsed();
    let mut fb = Framebuffer{width:640, height:480, pixels:vec![0; 640 * 480]};
    //let color = 5;

    println!("Rustation V2 Booting");
    println!("Time Elapsed: {:?}", elapsed);

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

        let v0 = Vertex {x: 100, y: 200};
    
        let v1 = Vertex {x: 200, y: 300};
        let v2 = Vertex {x: 250, y: 250};

        let color = 0xFFFF0000; // Colour of traingle
        
        
        
        draw_triangle(&mut fb, v0, v1, v2, color); // Doesn't draw to the screen
        
        ::std::thread::sleep(Duration::from_millis(100));
        canvas.clear();

        canvas.copy(&texture, None, Some(target)).unwrap();
        canvas.present();

        ::std::thread::sleep(Duration::from_millis(16));
    }

    Ok(())

}
