// Commands.rs
// SUPAHAXOR //
// 18/07/2026 //


use crate::gpu::rasterizer::Framebuffer;
use crate::gpu::rasterizer::Vertex;

fn edge(a: &Vertex, b: &Vertex, p: &Vertex) -> i32 {

    (p.x - a.x) * (b.y - a.y) -
    (p.y - a.y) * (b.x - a.x)
}

pub fn draw_triangle(
    fb: &mut Framebuffer,
    v0: Vertex,
    v1: Vertex,
    v2: Vertex,
    color: u32,
) {
    let min_x = v0.x.min(v1.x).min(v2.x);
    let max_x = v0.x.max(v1.x).min(v2.x);

    let min_y = v0.y.min(v1.y).min(v2.y);
    let max_y = v0.y.max(v1.y).max(v2.y);

    let mut pixels_drawn = 0;

    for y in min_y..=max_y {
        for x in min_x..=max_x {

            let p = Vertex{x,y};

            let w0 = edge(&v1,&v2,&p);
            let w1 = edge(&v2, &v0, &p);
            let w2 = edge(&v0, &v1, &p);

            if w0 >= 0 && w1 >= 0 && w2 >= 0 {
                fb.set_pixel(x, y, color);
                println!("{:X}", fb.pixels[v0.y as usize * fb.width + v0.x as usize]); // Writes to RAM
                pixels_drawn += 1;
            }
        }
    }
    println!("Pixels drawn: {}", pixels_drawn)

}