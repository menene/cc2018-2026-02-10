extern crate nalgebra_glm as glm;

use minifb::{Key, Window, WindowOptions};

use glm::Vec3;

mod framebuffer;
mod line;
mod polygon;

use crate::framebuffer::Framebuffer;
use crate::polygon::Polygon;

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 80;
    let framebuffer_height = 60;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Gráficas",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    let poligono = vec![
        Vec3::new(30.0, 20.0, 0.0),
        Vec3::new(50.0, 20.0, 0.0),
        Vec3::new(50.0, 40.0, 0.0),
        Vec3::new(30.0, 40.0, 0.0),
    ];

    framebuffer.set_current_color(0xFFCC00);
    framebuffer.filled_polygon(&poligono);

    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&poligono);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
    }
}
