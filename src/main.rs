extern crate nalgebra_glm as glm;

mod framebuffer;
mod line;
mod polygon;
mod bmp;

use glm::Vec3;

use crate::framebuffer::Framebuffer;
use crate::polygon::Polygon;
use crate::bmp::WriteBmp;

fn main() {
    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(0x000000);
    framebuffer.clear();

    let estrella = vec![
        Vec3::new(165.0, 380.0, 0.0),
        Vec3::new(185.0, 360.0, 0.0),
        Vec3::new(180.0, 330.0, 0.0),
        Vec3::new(207.0, 345.0, 0.0),
        Vec3::new(233.0, 330.0, 0.0),
        Vec3::new(230.0, 360.0, 0.0),
        Vec3::new(250.0, 380.0, 0.0),
        Vec3::new(220.0, 385.0, 0.0),
        Vec3::new(205.0, 410.0, 0.0),
        Vec3::new(193.0, 383.0, 0.0),
    ];

    // Relleno del polígono por scanline.
    framebuffer.set_current_color(0xFFCC00);
    framebuffer.filled_polygon(&estrella);

    // Contorno del mismo polígono encima del relleno.
    framebuffer.set_current_color(0xFFFFFF);
    framebuffer.polygon(&estrella);

    let _ = framebuffer.render_buffer("output.bmp");

    println!("Framebuffer rendered to output.bmp");
}
