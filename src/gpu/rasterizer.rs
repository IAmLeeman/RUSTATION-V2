// Custom rasterizer 
// ULTRAVIOLENCE //
// SUPAHAXOR //


pub struct Vertex {
    pub x: i32,
    pub y: i32,
}

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u32>, // RGBA set-up. // This is basically just a huge array.
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; width * height]
        }
    }
    #[inline]
    fn set_pixel(&mut self, x: i32, y: i32, color: u32) {
        if x < 0 || y < 0 {
            return;
        }
        let (x, y) = (x as usize, y as usize);
        if x >= self.width || y >= self.height {
            return;
        }

        self.pixels[y * self.width + x] = color;

    }
}