# 04 — Software Rendering: Filled Polygon

Cuarta etapa de gráficas del curso **cc2018 – Gráficas por Computadora** (UVG). Se rellena el interior de un polígono mediante un algoritmo de scanline. En esta etapa la geometría migra a vectores `Vec3` (con `nalgebra-glm`), base sobre la que se construyen las fases posteriores.

## Objetivo

- Migrar el trazado de líneas y polígonos a vectores `Vec3` de `nalgebra-glm`.
- Implementar el relleno de polígonos por scanline: para cada línea horizontal, calcular las intersecciones con las aristas y colorear los píxeles entre cada par (regla par/impar).
- Combinar relleno y contorno, y exportar el resultado como archivo BMP.

## Estructura

```
.
├── Cargo.toml          # Manifiesto del proyecto (incluye nalgebra-glm)
├── Cargo.lock          # Versiones exactas de las dependencias
└── src
    ├── main.rs         # Punto de entrada; rellena y dibuja el polígono
    ├── framebuffer.rs  # Buffer de píxeles en memoria
    ├── line.rs         # Algoritmo de Bresenham sobre Vec3
    ├── polygon.rs      # Contorno y relleno por scanline
    └── bmp.rs          # Escritura del framebuffer a un archivo BMP
```

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 04-SR-04-FILLED-POLYGON
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Revisar el archivo `output.bmp` en el directorio del proyecto para ver el polígono relleno.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [nalgebra-glm](https://docs.rs/nalgebra-glm/)
- [Framebuffer](https://en.wikipedia.org/wiki/Framebuffer)
- [BMP File Format](https://en.wikipedia.org/wiki/BMP_file_format)
- [Scanline Rendering](https://en.wikipedia.org/wiki/Scanline_rendering)
- [Even–odd rule](https://en.wikipedia.org/wiki/Even%E2%80%93odd_rule)
