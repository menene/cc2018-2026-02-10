# 07 — Raycasting: Cargador de Laberinto

Primera etapa de la fase de **Raycasting** del curso **cc2018 – Gráficas por Computadora** (UVG). Se construye el mundo 2D sobre el que operará el motor: un laberinto se **carga desde un archivo de texto**, se dibuja como una rejilla de bloques en el framebuffer y se marca la posición inicial del jugador. Es el cimiento de la vista en primera persona de las etapas siguientes; el rayo aparece en la etapa que sigue.

## Objetivo

- Cargar un laberinto desde un archivo de texto (`maze.txt`) hacia una matriz de caracteres.
- Dibujar el mundo 2D como una rejilla de bloques, tratando cada carácter distinto de un espacio como pared.
- Ubicar al jugador dentro del laberinto y dibujarlo sobre la rejilla.

## El laberinto

El archivo `maze.txt` describe el laberinto con caracteres: los espacios (` `) son piso transitable y cualquier otro carácter (`+`, `-`, `|`) es pared. La letra `g` marca la meta.

La letra `p` marca dónde empieza el jugador. Al cargar el archivo esa celda se convierte en piso y su posición se guarda en píxeles, al centro del bloque que ocupaba:

```
x = col * block_size + block_size / 2
y = row * block_size + block_size / 2
```

De esa forma el laberinto se maneja en celdas mientras que el jugador se maneja en píxeles, que es la unidad en la que avanzará el rayo más adelante.

## Estructura

```
.
├── Cargo.toml          # Manifiesto del proyecto (minifb, nalgebra-glm)
├── Cargo.lock          # Versiones exactas de las dependencias
├── maze.txt            # Definición del laberinto en texto
└── src
    ├── main.rs         # Punto de entrada; carga, ciclo de render y dibujo del mundo
    ├── framebuffer.rs  # Buffer de píxeles en memoria
    ├── maze.rs         # Carga del laberinto y posición inicial del jugador
    └── player.rs       # Estado del jugador (posición)
```

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 07-RC-01-MAZE-LOADER
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Se abre una ventana con el laberinto dibujado y el jugador como un cuadro amarillo sobre la celda marcada con `p`. Cerrar con `Escape` o con el botón de cerrar de la ventana.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [minifb](https://docs.rs/minifb/)
- [Raycasting](https://en.wikipedia.org/wiki/Ray_casting)
- [Lode's Computer Graphics Tutorial — Raycasting](https://lodev.org/cgtutor/raycasting.html)
