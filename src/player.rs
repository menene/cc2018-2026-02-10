use nalgebra_glm::Vec2;

use crate::maze::Maze;

pub struct Player {
    pub pos: Vec2,
}

impl Player {
    pub fn from_maze(maze: &mut Maze, block_size: usize) -> Player {
        for row in 0..maze.len() {
            for col in 0..maze[row].len() {
                if maze[row][col] == 'p' {
                    maze[row][col] = ' ';

                    let x = col * block_size + block_size / 2;
                    let y = row * block_size + block_size / 2;

                    return Player {
                        pos: Vec2::new(x as f32, y as f32),
                    };
                }
            }
        }

        Player {
            pos: Vec2::new(0.0, 0.0),
        }
    }
}
