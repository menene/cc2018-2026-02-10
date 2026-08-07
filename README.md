# 10 — Raycasting: Vista en Primera Persona

Cuarta etapa de la fase de **Raycasting** del curso **cc2018 – Gráficas por Computadora** (UVG). Hasta la etapa anterior el laberinto se veía desde arriba y el campo de visión era un abanico de cinco rayos dibujados sobre el piso. Aquí ese mismo abanico se convierte en la **vista en primera persona**: se lanza un rayo por cada columna de píxeles de la pantalla y cada uno se dibuja como una **estaca vertical** cuya altura depende de la distancia que recorrió. Se agrega también la **corrección del efecto de ojo de pez**, que se puede encender y apagar para comparar.

## Objetivo

- Devolver desde el lanzamiento de un rayo la distancia recorrida y el carácter contra el que chocó.
- Lanzar un rayo por cada columna de la pantalla y dibujar cada uno como una estaca vertical.
- Calcular la altura de la estaca a partir de la distancia y de la distancia al plano de proyección.
- Corregir el efecto de ojo de pez proyectando la distancia sobre la dirección de vista.
- Impedir que el jugador atraviese las paredes.

## Controles

| Tecla | Acción |
| ----- | ------ |
| `W` | Avanzar en la dirección de vista |
| `S` | Retroceder |
| `A` | Girar a la izquierda |
| `D` | Girar a la derecha |
| `M` | Cambiar entre la vista 2D y la vista 3D |
| `F` | Encender o apagar la corrección de ojo de pez |
| `Escape` | Salir |

El título de la ventana indica en todo momento qué vista está activa y si la corrección está encendida.

## De rayos a estacas

El lanzamiento de un rayo ahora devuelve un `Intersect` con dos datos: la **distancia** recorrida hasta chocar y el **carácter** de la celda contra la que chocó. La distancia define la altura de la pared y el carácter define su color, que es lo que mantiene distinguibles las paredes del laberinto.

En la vista 3D se lanza un rayo por cada columna de píxeles, con el mismo reparto de ángulos de la etapa anterior. Lo que cambia es qué se hace con el resultado: en lugar de pintar el recorrido del rayo sobre el piso, se dibuja una línea vertical en la columna que le corresponde.

La altura de esa estaca sale de una proporción. Una pared mide `BLOCK_SIZE` en el mundo; en pantalla se ve más alta mientras más cerca esté:

```
altura = (BLOCK_SIZE / distancia) * distancia_al_plano_de_proyección
```

La **distancia al plano de proyección** es la distancia a la que habría que poner una pantalla del ancho de la ventana para que abarque exactamente el campo de visión:

```
distancia_al_plano = (ancho / 2) / tan(FOV / 2)
```

Definirla así, y no como un número fijo, hace que cambiar el `FOV` siga produciendo una imagen consistente. La estaca se centra verticalmente en la pantalla y se recorta contra sus orillas, de modo que una pared muy cercana simplemente llena toda la columna.

## El efecto de ojo de pez

La distancia que devuelve el rayo se mide **a lo largo del rayo**, no a lo largo de la dirección de vista. Frente a una pared plana los rayos de las orillas del abanico llegan en diagonal y por lo tanto recorren más camino que el rayo del centro, aunque la pared esté igual de lejos. Como la altura depende de la distancia, esas estacas salen más bajas y la pared recta se ve **curveada hacia adentro**, como a través de un lente de ojo de pez.

La corrección consiste en proyectar la distancia sobre la dirección de vista, multiplicándola por el coseno de la diferencia entre el ángulo del rayo y el ángulo del jugador:

```
distancia_corregida = distancia * cos(ángulo_del_rayo - ángulo_del_jugador)
```

Frente a una pared plana a 250 píxeles, sin corregir la altura de las estacas varía unos 60 píxeles entre el centro y las orillas de la pantalla; con la corrección la variación es cero y la pared se ve recta. La tecla `F` permite alternar entre las dos versiones sin dejar de caminar, que es la forma más clara de ver la diferencia.

## Colisiones

En las etapas anteriores el jugador atravesaba las paredes. Ahora, antes de aceptar un movimiento, se revisa a qué celda del laberinto correspondería la nueva posición: si esa celda no es piso transitable, el movimiento se descarta.

La revisión no se hace sobre un solo punto sino sobre cuatro, separados del centro por el radio del jugador, de modo que el jugador se detenga antes de que su dibujo quede encajado en la pared. Los ejes se revisan por separado: primero el desplazamiento en `x` y después el desplazamiento en `y`. Así, al caminar en diagonal contra una pared, el eje bloqueado se descarta y el otro sigue avanzando, y el jugador se desliza a lo largo de la pared en lugar de quedarse pegado.

La celda de la meta cuenta como piso transitable. Si se tratara como pared, el jugador nunca podría pararse sobre ella y la condición de victoria jamás se cumpliría.

## Estructura

```
.
├── Cargo.toml          # Manifiesto del proyecto (minifb, nalgebra-glm)
├── Cargo.lock          # Versiones exactas de las dependencias
├── maze.txt            # Definición del laberinto en texto
└── src
    ├── main.rs         # Punto de entrada; ciclo de render, entrada y las dos vistas
    ├── framebuffer.rs  # Buffer de píxeles en memoria
    ├── maze.rs         # Carga del laberinto y estado inicial del jugador
    ├── player.rs       # Estado del jugador, lectura del teclado y colisiones
    └── caster.rs       # Lanzamiento de un rayo; devuelve distancia e impacto
```

Lanzar un rayo por columna son cientos de miles de operaciones por cuadro, así que el manifiesto activa optimizaciones (`opt-level = 3`) incluso en las compilaciones de desarrollo.

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 10-RC-04-MAZE-3D-VIEW
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Se abre una ventana con el laberinto visto desde adentro. Caminar con `W`/`A`/`S`/`D` y observar cómo las paredes crecen al acercarse y cómo el jugador se detiene al chocar contra ellas. Con `M` se cambia a la vista 2D para ver de dónde sale la imagen, y con `F` se apaga la corrección para ver las paredes curvearse. Cerrar con `Escape` o con el botón de cerrar de la ventana.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [minifb](https://docs.rs/minifb/)
- [Raycasting](https://en.wikipedia.org/wiki/Ray_casting)
- [Lode's Computer Graphics Tutorial — Raycasting](https://lodev.org/cgtutor/raycasting.html)
