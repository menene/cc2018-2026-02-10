# 11 — Raycasting: Texturas

Quinta etapa de la fase de **Raycasting** del curso **cc2018 – Gráficas por Computadora** (UVG). En la etapa anterior cada columna de la pantalla se pintaba de un solo color, elegido según el carácter de la celda contra la que chocó el rayo. Aquí ese color deja de ser fijo: se lee de una **imagen** cargada desde el disco. Para lograrlo, el rayo debe reportar no solo *contra qué* chocó sino *en qué punto exacto* de la pared lo hizo.

## Objetivo

- Decodificar imágenes PNG y guardarlas en memoria en el mismo formato del `Framebuffer`.
- Calcular la coordenada horizontal de textura a partir del punto de impacto del rayo.
- Distinguir contra cuál de las caras de la celda chocó el rayo.
- Orientar la textura de modo que no salga espejeada según la dirección de vista.
- Calcular la coordenada vertical contra la estaca completa y no contra el pedazo visible.

## Controles

| Tecla | Acción |
| ----- | ------ |
| `W` | Avanzar en la dirección de vista |
| `S` | Retroceder |
| `A` | Girar a la izquierda |
| `D` | Girar a la derecha |
| `M` | Cambiar entre la vista 2D y la vista 3D |
| `T` | Encender o apagar las texturas |
| `F` | Encender o apagar la corrección de ojo de pez |
| `P` | Encender o apagar el reporte en consola |
| `Escape` | Salir |

La tecla `T` alterna entre las paredes de color plano de la etapa anterior y las paredes con textura, que es la forma más directa de ver qué agregó esta etapa.

## Cargar las texturas

