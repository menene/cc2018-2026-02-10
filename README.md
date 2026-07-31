# 09 — Raycasting: Movimiento del Jugador

Tercera etapa de la fase de **Raycasting** del curso **cc2018 – Gráficas por Computadora** (UVG). En la etapa anterior el jugador aparecía en el laberinto con un rayo estático; ahora se agrega el **control del jugador**: el teclado modifica en cada cuadro su posición y su ángulo de vista, y el rayo que parte de él sigue esa dirección. Es la base interactiva sobre la que se construyen el campo de visión y la vista en primera persona.

## Objetivo

- Leer el teclado dentro del ciclo de render y actualizar el estado del jugador cuadro a cuadro.
- Avanzar y retroceder al jugador en la dirección de su ángulo de vista.
- Girar el ángulo de vista y confirmar que el rayo lanzado se reorienta con él.

## Controles

| Tecla | Acción |
| ----- | ------ |
| `W` | Avanzar en la dirección de vista |
| `S` | Retroceder |
| `A` | Girar a la izquierda |
| `D` | Girar a la derecha |
| `Escape` | Salir |

## Estructura

```
.
├── Cargo.toml          # Manifiesto del proyecto (minifb, nalgebra-glm)
├── Cargo.lock          # Versiones exactas de las dependencias
├── maze.txt            # Definición del laberinto en texto
└── src
    ├── main.rs         # Punto de entrada; ciclo de render, entrada y dibujo del mundo
    ├── framebuffer.rs  # Buffer de píxeles en memoria
    ├── maze.rs         # Carga del laberinto desde archivo de texto
    ├── player.rs       # Estado del jugador y lectura del teclado
    └── caster.rs       # Lanzamiento de un rayo sobre el laberinto
```

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 09-RC-03-MAZE-MOVEMENT
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Se abre una ventana con el laberinto y el jugador. Mover con `W`/`A`/`S`/`D` y observar cómo el rayo sigue la dirección de vista. Cerrar con `Escape` o con el botón de cerrar de la ventana.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [minifb](https://docs.rs/minifb/)
- [Raycasting](https://en.wikipedia.org/wiki/Ray_casting)
- [Lode's Computer Graphics Tutorial — Raycasting](https://lodev.org/cgtutor/raycasting.html)
