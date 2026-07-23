# 06 — Software Rendering: Render Loop

Sexta y última etapa de Software Rendering del curso **cc2018 – Gráficas por Computadora** (UVG). La imagen estática de la etapa anterior se convierte en **animación**: el dibujo pasa a ocurrir dentro del ciclo de render, de modo que la escena cambia en cada cuadro. Este es el ciclo principal sobre el que se construyen las fases interactivas posteriores.

## Objetivo

- Mover el dibujo al interior del ciclo: en cada iteración se actualiza el estado, se limpia el cuadro anterior y se vuelve a dibujar.
- Animar un objeto que rebota contra los bordes del framebuffer, invirtiendo su dirección y cambiando de color al tocarlos.
- Limitar la tasa de cuadros a ~60 FPS con una pausa por cuadro.

## Estructura

```
.
├── Cargo.toml          # Manifiesto del proyecto (minifb)
├── Cargo.lock          # Versiones exactas de las dependencias
└── src
    ├── main.rs         # Punto de entrada; ciclo de render y animación
    ├── framebuffer.rs  # Buffer de píxeles en memoria
    ├── line.rs         # Algoritmo de Bresenham sobre Vec3
    ├── polygon.rs      # Contorno y relleno por scanline
    └── bmp.rs          # Escritura del framebuffer a BMP (sin usar en esta etapa)
```

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 06-SR-06-RENDER-LOOP
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Se abre una ventana con el objeto animado rebotando en pantalla. Cerrar con `Escape` o con el botón de cerrar de la ventana.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [minifb](https://docs.rs/minifb/)
- [Framebuffer](https://en.wikipedia.org/wiki/Framebuffer)
- [Render loop / game loop](https://en.wikipedia.org/wiki/Video_game_programming#Game_structure)
- [Frame rate](https://en.wikipedia.org/wiki/Frame_rate)
