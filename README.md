# 17 — Raytracing: Sombras

Quinta etapa de la fase de **Raytracing** del curso **cc2018 – Gráficas por Computadora** (UVG). La etapa anterior iluminó cada objeto por separado: cada uno tiene su lado claro, su degradado y su punto de brillo. Pero los objetos no se enteran unos de otros — el cilindro está parado junto a la esfera y no le proyecta nada encima, porque nadie está preguntando si el camino hacia la luz está libre.

Esta etapa hace esa pregunta. Cuesta **un rayo más por píxel** y es lo que amarra los objetos entre sí: sin sombras la escena es un collage de figuras iluminadas; con sombras es un lugar.

## Objetivo

- Lanzar un **rayo de sombra** desde el punto de impacto hacia la luz.
- Anular la contribución de la luz cuando algo bloquea ese camino.
- Corregir el **acné de sombra** desplazando el origen del rayo.
- Agregar un piso a la escena para que las sombras tengan dónde caer.

## Controles

| Tecla | Acción |
| ----- | ------ |
| `←` | Orbitar hacia la izquierda |
| `→` | Orbitar hacia la derecha |
| `↑` | Subir sobre la escena |
| `↓` | Bajar bajo la escena |
| `Escape` | Salir |

## El rayo de sombra

Una sombra no es un objeto que haya que dibujar: es la **ausencia** de luz en un punto que sí está de frente a ella. Y saber si un punto recibe luz es exactamente el mismo problema que el motor ya sabe resolver desde la etapa 13 — lanzar un rayo y ver qué encuentra.

La diferencia está en la pregunta que se le hace al rayo. El rayo primario pregunta *«¿qué es lo primero que veo?»* y necesita el impacto **más cercano**. El rayo de sombra pregunta *«¿hay algo en el camino?»* y no necesita saber qué ni dónde: con que haya **uno** basta.

```rust
objects.iter().any(|object| {
    object
        .ray_intersect(&shadow_ray_origin, light_direction)
        .is_some_and(|blocker| blocker.distance < light_distance)
})
```

`any` corta en cuanto encuentra el primero, sin recorrer el resto. Esa es la razón de que las sombras salgan mucho más baratas que un segundo render completo, aunque en el peor caso —un punto iluminado, sin nada que lo bloquee— haya que revisar toda la escena igual.

## Detenerse en la luz

El detalle que sí importa es la comparación de distancias:

```rust
let light_distance = (light.position - intersect.point).magnitude();
```

Un objeto que está **más allá** de la luz no hace sombra. La luz no es un punto en el infinito hacia el que se apunta y ya: está a una distancia concreta, y solo lo que se interpone entre la superficie y ella cuenta. Sin esa comparación, cualquier objeto en esa dirección oscurecería el punto, incluso uno que esté detrás de la lámpara.

## Acné de sombra

Aquí aparece el bug clásico del raytracing, y vale la pena provocarlo antes de arreglarlo. Si el rayo de sombra sale exactamente del punto de impacto:

```rust
let shadow_ray_origin = intersect.point;
```

...la escena se llena de un moteado oscuro sobre las superficies que **sí** están iluminadas. La causa es aritmética, no geométrica: `intersect.point` se calculó en punto flotante y tiene error de redondeo, así que muchas veces queda un pelo **por debajo** de la superficie de la que salió. El rayo de sombra entonces sale desde adentro del objeto, choca inmediatamente contra su propia cara, y el punto se declara a sí mismo en sombra.

La corrección es empujar el origen un poco hacia afuera, en la dirección de la normal:

```rust
let shadow_ray_origin = intersect.point + intersect.normal * SHADOW_BIAS;
```

El valor del `SHADOW_BIAS` es un compromiso: muy chico no alcanza a salvar el error de redondeo y el moteado vuelve; muy grande despega la sombra del objeto que la produce y deja una franja de luz donde debería haber contacto. `1e-3` funciona para una escena de estas dimensiones — con una escena mil veces más grande habría que subirlo.

## Apagar la luz, no pintar de negro

La tentación es devolver un color oscuro cuando el punto está en sombra. El resultado se ve casi igual, pero conceptualmente es al revés: lo que hace la sombra no es agregar oscuridad, es **quitar luz**. Por eso lo que se anula es la intensidad, y la fórmula de Phong se queda intacta:

```rust
let light_intensity = if cast_shadow(...) { 0.0 } else { light.intensity };
```

La ventaja se ve al llegar a la etapa de reflexiones y a las escenas con varias luces: cada luz se evalúa por su cuenta con su propia sombra, y un punto puede quedar tapado para una y expuesto para otra. Con un color fijo de sombra eso no se puede expresar.

El costo de hacerlo bien es que las sombras salen **completamente negras**. Es correcto para el modelo que hay: una sola luz puntual, sin término ambiental y sin rebotes. En el mundo real la cara oscura recibe luz rebotada de las demás superficies, y eso llega hasta la etapa de reflexiones.

## Un piso para la escena

Una sombra necesita dónde caer. Las etapas anteriores tenían los objetos flotando contra el fondo, así que las sombras se habrían perdido en el vacío.

El piso no requirió una primitiva nueva: es el **cilindro** de la etapa anterior, aplastado y ensanchado.

```rust
Cylinder::new(
    Vec3::new(0.0, -2.0, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    0.25,
    6.0,
    slate,
)
```

Un cilindro de 6 unidades de radio y 0.25 de alto es un disco, y su tapa superior es una superficie plana con normal constante hacia arriba — un piso. Que la misma estructura sirva para una columna delgada e inclinada y para el suelo es la señal de que la primitiva quedó bien planteada.

Los objetos se bajaron para que descansen sobre él, y la cámara mira ahora un poco por encima del piso en lugar del origen.

## Estructura

```
.
├── Cargo.toml            # Manifiesto del proyecto (minifb, nalgebra-glm)
├── Cargo.lock            # Versiones exactas de las dependencias
└── src
    ├── main.rs           # Generación de rayos, rayo de sombra, Phong y ciclo de eventos
    ├── camera.rs         # Base de la cámara, cambio de base y órbita
    ├── light.rs          # Luz puntual
    ├── framebuffer.rs    # Buffer de píxeles en memoria
    ├── color.rs          # Color por canales, suma y escalado
    ├── ray_intersect.rs  # Material, Intersect y el trait común a los objetos
    ├── sphere.rs         # Esfera, solución de la cuadrática y normal
    └── cylinder.rs       # Cilindro finito: costado, tapas y normales
```

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 17-RT-05-SHADOW
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Los objetos descansan ahora sobre un disco de piedra y cada uno proyecta su sombra sobre él y sobre los demás — la esfera azul queda dentro de la sombra del cilindro. Con las flechas se orbita alrededor: conviene notar que las sombras **no se mueven** con la cámara, porque dependen de la luz y no del observador, a diferencia del brillo especular de la etapa anterior. Vale la pena bajar `SHADOW_BIAS` a `1e-7` para ver aparecer el acné de sombra, y subirlo a `0.5` para ver las sombras despegarse de los objetos. Cerrar con `Escape` o con el botón de cerrar de la ventana.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [minifb](https://docs.rs/minifb/)
- [nalgebra-glm](https://docs.rs/nalgebra-glm/)
- [Shadow mapping — shadow acne](https://en.wikipedia.org/wiki/Shadow_mapping)
- [Ray casting — visibilidad](https://en.wikipedia.org/wiki/Ray_casting)
- [Scratchapixel — Lights and Shadows](https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/ligth-and-shadows.html)
- [`Iterator::any`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.any)
