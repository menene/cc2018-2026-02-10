# 00 — Hello World

Primera etapa del curso **cc2018 – Gráficas por Computadora** (UVG). El objetivo es preparar el entorno de Rust y correr un programa mínimo antes de empezar con gráficas.

## Objetivo

- Instalar y verificar la toolchain de Rust (`rustc` y `cargo`).
- Entender la estructura de un proyecto de Cargo (`Cargo.toml` + `src/main.rs`).
- Compilar y ejecutar un "Hello, world!".

## Estructura

```
.
├── Cargo.toml      # Manifiesto del proyecto y dependencias
├── Cargo.lock      # Versiones exactas resueltas de las dependencias
└── src
    └── main.rs     # Punto de entrada del programa
```

## Cargo.toml y Cargo.lock

Cargo es el gestor de paquetes y build system de Rust. Estos dos archivos trabajan juntos:

- **`Cargo.toml`** — manifiesto escrito a mano. Declara nombre, versión, edición de Rust y dependencias. Aquí la sección `[dependencies]` está vacía porque el "Hello, world!" solo usa la librería estándar.

- **`Cargo.lock`** — generado por Cargo al compilar (no se edita a mano). Fija las versiones exactas de las dependencias para lograr builds reproducibles. Al ser un binario, se versiona en git.

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 00-HELLO-WORLD
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Salida esperada:
    ```
    Hello, world!
    ```
