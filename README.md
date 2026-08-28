# 14 — Raytracing: Materiales

Segunda etapa de la fase de **Raytracing** del curso **cc2018 – Gráficas por Computadora** (UVG). La etapa anterior dejó dos siluetas blancas: el rayo solo sabía contestar «toco algo» o «no toco nada». Aquí esa respuesta se vuelve una descripción del impacto —dónde fue, a qué distancia, contra qué superficie— y con eso aparecen el color y el orden en profundidad.

## Objetivo

- Devolver la información completa del impacto en lugar de un `bool`.
- Quedarse con el objeto **más cercano**, no con el primero del arreglo.
- Descartar los objetos que están detrás de la cámara.
- Calcular la normal de la superficie en el punto de impacto.
- Describir el color de cada objeto con un material.

## Controles

| Tecla | Acción |
| ----- | ------ |
| `Escape` | Salir |

## De un `bool` a un impacto

La etapa 13 se detenía en el discriminante: positivo significa que la recta cruza la esfera, y eso alcanzaba para pintar una silueta. Para pintar un color hay que **despejar `t`**, porque el color depende de contra qué objeto se chocó, y saber cuál exige saber cuál está más cerca.

La cuadrática tiene dos soluciones. La del signo negativo es la menor:

```
t = (−b − √discriminante) / 2a
```

Geométricamente son los dos puntos donde el rayo atraviesa la esfera: por dónde **entra** y por dónde **sale**. El que interesa es el de entrada, porque es el que la cámara ve; el otro está del otro lado de la superficie, oculto por ella.

Con `t` en mano se calcula todo lo demás:

| Campo | Cómo sale | Para qué |
| --- | --- | --- |
| `distance` | es `t` | comparar objetos y quedarse con el más cercano |
| `point` | `origen + t · dirección` | punto exacto del impacto |
| `normal` | `normalize(point − centro)` | hacia dónde ve la superficie |
| `material` | el del objeto tocado | de qué color pintar el píxel |

`point` y `normal` todavía no cambian ni un píxel de la imagen —de ahí el `#[allow(dead_code)]` sobre la estructura— pero se calculan ahora porque salen gratis de la misma cuenta y son la base de la iluminación: la normal dice qué tanto de frente le llega la luz a la superficie, y el punto es desde dónde se lanzará el rayo de sombra.

### `Option` en lugar de una bandera

Una versión común de este mismo paso guarda una bandera `is_intersecting` dentro de la estructura, junto con un constructor `empty()` que rellena el punto con ceros y el material con un negro de mentira. Funciona, pero obliga a llevar un impacto que no ocurrió y a recordar consultar la bandera antes de leer los demás campos.

Rust ya tiene un tipo para «esto puede no existir»:

```rust
fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect>;
```

Si no hubo impacto no hay estructura que llenar, y el compilador no deja leer un campo sin antes abrir el `Option`. La bandera y el impacto vacío desaparecen.

## Lo que está atrás

El discriminante positivo dice que **la recta** cruza la esfera, no que el rayo lo haga. Una esfera colocada detrás de la cámara la cruza igual, con una `t` negativa: el punto de corte queda «hacia atrás» sobre la recta.

```rust
if t <= 0.0 {
    return None;
}
```

Esa comparación es la corrección que la etapa anterior dejó pendiente. Sin ella, mover una esfera de `z = -4` a `z = 4` no la quita de la pantalla — la deja exactamente donde estaba.

## El impacto más cercano

`cast_ray` ya no puede devolver el primer objeto que toca. El arreglo de objetos está en el orden en que se escribió en `main`, y ese orden no tiene nada que ver con cuál está adelante. Hay que probarlos todos y quedarse con el de menor distancia:

```rust
for object in objects {
    if let Some(intersect) = object.ray_intersect(ray_origin, ray_direction) {
        if closest.is_none_or(|distance| intersect.distance < distance) {
            closest = Some(intersect.distance);
            color = intersect.material.diffuse;
        }
    }
}
```

