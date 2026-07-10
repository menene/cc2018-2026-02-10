# 01 — Software Rendering: Point

Primera etapa de gráficas del curso **cc2018 – Gráficas por Computadora** (UVG). Se implementan las bases del renderizado por software: un framebuffer y la escritura de imágenes BMP.

## Objetivo

- Comprender las bases de las gráficas por computadora.
- Implementar una clase `Framebuffer` en Rust.
- Implementar una clase BMP para escribir archivos BMP.
- Dibujar puntos y exportar el resultado como archivo BMP.

## Estructura

```
.
├── Cargo.toml          # Manifiesto del proyecto
├── Cargo.lock          # Versiones exactas de las dependencias
└── src
    ├── main.rs         # Punto de entrada; dibuja los puntos
    ├── framebuffer.rs  # Buffer de píxeles en memoria
    └── bmp.rs          # Escritura del framebuffer a un archivo BMP
```

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 01-SR-01-POINT
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Revisar el archivo `output.bmp` en el directorio del proyecto para ver los puntos dibujados.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [Framebuffer](https://en.wikipedia.org/wiki/Framebuffer)
- [BMP File Format](https://en.wikipedia.org/wiki/BMP_file_format)
- [Raster Graphics](https://en.wikipedia.org/wiki/Raster_graphics)
