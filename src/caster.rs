use crate::framebuffer::Framebuffer;
use crate::maze::Maze;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut d = 0.0;

    let cos_a = a.cos();
    let sin_a = a.sin();

    if draw_line {
        framebuffer.set_current_color(0xFFDDDD);
    }

    loop {
        let x = (player.pos.x + d * cos_a) as usize;
        let y = (player.pos.y + d * sin_a) as usize;

        let i = x / block_size;
        let j = y / block_size;

        if j >= maze.len() || i >= maze[j].len() {
            return Intersect {
                distance: d,
                impact: ' ',
            };
        }

        if maze[j][i] != ' ' {
            return Intersect {
                distance: d,
                impact: maze[j][i],
            };
        }

        if draw_line {
            framebuffer.point(x, y);
        }

        d += 1.0;
    }
}
