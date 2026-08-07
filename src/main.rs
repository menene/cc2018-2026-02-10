mod caster;
mod framebuffer;
mod maze;
mod player;

use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::f32::consts::PI;
use std::time::Duration;

use crate::caster::cast_ray;
use crate::framebuffer::Framebuffer;
use crate::maze::{load_maze, Maze};
use crate::player::{process_events, Player};

const BLOCK_SIZE: usize = 100;
const NUM_RAYS_2D: usize = 5;
const FOV: f32 = PI / 3.0;
const REPORT_EVERY: u64 = 60;
const REPORT_COLUMNS: [f32; 5] = [0.0, 0.25, 0.5, 0.75, 1.0];

fn cell_color(cell: char) -> u32 {
    match cell {
        '+' => 0x00AAFF,
        '-' => 0xFF5555,
        '|' => 0xFF5555,
        'g' | 'G' => 0x00FF00,
        _ => 0xFFDDDD,
    }
}

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, cell: char) {
    if cell == ' ' {
        return;
    }

    framebuffer.set_current_color(cell_color(cell));

    for x in xo..xo + BLOCK_SIZE {
        for y in yo..yo + BLOCK_SIZE {
            framebuffer.point(x, y);
        }
    }
}

fn ray_angle(player: &Player, i: usize, num_rays: usize) -> f32 {
    let ray_fraction = i as f32 / (num_rays - 1) as f32;
    player.a - FOV / 2.0 + FOV * ray_fraction
}

fn projection_plane_distance(width: usize) -> f32 {
    (width as f32 / 2.0) / (FOV / 2.0).tan()
}

fn render_2d(framebuffer: &mut Framebuffer, maze: &Maze, player: &Player) {
    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            draw_cell(framebuffer, col * BLOCK_SIZE, row * BLOCK_SIZE, cell);
        }
    }

    framebuffer.set_current_color(0xFFFF00);

    let px = player.pos.x as usize;
    let py = player.pos.y as usize;

    for x in px.saturating_sub(3)..=px + 3 {
        for y in py.saturating_sub(3)..=py + 3 {
            framebuffer.point(x, y);
        }
    }

    for i in 0..NUM_RAYS_2D {
        let angle = ray_angle(player, i, NUM_RAYS_2D);
        cast_ray(framebuffer, maze, player, angle, BLOCK_SIZE, true);
    }
}

fn render_world(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    fisheye_correction: bool,
) {
    let num_rays = framebuffer.width;
    let half_height = framebuffer.height as f32 / 2.0;
    let plane_distance = projection_plane_distance(framebuffer.width);

    for i in 0..num_rays {
        let angle = ray_angle(player, i, num_rays);
        let intersect = cast_ray(framebuffer, maze, player, angle, BLOCK_SIZE, false);

        if intersect.impact == ' ' {
            continue;
        }

        let distance = if fisheye_correction {
            intersect.distance * (angle - player.a).cos()
        } else {
            intersect.distance
        };

        let distance = distance.max(1.0);

        let stake_height = (BLOCK_SIZE as f32 / distance) * plane_distance;

        let stake_top = (half_height - stake_height / 2.0).max(0.0) as usize;
        let stake_bottom =
            (half_height + stake_height / 2.0).min(framebuffer.height as f32) as usize;

        framebuffer.set_current_color(cell_color(intersect.impact));

        for y in stake_top..stake_bottom {
            framebuffer.point(i, y);
        }
    }
}

fn print_report(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    fisheye_correction: bool,
) {
    let num_rays = framebuffer.width;
    let half_height = framebuffer.height as f32 / 2.0;
    let plane_distance = projection_plane_distance(framebuffer.width);

    println!(
        "\njugador ({:.0}, {:.0})   vista {:.1}°   ojo de pez: {}",
        player.pos.x,
        player.pos.y,
        player.a.to_degrees(),
        if fisheye_correction {
            "corregido"
        } else {
            "sin corregir"
        }
    );
    println!(
        "  columna    desvío   distancia   corregida    altura    arriba     abajo   pared"
    );

    for fraction in REPORT_COLUMNS {
        let i = ((num_rays - 1) as f32 * fraction) as usize;
        let angle = ray_angle(player, i, num_rays);
        let intersect = cast_ray(framebuffer, maze, player, angle, BLOCK_SIZE, false);

        let corrected = intersect.distance * (angle - player.a).cos();
        let distance = if fisheye_correction {
            corrected
        } else {
            intersect.distance
        };
        let stake_height = (BLOCK_SIZE as f32 / distance.max(1.0)) * plane_distance;

        let stake_top = (half_height - stake_height / 2.0).max(0.0) as usize;
        let stake_bottom =
            (half_height + stake_height / 2.0).min(framebuffer.height as f32) as usize;

        println!(
            "  {:>7}   {:>6.1}°   {:>9.1}   {:>9.1}   {:>7.1}   {:>7}   {:>7}   {}",
            i,
            (angle - player.a).to_degrees(),
            intersect.distance,
            corrected,
            stake_height,
            stake_top,
            stake_bottom,
            if intersect.impact == ' ' {
                '·'
            } else {
                intersect.impact
            }
        );
    }
}

fn main() {
    let window_width = 1300;
    let window_height = 900;
    let framebuffer_width = 1300;
    let framebuffer_height = 900;
    let frame_delay = Duration::from_millis(16);

    let (maze, mut player) = load_maze("./maze.txt", BLOCK_SIZE);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    framebuffer.set_background_color(0x333355);

    let mut window = Window::new(
        "Maze Runner",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    let mut mode_3d = true;
    let mut fisheye_correction = true;
    let mut report_enabled = true;
    let mut title_dirty = true;
    let mut frame: u64 = 0;

    println!(
        "altura = (BLOCK_SIZE / distancia) * distancia_al_plano_de_proyección\n\
         BLOCK_SIZE = {}   FOV = {:.1}°   distancia_al_plano = {:.1}",
        BLOCK_SIZE,
        FOV.to_degrees(),
        projection_plane_distance(framebuffer_width)
    );

    while window.is_open() && !window.is_key_down(Key::Escape) {
        frame += 1;

        process_events(&window, &mut player, &maze, BLOCK_SIZE);

        if window.is_key_pressed(Key::M, KeyRepeat::No) {
            mode_3d = !mode_3d;
            title_dirty = true;
        }

        if window.is_key_pressed(Key::F, KeyRepeat::No) {
            fisheye_correction = !fisheye_correction;
            title_dirty = true;
        }

        if window.is_key_pressed(Key::P, KeyRepeat::No) {
            report_enabled = !report_enabled;
            title_dirty = true;
        }

        if title_dirty {
            window.set_title(&format!(
                "Maze Runner — vista: {} (M) — ojo de pez: {} (F) — consola: {} (P)",
                if mode_3d { "3D" } else { "2D" },
                if fisheye_correction {
                    "corregido"
                } else {
                    "sin corregir"
                },
                if report_enabled { "activa" } else { "apagada" },
            ));
            title_dirty = false;
        }

        if report_enabled && frame % REPORT_EVERY == 0 {
            print_report(&mut framebuffer, &maze, &player, fisheye_correction);
        }

        let i = player.pos.x as usize / BLOCK_SIZE;
        let j = player.pos.y as usize / BLOCK_SIZE;
        if maze.get(j).and_then(|row| row.get(i)) == Some(&'g') {
            println!("¡Meta alcanzada! Fin del juego.");
            break;
        }

        framebuffer.clear();

        if mode_3d {
            render_world(&mut framebuffer, &maze, &player, fisheye_correction);
        } else {
            render_2d(&mut framebuffer, &maze, &player);
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
