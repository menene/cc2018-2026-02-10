# 13 — Raytracing: Rayos

Primera etapa de la fase de **Raytracing** del curso **cc2018 – Gráficas por Computadora** (UVG). Aquí cambia el tema. Las seis etapas anteriores construyeron un raycaster: un mundo hecho de celdas de una retícula, un rayo por columna de la pantalla y una pared vertical dibujada al final de cada rayo. De ahí se conserva una sola idea —**preguntarle a un rayo qué encuentra**— y todo lo demás se reemplaza.

## Objetivo

- Generar un rayo por cada píxel de la pantalla, no uno por columna.
- Definir la cámara, el plano de proyección y el mapeo de píxel a dirección.
- Resolver analíticamente la intersección entre un rayo y una esfera.
- Describir los objetos de la escena con un trait común.

## Controles

| Tecla | Acción |
| ----- | ------ |
| `Escape` | Salir |

## Qué cambia respecto del raycaster

| | Raycasting (etapas 07–12) | Raytracing (etapas 13 en adelante) |
| --- | --- | --- |
| Rayos | Uno por columna: 800 | Uno por píxel: 800 × 600 = 480 000 |
| Mundo | Celdas de una retícula, todas iguales | Objetos con geometría propia |
| Intersección | Avanzar de a poco hasta caer en una celda ocupada | Resolver una ecuación |
| Resultado del rayo | Una estaca vertical | Un píxel |
| Cámara | Siempre a la altura de los ojos, gira en un solo eje | Un punto en el espacio, ve en cualquier dirección |

La diferencia de fondo es la segunda fila. El raycaster podía dar pasos pequeños y preguntar «¿ya estoy dentro de una pared?» porque el mundo era una retícula y esa pregunta se contesta con un índice. Una esfera no tiene celdas: hay que resolver **dónde** la recta del rayo cruza la superficie, y eso es álgebra, no búsqueda.

## De un píxel a un rayo

La cámara está en el origen y ve hacia **−Z**. A una unidad de distancia se coloca un **plano de proyección**, una ventana rectangular por la que se mira la escena. Cada píxel de la pantalla es un punto de ese plano, y el rayo de ese píxel es la recta que va de la cámara hacia él.

El píxel `(x, y)` se lleva primero al rango `-1..1`:

```
screen_x =  (2 · x) / ancho − 1
screen_y = −(2 · y) / alto  + 1
```

La `y` va con signo contrario porque el píxel 0 está **arriba** en el framebuffer, mientras que el eje Y del mundo crece **hacia arriba**. Sin ese cambio de signo la imagen sale de cabeza.

Ese rango es el mismo en ambos ejes, pero la ventana no es cuadrada: 800 × 600. Multiplicar `screen_x` por la **relación de aspecto** (ancho / alto) devuelve la proporción correcta; sin esa corrección las esferas salen ovaladas, estiradas a lo ancho.

Con eso, la dirección del rayo es el vector que va del origen al punto del plano, normalizado:

```
dirección = normalize(screen_x, screen_y, −1)
```

El `−1` es la distancia al plano de proyección, y es también lo que fija el **campo de visión**: con el plano a una unidad y el borde de la pantalla en ±1, el ángulo de apertura es de 90 grados. Acercar el plano abre el campo de visión, alejarlo lo cierra — el mismo efecto que un lente gran angular o un teleobjetivo. Es el equivalente de la constante `FOV` del raycaster, expresado como una distancia en lugar de un ángulo.

Normalizar no es opcional por costumbre: las cuentas de intersección de las etapas siguientes interpretan el parámetro `t` como una distancia, y eso solo es cierto si la dirección mide 1.

## La intersección rayo–esfera

Un punto del rayo se escribe `origen + t · dirección`, donde `t` es qué tan lejos se avanzó. Un punto de la esfera cumple `|punto − centro|² = radio²`. Sustituir lo primero en lo segundo deja una **ecuación cuadrática** en `t`:

```
a = dirección · dirección
b = 2 · (origen − centro) · dirección
c = (origen − centro) · (origen − centro) − radio²

a·t² + b·t + c = 0
```

No hace falta resolverla todavía. El **discriminante** `b² − 4ac` ya contesta la pregunta de esta etapa:

- negativo → la recta pasa de largo, no hay solución real;
- cero → la roza, tangente;
- positivo → la atraviesa, entra por un punto y sale por otro.

```rust
discriminant > 0.0
```

Esa línea es todo el objeto de esta etapa. Las etapas siguientes despejarán `t` para saber **dónde** fue el impacto, y de ahí saldrán la normal, el material, la luz y las sombras.

Vale la pena notar dos cosas que este criterio todavía no distingue:

- **No sabe qué está adelante.** `cast_ray` devuelve blanco con el primer objeto que toca y deja de buscar. Con dos esferas superpuestas no hay forma de saber cuál tapa a cuál, porque ambas se pintan del mismo color. Ordenar por distancia es el trabajo de la etapa siguiente.
- **No sabe qué está atrás.** Una esfera colocada detrás de la cámara también produce discriminante positivo: la **recta** la cruza, aunque el **rayo** vaya en sentido contrario. La corrección es exigir `t > 0`, y llega junto con el cálculo del punto de impacto.

## Un trait para los objetos

`RayIntersect` declara la única operación que la escena necesita de un objeto:

```rust
pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> bool;
}
```

`Sphere` lo implementa con la cuadrática de arriba. Un cubo, un plano o un triángulo lo implementarían con su propia ecuación, y `cast_ray` no cambiaría ni una línea: recorre objetos y pregunta. La esfera es el primer caso porque su ecuación es la más corta que existe, no porque el diseño dependa de ella.

## El costo

La imagen completa son 480 000 rayos, y cada rayo se prueba contra todos los objetos. Con dos esferas eso es cerca de un millón de intersecciones — barato, pero crece con el número de objetos y con la resolución.

Como la escena es estática y la cámara no se mueve, la imagen se calcula **una sola vez**, antes del ciclo de la ventana, y el ciclo se limita a volver a presentar el mismo buffer. Cuando la cámara empiece a moverse (etapa `15-RT-03-ORBIT-CAMERA`) habrá que volver a renderizar en cada cuadro, y ahí el costo por rayo empezará a importar.

## Estructura

```
.
├── Cargo.toml            # Manifiesto del proyecto (minifb, nalgebra-glm)
├── Cargo.lock            # Versiones exactas de las dependencias
└── src
    ├── main.rs           # Cámara, generación de rayos y ciclo de la ventana
    ├── framebuffer.rs    # Buffer de píxeles en memoria
    ├── ray_intersect.rs  # Trait común a todos los objetos de la escena
    └── sphere.rs         # Esfera e intersección rayo–esfera
```

El framebuffer es el mismo de las etapas anteriores, sin cambios: sigue siendo un `Vec<u32>` con un color actual y una operación `point`.

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 13-RT-01-RAYS
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Se abre una ventana con dos siluetas blancas sobre el fondo: la esfera grande al centro y la pequeña a la derecha, más lejos. Vale la pena mover los centros y los radios en `main.rs` para ver cómo cambian el tamaño y la posición, y cambiar la distancia al plano de proyección para ver cómo se abre y se cierra el campo de visión. Cerrar con `Escape` o con el botón de cerrar de la ventana.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [minifb](https://docs.rs/minifb/)
- [nalgebra-glm](https://docs.rs/nalgebra-glm/)
- [Ray tracing (graphics)](https://en.wikipedia.org/wiki/Ray_tracing_(graphics))
- [Line–sphere intersection](https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection)
- [Scratchapixel — Ray-Sphere Intersection](https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection.html)
- [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html)
