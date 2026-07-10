# 02 — Software Rendering: Line

Segunda etapa de gráficas del curso **cc2018 – Gráficas por Computadora** (UVG). Se implementa el trazado de líneas con el algoritmo de Bresenham sobre el framebuffer de la etapa anterior.

## Objetivo

- Comprender e implementar el algoritmo de Bresenham para trazar líneas.
- Integrar el trazado de líneas con las clases `Framebuffer` y de escritura BMP existentes.
- Dibujar líneas y exportar el resultado como archivo BMP.

## Estructura

```
.
├── Cargo.toml          # Manifiesto del proyecto
├── Cargo.lock          # Versiones exactas de las dependencias
└── src
    ├── main.rs         # Punto de entrada; dibuja las líneas
    ├── framebuffer.rs  # Buffer de píxeles en memoria
    ├── line.rs         # Algoritmo de Bresenham
    └── bmp.rs          # Escritura del framebuffer a un archivo BMP
```

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 02-SR-02-LINE
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Revisar el archivo `output.bmp` en el directorio del proyecto para ver las líneas dibujadas.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [Framebuffer](https://en.wikipedia.org/wiki/Framebuffer)
- [BMP File Format](https://en.wikipedia.org/wiki/BMP_file_format)
- [Raster Graphics](https://en.wikipedia.org/wiki/Raster_graphics)
- [Bresenham's Line Algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
