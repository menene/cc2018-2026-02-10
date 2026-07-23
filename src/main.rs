extern crate nalgebra_glm as glm;

use minifb::{Key, Window, WindowOptions};

use glm::Vec3;

mod framebuffer;
mod line;
mod polygon;

use crate::framebuffer::Framebuffer;
use crate::polygon::Polygon;

fn main() {
    // La ventana se muestra a mayor resolución que el framebuffer;
    // minifb escala el buffer interno al tamaño de la ventana.
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

    // La escena se dibuja una sola vez, antes del ciclo.
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

    // Ciclo de ventana: mantiene la ventana abierta y responde a eventos.
    // La imagen es estática; solo se vuelve a presentar el mismo buffer.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();
    }
}
