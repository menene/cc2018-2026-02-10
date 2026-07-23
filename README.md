# 03 — Software Rendering: Polygon

Tercera etapa de gráficas del curso **cc2018 – Gráficas por Computadora** (UVG). Se dibujan contornos de polígonos a partir de una lista de vértices, reutilizando el algoritmo de líneas de la etapa anterior.

## Objetivo

- Representar un polígono como una secuencia ordenada de vértices.
- Trazar su contorno uniendo cada vértice con el siguiente y cerrando la figura del último al primero.
- Reutilizar el trazado de líneas y la escritura BMP existentes para exportar el resultado.

## Estructura

```
.
├── Cargo.toml          # Manifiesto del proyecto
├── Cargo.lock          # Versiones exactas de las dependencias
└── src
    ├── main.rs         # Punto de entrada; define y dibuja los polígonos
    ├── framebuffer.rs  # Buffer de píxeles en memoria
    ├── line.rs         # Algoritmo de Bresenham
    ├── polygon.rs      # Contorno de polígonos a partir de vértices
    └── bmp.rs          # Escritura del framebuffer a un archivo BMP
```

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 03-SR-03-POLYGON
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Revisar el archivo `output.bmp` en el directorio del proyecto para ver los polígonos dibujados.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [Framebuffer](https://en.wikipedia.org/wiki/Framebuffer)
- [BMP File Format](https://en.wikipedia.org/wiki/BMP_file_format)
- [Raster Graphics](https://en.wikipedia.org/wiki/Raster_graphics)
- [Polygon](https://en.wikipedia.org/wiki/Polygon)
