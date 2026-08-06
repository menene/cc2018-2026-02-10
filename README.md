# cc2018 — Gráficas por Computadora

Repositorio del curso **cc2018 – Gráficas por Computadora** de la Universidad del Valle de Guatemala (UVG), semestre que inicia en **julio de 2026**.

Cada tema, laboratorio y punto de entrega del curso vive en su propia **rama de git**. `main` contiene esta descripción general; haz checkout de cualquier rama para obtener el código de esa etapa.

> **Lenguaje:** [Rust](https://www.rust-lang.org/) 🦀
> El renderizado se construye desde cero — un `Framebuffer` propio, exportación a BMP para las etapas iniciales, y una ventana con render loop en tiempo real para las etapas interactivas.

---

## Sobre el curso

El curso construye un stack de gráficas desde cero, en cuatro fases, cada una culminando en un proyecto:

1. **Software Rendering (SR)** — framebuffer, primitivas (puntos, líneas, polígonos, triángulos), relleno, ventanas y el main render loop.
2. **Raycasting (RC)** — un laberinto 2D cargado desde texto, un campo de visión en primera persona, texturas y sprites → **Proyecto 1**.
3. **Raytracing (RT)** — rayos 3D, materiales, iluminación, sombras, reflexiones y refracciones → **Proyecto 2**.
4. **Render Pipeline (RP)** — un pipeline completo de modelo → vista → proyección → viewport, modelos OBJ, flat shading, cámaras y fragment shaders → **Proyecto 3**.

Las ramas están ordenadas según la **secuencia del curso 2026** (SR → RC → RT → RP), que *no* es el orden alfabético de los prefijos.

---

## Cómo correr cualquier etapa

```bash
# Clonar
git clone https://github.com/menene/cc2018-2026-02-10.git
cd cc2018-2026-02-10

# Elegir una etapa
git checkout 04-SR-04-FILLED-POLYGON

# Compilar y correr
cargo run
```

Las primeras etapas de Software Rendering escriben una imagen (por ejemplo `output.bmp`) en el directorio del proyecto; a partir de `05-SR-05-WINDOWS` se abre una ventana con render loop en vivo.

---

## Índice de ramas

### Fase 0 · Preparación
| Rama | Descripción |
|------|-------------|
| `00-HELLO-WORLD` | Hello world en Rust; setup del entorno y de Cargo |

### Fase 1 · Software Rendering
| Rama | Descripción |
|------|-------------|
| `01-SR-01-POINT` | `Framebuffer` + escritor BMP; dibujar puntos |
| `02-SR-02-LINE` | Dibujo de líneas |
| `03-SR-03-POLYGON` | Contornos de polígonos a partir de vértices |
| `04-SR-04-FILLED-POLYGON` | Migración a vectores (`Vec3` / nalgebra-glm) y relleno de polígonos por scanline |
| `05-SR-05-WINDOWS` | Ventana en tiempo real |
| `06-SR-06-RENDER-LOOP` | Main render loop |

### Fase 2 · Raycasting
| Rama | Descripción |
|------|-------------|
| `07-RC-01-MAZE-LOADER` | Cargar laberinto desde archivo de texto; mundo 2D y cast ray |
| `08-RC-02-MAZE-PLAYER` | Estructura del jugador y un rayo estático en su dirección de vista |
| `09-RC-03-MAZE-MOVEMENT` | Control del jugador por teclado y abanico de rayos del campo de visión |
| `10-RC-04-MAZE-EVENTS` | Eventos de entrada, texturas y sprites |

### Fase 3 · Raytracing
| Rama | Descripción |
|------|-------------|
| `11-RT-01-RAYS` | Rayos 3D y objetos 3D |
| `12-RT-02-MATERIALS` | Sistema de materiales |
| `13-RT-03-ORBIT-CAMERA` | Cámara orbital |
| `14-RT-04-LIGHT` | Iluminación difusa y especular |
| `15-RT-05-SHADOW` | Sombras |
| `16-RT-06-REFLECTIONS` | Reflexiones |
| `17-RT-07-REFRACTIONS` | Refracciones |

### Fase 4 · Render Pipeline
| Rama | Descripción |
|------|-------------|
| `18-RP-01-3D-MODELS` | Carga de OBJ y vertex arrays |
| `19-RP-02-FLAT-SHADING` | Flat shading |
| `20-RP-03-VALUE-INTERPOLATION` | Coordenadas baricéntricas y relleno de triángulos |
| `21-RP-04-MATRIX-TRANSFORMATIONS` | Matrices de modelo / vista / proyección / viewport |
| `22-RP-05-ORBIT-CAMERA` | Movimiento de cámara y de personaje |
| `23-RP-06-FRAGMENT-SHADER` | Fragment shaders |
| `24-RP-07-ANIMATED-FRAGMENT-SHADER` | Shaders animados en el tiempo |
| `25-RP-08-COMBINED-FRAGMENT-SHADER` | Shaders combinados / múltiples objetos |
| `26-RP-09-NOISE-FRAGMENT-SHADER` | Shaders basados en ruido (warp speed) |

---

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [Framebuffer](https://en.wikipedia.org/wiki/Framebuffer)
- [BMP File Format](https://en.wikipedia.org/wiki/BMP_file_format)
- [Raster Graphics](https://en.wikipedia.org/wiki/Raster_graphics)
- [Ray casting](https://en.wikipedia.org/wiki/Ray_casting)
- [Ray tracing](https://en.wikipedia.org/wiki/Ray_tracing_(graphics))

