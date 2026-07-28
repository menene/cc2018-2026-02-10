use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;

/// Lanza un rayo desde el jugador en la dirección `a` (radianes) y avanza
/// dibujándolo hasta chocar con una pared o salir del laberinto.
pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    a: f32,
    block_size: usize,
) {
    let mut d = 0.0;

    framebuffer.set_current_color(0xFFDDDD);

    loop {
        let x = (player.pos.x + d * a.cos()) as usize;
        let y = (player.pos.y + d * a.sin()) as usize;

        // celda del laberinto que ocupa el punto actual (columna = x, fila = y)
        let i = x / block_size;
        let j = y / block_size;

        // fuera del laberinto: no hay más que recorrer
        if j >= maze.len() || i >= maze[j].len() {
            return;
        }

        // se encontró una pared
        if maze[j][i] != ' ' {
            return;
        }

        framebuffer.point(x, y);

        d += 1.0;
    }
}
