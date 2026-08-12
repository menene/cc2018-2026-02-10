# 12 — Raycasting: Sprites

Sexta etapa de la fase de **Raycasting** del curso **cc2018 – Gráficas por Computadora** (UVG). Hasta aquí el mundo estaba hecho solo de paredes, y una pared siempre está alineada a la retícula del laberinto: el rayo la encuentra y la dibuja en la columna que le toca. Un **sprite** no funciona así. No está alineado a nada, ningún rayo lo busca, y sin embargo tiene que aparecer del tamaño correcto, en la columna correcta y —lo más difícil— **detrás de las paredes que lo tapan**.

## Objetivo

- Colocar enemigos en el mundo desde el archivo del laberinto.
- Proyectar una posición del mundo a una columna de la pantalla.
- Escalar el sprite con la misma fórmula que las paredes.
- Recortar el sprite contra las paredes usando un buffer de profundidad.
- Dibujar con transparencia por color clave.
- Ordenar los sprites entre sí de atrás hacia adelante.

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

En la vista 2D los enemigos aparecen como puntos magenta sobre el mapa, lo que permite comparar dónde están con cómo se ven desde adentro.

## Enemigos en el laberinto

Los enemigos se colocan desde `maze.txt`, igual que el jugador. El carácter `e` marca una posición y deja la celda como **piso transitable**: un enemigo no es geometría, no detiene los rayos y no detiene al jugador.

```
+--+--+--+--+
|p          |
+  +--+  +  +
|  |  e  |  |
+  +  +--+--+
|  |     e  |
+  +--+--+  +
|  e     | g|
+--+--+--+--+
```

`load_maze` ya hacía exactamente esto con la `p` del jugador, así que agregar enemigos es extender ese mismo `match` con un caso más. Un enemigo queda descrito por su posición y por el carácter con el que el `TextureManager` encuentra su imagen — el mismo mapa de texturas de la etapa anterior, con una entrada nueva para `e`.

## De una posición a una columna

Una pared se dibuja en la columna del rayo que la encontró. Con un sprite hay que hacer el camino inverso: partir de su posición y calcular en qué columna cae.

El primer paso es el ángulo del enemigo visto desde el jugador, y qué tanto se desvía de la dirección de vista:

```
ángulo_del_enemigo = atan2(enemigo.y − jugador.y, enemigo.x − jugador.x)
desvío = ángulo_del_enemigo − ángulo_del_jugador
```

Esa resta necesita **normalizarse** a `-π..π`. Los dos ángulos crecen sin límite conforme el jugador gira, así que un enemigo que está justo enfrente puede dar una diferencia de casi una vuelta completa; sin normalizar, el sprite desaparecería según cuántas vueltas lleve dado el jugador.

Con el desvío ya normalizado, la columna sale de una interpolación lineal. Esto funciona porque los rayos se reparten **linealmente en el ángulo**: la columna `i` corresponde al ángulo `a − FOV/2 + FOV · i/(ancho−1)`, así que despejar `i` es despejar una recta:

```
columna = (desvío / FOV + 0.5) · (ancho − 1)
```

Antes de proyectar hay que descartar a los enemigos que están al costado o a la espalda. Pasado un cuarto de vuelta la relación entre ángulo y columna deja de tener sentido, y un enemigo detrás del jugador produciría una columna cualquiera dentro de la pantalla.

El tamaño usa la **misma fórmula que las estacas** de las paredes, con la misma distancia al plano de proyección:

```
tamaño = (BLOCK_SIZE / distancia) · distancia_al_plano_de_proyección
```

Usar la misma fórmula no es un detalle de estilo: es lo que hace que un enemigo y una pared que están a la misma distancia se vean del mismo alto. La distancia que entra ahí es la **perpendicular** a la dirección de vista, `distancia · cos(desvío)`, por la misma razón que en las paredes — si se usara la distancia en línea recta, el enemigo crecería al moverse hacia la orilla de la pantalla sin haberse acercado.

## El buffer de profundidad

Este es el problema central de la etapa. Los sprites se dibujan **después** de las paredes, así que por omisión quedan encima de ellas: un enemigo al otro lado de una pared se ve flotando sobre ella.

La información que hace falta ya se calculó y se tiró a la basura. Al dibujar las paredes, cada columna de la pantalla supo exactamente a qué distancia estaba su pared. Basta con **guardarla**:

```rust
let mut depth = vec![f32::INFINITY; framebuffer_width];
```

`render_world` anota en `depth[i]` la distancia de la pared de cada columna, e `INFINITY` donde no hubo pared. Después, al dibujar un sprite, cada una de sus columnas se compara contra ese valor:

```rust
if depth[x] <= projection.depth {
    continue;   // la pared está más cerca: el enemigo queda tapado
}
```

La comparación es **por columna**, no por sprite, y ahí está la gracia: un enemigo que asoma por la esquina de una pared se recorta verticalmente justo en la orilla, con unas columnas dibujadas y otras no. El reporte de consola (`P`) informa cuántas columnas de cada enemigo sobrevivieron la prueba, lo que permite ver la diferencia entre «tapado por pared», «fuera de pantalla» y «208 de 412 columnas visibles» sin depender del ojo.

Es una versión reducida del *z-buffer* que usan las tarjetas de video, con una diferencia: aquí basta un valor por columna porque las paredes son verticales y ocupan la columna entera. En la fase de Render Pipeline hará falta uno por píxel.

## Transparencia por color clave

