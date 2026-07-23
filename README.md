# 05 — Software Rendering: Windows

Quinta etapa de gráficas del curso **cc2018 – Gráficas por Computadora** (UVG). El renderizado deja de exportarse a un archivo BMP y pasa a mostrarse en una **ventana en tiempo real** con `minifb`. Esta es la base sobre la que se construyen las etapas interactivas posteriores.

## Objetivo

- Abrir una ventana del sistema y presentar el framebuffer en pantalla en lugar de escribir a disco.
- Introducir el ciclo de ventana que mantiene la ventana abierta y responde a eventos (cerrar con la tecla `Escape` o el botón de cerrar).
- Separar la resolución del framebuffer de la resolución de la ventana; `minifb` escala el buffer interno al tamaño de la ventana.

## Estructura

```
.
├── Cargo.toml          # Manifiesto del proyecto (incluye minifb)
├── Cargo.lock          # Versiones exactas de las dependencias
└── src
    ├── main.rs         # Punto de entrada; abre la ventana y el ciclo
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
    git checkout 05-SR-05-WINDOWS
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Se abre una ventana con el polígono dibujado. Cerrar con `Escape` o con el botón de cerrar de la ventana.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [minifb](https://docs.rs/minifb/)
- [nalgebra-glm](https://docs.rs/nalgebra-glm/)
- [Framebuffer](https://en.wikipedia.org/wiki/Framebuffer)
- [Event loop](https://en.wikipedia.org/wiki/Event_loop)
