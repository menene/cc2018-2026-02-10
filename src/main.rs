use minifb::{Key, Window, WindowOptions};
use std::time::Duration;

mod framebuffer;

use crate::framebuffer::Framebuffer;

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 80;
    let framebuffer_height = 60;

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "UVG Graphics",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    framebuffer.set_background_color(0x000000);

    // Estado de la animación: posición y dirección del objeto.
    let mut x = 0_i32;
    let mut x2 = 1_i32;
    let mut x3 = 2_i32;
    let mut speed_x = 1_i32;

    let mut y = 40_i32;
    let mut speed_y = 1_i32;

    // Pausa entre cuadros para limitar la animación a ~60 FPS.
    let frame_delay = Duration::from_millis(16);

    framebuffer.set_current_color(0xFFFFFF);

    // Ciclo de render: cada iteración actualiza el estado, redibuja y presenta.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Rebote horizontal: al tocar un borde se invierte la dirección
        // y se cambia el color del objeto.
        if x3 as usize >= framebuffer_width {
            framebuffer.set_current_color(0xF17102);
            speed_x = -1;
        }
        if x <= 0 {
            framebuffer.set_current_color(0x00FF00);
            speed_x = 1;
        }

        x += speed_x;
        x2 += speed_x;
        x3 += speed_x;

        // Rebote vertical.
        if y as usize >= framebuffer_height {
            speed_y = -1;
        }
        if y <= 0 {
            speed_y = 1;
        }

        y += speed_y;

        // Limpiar el cuadro anterior y dibujar el objeto en su nueva posición.
        framebuffer.clear();
        framebuffer.point(x as usize, y as usize);
        framebuffer.point(x2 as usize, y as usize);
        framebuffer.point(x3 as usize, y as usize);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