Decodificar PNG a mano no es el tema de esta etapa, así que se agrega la dependencia [`image`](https://docs.rs/image/) limitada al formato PNG:

```toml
image = { version = "0.25", default-features = false, features = ["png"] }
```

Lo que sí importa es **cómo** se guarda lo decodificado. El `Framebuffer` trabaja con colores empacados en un `u32` como `0xRRGGBB`, así que la textura se convierte a ese formato una sola vez al cargarla:

```rust
let pixels = image
    .pixels()
    .map(|p| ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | p[2] as u32)
    .collect();
```

Muestrear la textura durante el render queda entonces reducido a un acceso a un `Vec<u32>`, sin ninguna conversión de color. Esto importa porque el muestreo ocurre una vez **por píxel de pared**: con estacas de varios cientos de píxeles en 1300 columnas, son cientos de miles de muestreos por cuadro.

Las imágenes se leen del disco una sola vez, antes de que arranque el ciclo de render. El `TextureManager` guarda un `HashMap<char, Texture>` que asocia cada carácter del laberinto con su imagen, más una textura de respaldo para cualquier carácter no contemplado.

El muestreo usa **coordenadas normalizadas**: `u` y `v` van de 0 a 1 y solo al final se multiplican por el tamaño real de la imagen. Así el resto del programa nunca necesita saber que las texturas miden 128×128, y cambiar una por otra de distinta resolución no obliga a tocar nada más.

## Dónde pegó el rayo

Hasta ahora el rayo devolvía la distancia y el carácter de la celda. Para texturizar hace falta un dato más: **en qué punto a lo ancho de la pared** ocurrió el impacto, un valor de 0 a 1 que se llama `u`.

El rayo avanza de un píxel a la vez y se detiene en cuanto entra a una celda que no es piso. En ese momento el punto de impacto ya está *dentro* de la celda, pero apenas cruzando una de sus caras. Restándole el origen de la celda se obtienen dos coordenadas locales:

```
hit_x = x − columna · BLOCK_SIZE
hit_y = y − fila   · BLOCK_SIZE
```

Una de las dos queda pegada a una orilla de la celda (vale casi 0 o casi `BLOCK_SIZE`) y la otra puede valer cualquier cosa. **La que está pegada a la orilla dice qué cara se cruzó; la otra es la que recorre la pared** y por lo tanto es la que sirve de coordenada de textura. Comparar qué tan cerca está cada una de su orilla más próxima decide el caso:

```
orilla_x = min(hit_x, BLOCK_SIZE − hit_x)
orilla_y = min(hit_y, BLOCK_SIZE − hit_y)
```

Si `orilla_x` es la menor, se cruzó una cara **vertical** (la izquierda o la derecha de la celda) y la textura se recorre con `hit_y`. Si no, se cruzó una cara **horizontal** y se recorre con `hit_x`.

## El espejo

Saber cuál coordenada usar no basta: falta decidir **hacia dónde crece**. Si se toma `u = hit_y / BLOCK_SIZE` siempre, las dos caras verticales de una misma celda salen espejeadas entre sí, y una textura con cualquier detalle asimétrico lo delata de inmediato.

La regla es que `u` debe crecer hacia la **derecha del jugador**, porque las columnas de la pantalla también se recorren de izquierda a derecha. Visto el mapa desde arriba, con `x` hacia la derecha y `y` hacia abajo, alguien que mira al este tiene el sur a su derecha, y alguien que mira al sur tiene el oeste a su derecha. De ahí salen los dos casos:

| Cara | Dirección de vista | Coordenada |
| ---- | ------------------ | ---------- |
| Vertical | `cos(ángulo) > 0` (al este) | `u = hit_y / BLOCK_SIZE` |
| Vertical | `cos(ángulo) < 0` (al oeste) | `u = 1 − hit_y / BLOCK_SIZE` |
| Horizontal | `sin(ángulo) > 0` (al sur) | `u = 1 − hit_x / BLOCK_SIZE` |
| Horizontal | `sin(ángulo) < 0` (al norte) | `u = hit_x / BLOCK_SIZE` |

El reporte de consola (`P`) imprime, para cinco columnas repartidas a lo ancho de la pantalla, contra qué cara chocó el rayo y qué valor de `u` resultó. Recorriendo una pared de lado a lado se puede comprobar que `u` sube de forma continua de 0 a 1 y que no salta ni se invierte al cambiar de celda.

Como el rayo avanza de píxel en píxel, el punto de impacto tiene una precisión de aproximadamente un píxel. En una celda de 100 píxeles con una textura de 128 eso es poco más de un téxel de error, invisible en la práctica; solo cerca de las esquinas, donde las dos coordenadas quedan igual de pegadas a sus orillas, la elección de cara puede irse por la que no es.

## La coordenada vertical

La coordenada `v` recorre la estaca de arriba hacia abajo. El detalle está en **contra qué** se mide.

La estaca se calcula centrada en la pantalla, y cuando la pared está muy cerca sus extremos se salen de ella. Si `v` se midiera contra el pedazo visible, una pared cercana mostraría la textura entera comprimida dentro de la pantalla en lugar de mostrar únicamente el pedazo que le toca, y la textura parecería encogerse al acercarse en vez de crecer. Por eso `v` se mide siempre contra los extremos **sin recortar**:

```
v = (y − extremo_superior_sin_recortar) / altura_de_la_estaca
```

El recorte se aplica solo al rango de píxeles que efectivamente se dibujan. De este modo acercarse a una pared amplía la textura de forma continua, que es lo que se espera.

## Cielo y piso

Con las paredes texturizadas, el fondo uniforme de la etapa anterior dejaba la escena sin punto de apoyo. Antes de lanzar los rayos se pinta la mitad superior de la pantalla de color de cielo y la inferior de color de piso. Son dos colores planos, sin textura ni perspectiva: alcanzan para separar la escena y no cuestan nada.

## Estructura

```
.
├── Cargo.toml          # Manifiesto del proyecto (minifb, nalgebra-glm, image)
├── Cargo.lock          # Versiones exactas de las dependencias
├── maze.txt            # Definición del laberinto en texto
├── assets              # Texturas de las paredes en PNG
│   └── wall1..5.png
└── src
    ├── main.rs         # Punto de entrada; ciclo de render, entrada y las dos vistas
    ├── framebuffer.rs  # Buffer de píxeles en memoria
    ├── maze.rs         # Carga del laberinto y estado inicial del jugador
    ├── player.rs       # Estado del jugador, lectura del teclado y colisiones
    ├── caster.rs       # Lanzamiento de un rayo; distancia, impacto y coordenada de textura
    └── textures.rs     # Carga de las imágenes y muestreo de color
```

La vista 2D sigue usando colores planos por carácter. Es un mapa, no una escena: los colores distinguen las celdas de un vistazo mejor que las texturas.

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 11-RC-05-MAZE-TEXTURES
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Se abre una ventana con el laberinto texturizado. Caminar con `W`/`A`/`S`/`D` y observar cómo la textura se amplía al acercarse a una pared y cómo se desliza a lo largo de ella al caminar de lado. Con `T` se apagan las texturas para comparar contra la etapa anterior, y con `P` se puede seguir en consola la cara y la coordenada `u` de cada rayo. Cerrar con `Escape` o con el botón de cerrar de la ventana.

El programa busca las texturas en `assets/`, con rutas relativas al directorio desde el que se ejecuta; hay que correrlo desde la raíz del proyecto.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [minifb](https://docs.rs/minifb/)
- [image](https://docs.rs/image/)
- [Raycasting](https://en.wikipedia.org/wiki/Ray_casting)
- [Lode's Computer Graphics Tutorial — Textured Raycasting](https://lodev.org/cgtutor/raycasting2.html)
- [Texture mapping](https://en.wikipedia.org/wiki/Texture_mapping)
