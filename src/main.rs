mod framebuffer;
mod maze;
mod player;
mod caster;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::Vec2;
use std::f32::consts::PI;
use std::time::Duration;

use crate::caster::cast_ray;
use crate::framebuffer::Framebuffer;
use crate::maze::{load_maze, Maze};
use crate::player::{process_events, Player};

const BLOCK_SIZE: usize = 100;

/// Dibuja una celda del laberinto como un bloque relleno. Los espacios son
/// piso y no se dibujan.
fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, cell: char) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(0xFFDDDD);

    for x in xo..xo + BLOCK_SIZE {
        for y in yo..yo + BLOCK_SIZE {
            framebuffer.point(x, y);
        }
    }
}

/// Dibuja el mundo 2D: el laberinto, el jugador y el rayo que parte de él.
fn render(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player) {
    // dibuja el laberinto
    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            draw_cell(framebuffer, col * BLOCK_SIZE, row * BLOCK_SIZE, cell);
        }
    }

    // dibuja al jugador como un pequeño marcador
    framebuffer.set_current_color(0xFFFF00);
    let px = player.pos.x as usize;
    let py = player.pos.y as usize;
    for x in px.saturating_sub(3)..=px + 3 {
        for y in py.saturating_sub(3)..=py + 3 {
            framebuffer.point(x, y);
        }
    }

    // lanza un rayo en la dirección de vista del jugador
    cast_ray(framebuffer, maze, player, player.a, BLOCK_SIZE);
}

fn main() {
    let window_width = 1300;
    let window_height = 900;
    let framebuffer_width = 1300;
    let framebuffer_height = 900;
    let frame_delay = Duration::from_millis(16);

    let maze = load_maze("./maze.txt");

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    framebuffer.set_background_color(0x333355);

    let mut window = Window::new(
        "Maze Runner",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut player = Player {
        pos: Vec2::new(150.0, 150.0),
        a: PI / 3.0,
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        process_events(&window, &mut player);

        framebuffer.clear();

        render(&mut framebuffer, &maze, &player);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