El PNG del enemigo trae canal alfa, pero está **opaco de punta a punta**: los 16384 píxeles tienen alfa 255. El fondo no es «nada», es un magenta concreto, `0x980088`, que cubre 13232 de esos píxeles y que no aparece en ningún lado del dibujo.

La transparencia se decide entonces comparando el color:

```rust
if color == TRANSPARENT {
    continue;
}
```

Es la técnica del **color clave**, la que usaban los juegos de la época: se reserva un color que no exista en el arte y se acuerda que significa «no dibujar». Cuesta una comparación por píxel y no necesita que el formato de imagen soporte transparencia.

## Buscar la textura una vez, no un millón

Un enemigo cercano llega a ocupar la pantalla entera. Eso son más de un millón de píxeles, y cada uno necesita un color de la textura.

La versión directa pide ese color al `TextureManager` pasándole el carácter del enemigo, que internamente busca la textura en un `HashMap`. Funciona, pero esconde un costo: **una búsqueda con hash por píxel**. Con más de un millón de píxeles por cuadro entre paredes y sprites, esa búsqueda —unas decenas de nanosegundos— pasa a dominar el tiempo de render y lo vuelve además muy variable, porque depende de qué tan cerca esté el enemigo. El resultado se ve como parpadeo: el cuadro tarda siete milisegundos parado en un pasillo vacío y sesenta con un enemigo enfrente.

La corrección es mover la búsqueda fuera del ciclo. El carácter no cambia mientras se dibuja una estaca ni mientras se dibuja un sprite, así que la textura se pide **una vez** y después se muestrea sobre esa referencia:

```rust
let texture = texture_manager.get(enemy.texture_key);

for x in first_x..last_x {
    for y in first_y..last_y {
        let color = texture.sample(u, v);
        ...
    }
}
```

Medido en el peor caso —un enemigo llenando la pantalla— el dibujo de los sprites baja de 52.6 ms a 2.8 ms, y el de las paredes de 8.2 ms a 2.8 ms. Es la misma cantidad de píxeles y la misma imagen: lo único que cambió fue *dónde* está la búsqueda.

## El orden entre sprites

El buffer de profundidad resuelve qué tapan las paredes, pero no resuelve qué pasa entre dos enemigos que se traslapan: el segundo en dibujarse taparía al primero sin importar cuál está más cerca.

La solución es ordenarlos por distancia y dibujarlos **de atrás hacia adelante**, de modo que los cercanos se pinten encima de los lejanos. Es el algoritmo del pintor, y alcanza porque los sprites son pocos y siempre están de frente a la cámara.

## Un sprite siempre ve de frente

Vale la pena notar una limitación de este enfoque, porque se descubre de inmediato al jugar: **el enemigo se ve igual desde cualquier lado**. Se le puede dar la vuelta completa y nunca muestra la espalda.

No es un error. Un sprite es una imagen plana que se dibuja siempre alineada a la pantalla —un *billboard*—, así que gira junto con el jugador y le presenta siempre la misma cara. El programa solo tiene una imagen del enemigo y no hay nada en el código que dependa de hacia dónde está viendo, porque el enemigo ni siquiera tiene una dirección de vista.

Los juegos de la época lo resolvían guardando **ocho imágenes por enemigo**, una cada 45 grados, y eligiendo cuál dibujar según el ángulo entre la dirección del enemigo y la posición del jugador. La proyección, el escalado, la prueba de profundidad y la transparencia no cambian en nada: lo único que cambia es de cuál textura se muestrea. Para objetos simétricos —un barril, una lámpara, un objeto recogible— una sola imagen es de hecho la respuesta correcta.

## Estructura

```
.
├── Cargo.toml          # Manifiesto del proyecto (minifb, nalgebra-glm, image)
├── Cargo.lock          # Versiones exactas de las dependencias
├── maze.txt            # Laberinto, posición inicial del jugador y enemigos
├── assets              # Texturas de las paredes y del sprite
│   ├── wall1..5.png
│   └── sprite1.png
└── src
    ├── main.rs         # Ciclo de render, buffer de profundidad y dibujo de sprites
    ├── framebuffer.rs  # Buffer de píxeles en memoria
    ├── maze.rs         # Carga del laberinto, del jugador y de los enemigos
    ├── player.rs       # Estado del jugador, lectura del teclado y colisiones
    ├── enemy.rs        # Posición y textura de un sprite
    ├── caster.rs       # Lanzamiento de un rayo; distancia, impacto y coordenada de textura
    └── textures.rs     # Carga de las imágenes, muestreo y color de transparencia
```

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 12-RC-06-MAZE-SPRITES
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Se abre una ventana con el laberinto y tres enemigos. Caminar con `W`/`A`/`S`/`D` y observar cómo crecen al acercarse y cómo se recortan al asomarse por la esquina de una pared. Con `M` se puede ver en el mapa dónde están realmente, y con `P` seguir en consola cuántas columnas de cada uno sobreviven la prueba de profundidad. Cerrar con `Escape` o con el botón de cerrar de la ventana.

El programa busca las imágenes en `assets/`, con rutas relativas al directorio desde el que se ejecuta; hay que correrlo desde la raíz del proyecto.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [minifb](https://docs.rs/minifb/)
- [image](https://docs.rs/image/)
- [Lode's Computer Graphics Tutorial — Raycasting with sprites](https://lodev.org/cgtutor/raycasting3.html)
- [Z-buffering](https://en.wikipedia.org/wiki/Z-buffering)
- [Chroma key](https://en.wikipedia.org/wiki/Chroma_key)
- [Painter's algorithm](https://en.wikipedia.org/wiki/Painter%27s_algorithm)