Es la misma idea del buffer de profundidad de los sprites (etapa `12-RC-06-MAZE-SPRITES`), con dos diferencias: aquí se resuelve por píxel y no por columna, y no hace falta guardar el buffer entero porque la decisión se toma dentro del mismo rayo.

La escena tiene una esfera azul que se traslapa con la de marfil y está más cerca de la cámara. Está puesta precisamente para que se note: si se quitara la comparación de distancias, la azul quedaría **debajo** de la de marfil por estar después en el arreglo.

## Un tipo para el color

El framebuffer guarda enteros de 32 bits con los tres canales empacados. Ese formato es cómodo para escribir un píxel y molesto para todo lo demás: sumar dos colores empacados suma los canales entre sí y el acarreo de uno se mete en el siguiente.

`Color` guarda los canales por separado y define las dos operaciones que la fase va a necesitar:

- **Suma** — es sumar luz. Usa `saturating_add`, así que dos luces intensas dan blanco en lugar de dar la vuelta a 0.
- **Multiplicación por un escalar** — es subir o bajar la intensidad. Recorta a `0..255` para que un factor mayor que 1 no desborde.

Ninguna de las dos se usa todavía; con un solo color difuso y sin luces no hay nada que mezclar. Aparecen en `16-RT-04-LIGHT`, donde el color de un píxel pasa a ser una suma de aportes multiplicados por intensidades. El empacado a entero ocurre una sola vez, al escribir el píxel, con `to_hex`.

## Materiales

Un `Material` es lo que la superficie hace con la luz. Por ahora tiene un solo campo, `diffuse`, que es el color que se ve:

```rust
let ivory = Material::new(Color::new(100, 100, 80));
let rubber = Material::new(Color::new(80, 0, 0));
let cobalt = Material::new(Color::new(40, 80, 140));
```

Separar el material del objeto no es ceremonia: tres esferas pueden compartir el mismo material, y en las etapas siguientes ese material crece con el brillo especular, el coeficiente de reflexión y el índice de refracción, sin que la esfera cambie.

## Lo que todavía falta

La imagen tiene color, pero sigue siendo plana: cada esfera es un **disco** de color uniforme, sin volumen. El color de un píxel depende de qué material se tocó y de nada más — no de la normal, no de dónde está la luz, porque todavía no hay luz.

Eso es lo que hace la etapa `16-RT-04-LIGHT`: multiplicar el color difuso por qué tanto de frente le llega la luz a la superficie, que es justamente lo que mide la normal ya calculada aquí. Antes, `15-RT-03-ORBIT-CAMERA` saca la cámara del origen para poder ver la escena desde cualquier lado.

## Estructura

```
.
├── Cargo.toml            # Manifiesto del proyecto (minifb, nalgebra-glm)
├── Cargo.lock            # Versiones exactas de las dependencias
└── src
    ├── main.rs           # Cámara, generación de rayos e impacto más cercano
    ├── framebuffer.rs    # Buffer de píxeles en memoria
    ├── color.rs          # Color por canales, suma y escalado
    ├── ray_intersect.rs  # Material, Intersect y el trait común a los objetos
    └── sphere.rs         # Esfera, solución de la cuadrática y normal
```

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 14-RT-02-MATERIALS
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Se abren tres esferas de colores distintos. Vale la pena intercambiar el orden de las esferas en el arreglo para comprobar que la imagen no cambia, mover la esfera azul a `z = 4` para verla desaparecer, y quedarse con el primer impacto en lugar del más cercano para verla hundirse detrás de la de marfil. Cerrar con `Escape` o con el botón de cerrar de la ventana.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [`Option` en el libro de Rust](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
- [minifb](https://docs.rs/minifb/)
- [nalgebra-glm](https://docs.rs/nalgebra-glm/)
- [Line–sphere intersection](https://en.wikipedia.org/wiki/Line%E2%80%93sphere_intersection)
- [Scratchapixel — Ray-Sphere Intersection](https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection.html)
- [Understandable RayTracing in 256 lines](https://github.com/ssloy/tinyraytracer/wiki)
