# 07 — Raycasting: Cargador de Laberinto

Primera etapa de la fase de **Raycasting** del curso **cc2018 – Gráficas por Computadora** (UVG). Se construye el mundo 2D sobre el que operará el motor: un laberinto se **carga desde un archivo de texto**, se dibuja como una rejilla de bloques en el framebuffer, y desde la posición del jugador se **lanza un rayo** que avanza hasta chocar con una pared. Es el cimiento de la vista en primera persona de las etapas siguientes.

## Objetivo

- Cargar un laberinto desde un archivo de texto (`maze.txt`) hacia una matriz de caracteres.
- Dibujar el mundo 2D como una rejilla de bloques, tratando cada carácter distinto de un espacio como pared.
- Lanzar un rayo desde el jugador en la dirección de su ángulo de vista y detenerlo al encontrar una pared.

## El laberinto

El archivo `maze.txt` describe el laberinto con caracteres: los espacios (` `) son piso transitable y cualquier otro carácter (`+`, `-`, `|`) es pared. La letra `g` marca la meta.

## Estructura

```
.
├── Cargo.toml          # Manifiesto del proyecto (minifb, nalgebra-glm)
├── Cargo.lock          # Versiones exactas de las dependencias
├── maze.txt            # Definición del laberinto en texto
└── src
    ├── main.rs         # Punto de entrada; carga, ciclo de render y dibujo del mundo
    ├── framebuffer.rs  # Buffer de píxeles en memoria
    ├── maze.rs         # Carga del laberinto desde archivo de texto
    ├── player.rs       # Estado del jugador (posición y ángulo de vista)
    └── caster.rs       # Lanzamiento de un rayo sobre el laberinto
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

3. Se abre una ventana con el laberinto dibujado, el jugador y el rayo que parte de él. Cerrar con `Escape` o con el botón de cerrar de la ventana.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [minifb](https://docs.rs/minifb/)
- [Raycasting](https://en.wikipedia.org/wiki/Ray_casting)
- [Lode's Computer Graphics Tutorial — Raycasting](https://lodev.org/cgtutor/raycasting.html)
