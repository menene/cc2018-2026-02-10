# 08 — Raycasting: El Jugador y su Rayo

Segunda etapa de la fase de **Raycasting** del curso **cc2018 – Gráficas por Computadora** (UVG). Sobre el mundo 2D de la etapa anterior el jugador gana un **ángulo de vista** y aparece el módulo `caster`: desde la posición del jugador se lanza un **rayo** que avanza paso a paso en la dirección de ese ángulo y se detiene al chocar con una pared. El rayo todavía es fijo; el control por teclado llega en la etapa siguiente.

## Objetivo

- Agregar el ángulo de vista (`a`) al estado del jugador.
- Lanzar un rayo desde la posición del jugador y hacerlo avanzar en la dirección de su ángulo de vista.
- Detener el rayo al encontrar una celda que no sea un espacio, o al salir de los límites del laberinto.

## El rayo

El rayo avanza acumulando una distancia `d` a partir de la posición del jugador:

```
x = player.pos.x + d * cos(a)
y = player.pos.y + d * sin(a)
```

En cada paso la posición en píxeles se convierte a una celda del laberinto dividiendo entre el tamaño de bloque. Si esa celda no es un espacio, el rayo chocó con una pared y el ciclo termina; de lo contrario se pinta el píxel y `d` crece en uno. Esa distancia recorrida hasta el choque es la que más adelante determina la altura de las paredes en la vista en primera persona.

## Estructura

```
.
├── Cargo.toml          # Manifiesto del proyecto (minifb, nalgebra-glm)
├── Cargo.lock          # Versiones exactas de las dependencias
├── maze.txt            # Definición del laberinto en texto
└── src
    ├── main.rs         # Punto de entrada; ciclo de render y dibujo del mundo
    ├── framebuffer.rs  # Buffer de píxeles en memoria
    ├── maze.rs         # Carga del laberinto y estado inicial del jugador
    ├── player.rs       # Estado del jugador: posición y ángulo de vista
    └── caster.rs       # Lanzamiento de un rayo sobre el laberinto
```

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 08-RC-02-MAZE-PLAYER
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Se abre una ventana con el laberinto, el jugador dibujado como un cuadro amarillo y el rayo que sale de él hacia la pared más cercana. En esta etapa el jugador todavía no se mueve. Cerrar con `Escape` o con el botón de cerrar de la ventana.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [minifb](https://docs.rs/minifb/)
- [Raycasting](https://en.wikipedia.org/wiki/Ray_casting)
- [Lode's Computer Graphics Tutorial — Raycasting](https://lodev.org/cgtutor/raycasting.html)
