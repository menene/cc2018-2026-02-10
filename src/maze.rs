use std::fs::File;
use std::io::{BufRead, BufReader};

/// El laberinto es una matriz de caracteres: un espacio (' ') es piso
/// transitable y cualquier otro carácter se trata como pared.
pub type Maze = Vec<Vec<char>>;

/// Carga el laberinto desde un archivo de texto, una fila por línea.
pub fn load_maze(filename: &str) -> Maze {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}
