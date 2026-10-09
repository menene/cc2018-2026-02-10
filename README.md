# 19 — Raytracing: Refracciones

Séptima y última etapa de la fase de **Raytracing** del curso **cc2018 – Gráficas por Computadora** (UVG). En la etapa anterior el rayo aprendió a rebotar: al chocar con una superficie pulida seguía viajando en otra dirección. Pero todas las superficies de la escena eran opacas — el rayo nunca entraba en nada.

Esta etapa le enseña a **atravesar**. Un rayo que llega a un material transparente se parte en dos: una parte rebota y otra entra al material, cambiando de dirección al cruzar la frontera. Ese doblez es lo que hace que un lápiz dentro de un vaso de agua se vea quebrado, y es lo único que falta para que la escena tenga vidrio.

## Objetivo

- Implementar la **ley de Snell** para calcular la dirección del rayo transmitido.
- Detectar y manejar la **reflexión interna total**.
- Agregar un cuarto canal al albedo y un **índice de refracción** al material.
- Permitir que un rayo golpee un objeto **desde adentro**.
- Dejar que la luz pase por los objetos transparentes al calcular sombras.

## Controles

| Tecla | Acción |
| ----- | ------ |
| `←` | Orbitar hacia la izquierda |
| `→` | Orbitar hacia la derecha |
| `↑` | Subir sobre la escena |
| `↓` | Bajar bajo la escena |
| `Escape` | Salir |

## El doblez

La luz viaja más lento dentro del vidrio que en el aire, y al cruzar la frontera entre ambos medios cambia de dirección. Cuánto cambia lo dice la ley de Snell, que relaciona los ángulos con los **índices de refracción** de los dos medios:

```
n₁ · sen θ₁ = n₂ · sen θ₂
```

El aire vale 1.0, el agua 1.33, el vidrio alrededor de 1.5 y el diamante 2.42. Entre más alto el índice del segundo medio, más se cierra el rayo contra la normal.

En código no se trabaja con ángulos sino con vectores, así que la fórmula se reescribe en términos del coseno —que es un producto punto— y de `eta`, la razón entre los dos índices:

```rust
let eta = outside_index / inside_index;
let cosine_incident = cosine.abs();

let cosine_squared = 1.0 - eta * eta * (1.0 - cosine_incident * cosine_incident);

Some(incident * eta + surface_normal * (eta * cosine_incident - cosine_squared.sqrt()))
```

El resultado es un vector, no un ángulo, y se usa igual que el reflejado: como la dirección de un rayo nuevo que se dispara contra la escena.

## Entrar y salir

Una diferencia importante con `reflect`: la refracción no es simétrica. Al entrar al vidrio el rayo se cierra contra la normal; al salir se abre. Son la misma fórmula con los índices intercambiados, y la función tiene que saber en cuál de los dos casos está.

La pista está en el signo del producto punto entre el rayo y la normal:

```rust
let (outside_index, inside_index, surface_normal) = if cosine < 0.0 {
    (AIR_REFRACTIVE_INDEX, refractive_index, *normal)
} else {
    (refractive_index, AIR_REFRACTIVE_INDEX, -normal)
};
```

Si el coseno es negativo, el rayo viene de frente contra la normal: está entrando. Si es positivo, el rayo y la normal apuntan hacia el mismo lado, lo que solo puede pasar si el rayo golpea la superficie desde adentro: está saliendo. En ese caso se intercambian los índices y se voltea la normal, porque el resto del cálculo asume que la normal apunta hacia el medio de donde viene el rayo.

Esto es lo que permite que la esfera de vidrio se dibuje con una sola función: el rayo primario entra por el frente, el rayo transmitido cruza el interior y sale por atrás, y los dos impactos pasan por el mismo código.

## La esfera por dentro

Para que ese segundo impacto exista, `Sphere` tuvo que cambiar. Hasta ahora descartaba la raíz lejana de la cuadrática:

```rust
let t = (-b - discriminant.sqrt()) / (2.0 * a);

if t <= 0.0 {
    return None;
}
```

Una esfera opaca vista desde afuera no necesita más que la raíz cercana — la cara de atrás nunca se ve. Pero un rayo que nace **dentro** de la esfera tiene la raíz cercana detrás de sí, en negativo, y con ese código la esfera no devolvía nada: el rayo salía al vacío sin doblarse y el vidrio se veía como una bola de plástico transparente, sin distorsión.

```rust
let near = (-b - root) / (2.0 * a);
let far = (-b + root) / (2.0 * a);

let t = [near, far].into_iter().find(|t| *t > EPSILON)?;
```

Se toma la primera raíz que quede adelante. El cilindro ya lo hacía desde la etapa 13, porque sus dos tapas lo obligaban a comparar varios candidatos.

La normal se sigue calculando igual, apuntando hacia afuera del centro — incluso cuando el rayo la golpea por dentro. No hay que voltearla aquí: `refract` ya se encarga, y dejarla siempre hacia afuera mantiene una sola convención en toda la escena.

## Cuando no hay salida

No siempre existe un rayo transmitido. Al pasar de un medio denso a uno menos denso —de vidrio a aire— el rayo se abre, y si llega demasiado inclinado tendría que abrirse más de 90°, lo cual es imposible: la superficie deja de comportarse como ventana y se comporta como espejo. Es la **reflexión interna total**, el mismo efecto que hace que la superficie del agua, vista desde abajo, se vea plateada en los bordes.

La aritmética lo delata sola. El término bajo la raíz se vuelve negativo:

```rust
if cosine_squared < 0.0 {
    return None;
}
```

Por eso `refract` devuelve `Option<Vec3>` y no un vector: el caso sin solución no es un error ni un vector cero, es un resultado legítimo que el tipo hace imposible ignorar. Cuando ocurre, la energía que iba a transmitirse se refleja:

```rust
let reflected_weight = if transparency > 0.0 && refracted.is_none() {
    reflectivity + transparency
} else {
    reflectivity
};
```

Es el borde brillante que se ve en el contorno de la esfera de vidrio, donde los rayos la rozan de lado.

## El cuarto canal del albedo

El albedo pasó de tres coeficientes a cuatro, y el material ganó un campo:

```rust
pub albedo: [f32; 4],
pub refractive_index: f32,
```

El nuevo canal es la transparencia, y entra en la mezcla exactamente igual que la reflectividad en la etapa anterior:

```rust
let own_weight = (1.0 - reflectivity - transparency).max(0.0);
```

Los tres pesos —color propio, reflejo, transmisión— suman 1, así que la superficie no inventa luz. El vidrio de la escena usa `[0.0, 0.5, 0.1, 0.8]`: no tiene color difuso propio, responde con un brillo especular, refleja un 10% y transmite un 80%. El 10% restante es el peso que se queda en el `own_weight` y sostiene el brillo.

En la realidad esos pesos no son constantes: dependen del ángulo de incidencia, y por eso un lago se ve transparente a los pies y como espejo en la orilla lejana. Calcularlo es lo que hacen las **ecuaciones de Fresnel**, y es el siguiente paso natural para quien quiera llevar este motor más allá del curso.

## El empujón, otra vez — pero en otra dirección

Las dos etapas anteriores desplazaron el origen de los rayos secundarios en la dirección de la normal, para que no chocaran contra su propia superficie por error de redondeo. Con la refracción esa receta deja de servir: el rayo transmitido tiene que irse **hacia adentro** del objeto, que es justo el lado del que la normal lo aleja. Empujarlo por la normal lo sacaría del vidrio antes de que entre.

La corrección sirve para los dos casos:

```rust
let refract_origin = intersect.point + refract_direction * SECONDARY_RAY_BIAS;
```

Se desplaza el origen en la dirección en la que el rayo ya va a viajar. Para el reflejado eso apunta hacia afuera y hace lo mismo que la normal; para el transmitido apunta hacia adentro y hace lo correcto. Una sola constante, `SECONDARY_RAY_BIAS`, cubre ambos, y `SHADOW_BIAS` se queda con la normal porque el rayo de sombra siempre sale de la superficie.

## Sombras que dejan pasar la luz

El rayo de sombra de la etapa 17 respondía sí o no: si algo se interponía entre el punto y la luz, el punto quedaba a oscuras. Con vidrio en la escena esa respuesta se vuelve absurda — una bola transparente proyectaba una sombra tan negra como una de hierro.

`cast_shadow` ahora devuelve cuánta luz pasa, no si pasa:

```rust
transmission *= blocker.material.albedo[3];
```

Cada obstáculo multiplica por su transparencia. Un objeto opaco tiene transparencia 0.0 y apaga la luz por completo, igual que antes; el vidrio deja pasar el 80% y proyecta una sombra tenue. El comportamiento viejo sigue adentro del nuevo, como el caso particular que siempre fue.

Lo que esta aproximación **no** hace es doblar la luz: la atenúa en línea recta. Un vidrio real concentra los rayos y dibuja una mancha brillante dentro de su propia sombra —una cáustica— que este modelo no produce, porque las sombras se calculan desde la superficie hacia la luz y no siguiendo el camino real de la luz hacia la superficie.

## El costo de la recursión

Hasta la etapa anterior la recursión era lineal: un rayo entraba, un rayo salía. Ahora cada impacto en vidrio puede generar dos rayos, y el peor caso pasa a ser exponencial en la profundidad: hasta 2^(d+1) − 1 rayos por píxel.

En la práctica el crecimiento es más suave, porque solo un material de la escena tiene reflectividad y transparencia al mismo tiempo, y porque las ramas mueren temprano al salir al fondo. Medido en esta escena a 800×600, subir `MAX_DEPTH` de 3 a 6 multiplica el tiempo de render por unas 2.5 veces y produce una imagen que no se distingue de la anterior: a esa altura cada rayo lleva tantos factores menores que 1 encima que su aporte ya no alcanza un canal de 8 bits.

`MAX_DEPTH = 3` se queda, entonces, no por falta de presupuesto sino porque es donde la imagen deja de cambiar.

## La escena

El material nuevo es el vidrio:

```rust
let glass = Material::new(Color::new(255, 255, 255), 125.0, [0.0, 0.5, 0.1, 0.8], 1.5);
```

La esfera de vidrio se coloca al frente, parcialmente encima de la esfera de marfil, que es el objeto más iluminado de la escena. La posición no es casual: un vidrio solo se ve si hay algo detrás que deformar. Contra el fondo —que casi no tiene luz— la misma esfera se ve como una bola negra, que fue exactamente lo que apareció en el primer intento.

Adentro se alcanzan a ver el piso invertido, la franja del horizonte curvada y el reflejo del pilar de jade. La inversión es la firma de la refracción: una esfera transparente funciona como lente y da vuelta la imagen de lo que tiene detrás.

Los otros seis objetos conservan sus materiales; solo se les agregó el cuarto canal en 0.0 y un índice de refracción de 1.0, que para un material opaco nunca se usa.

## Estructura

```
.
├── Cargo.toml            # Manifiesto del proyecto (minifb, nalgebra-glm)
├── Cargo.lock            # Versiones exactas de las dependencias
└── src
    ├── main.rs           # Rayos primarios, reflejado, refractado y de sombra, Phong y ciclo de eventos
    ├── camera.rs         # Base de la cámara, cambio de base y órbita
    ├── light.rs          # Luz puntual
    ├── framebuffer.rs    # Buffer de píxeles en memoria
    ├── color.rs          # Color por canales, suma y escalado
    ├── ray_intersect.rs  # Material con albedo de cuatro canales e índice de refracción
    ├── sphere.rs         # Esfera, raíz cercana y lejana de la cuadrática, normal
    └── cylinder.rs       # Cilindro finito: costado, tapas y normales
```

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 19-RT-07-REFRACTIONS
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Vale la pena orbitar despacio alrededor de la esfera de vidrio: la imagen de adentro se mueve mucho más rápido que la escena, porque un desplazamiento pequeño de la cámara cambia bastante el ángulo con que los rayos entran al lente. Poner el índice de refracción en `1.0` hace que el vidrio deje de doblar la luz y la esfera casi desaparezca, quedando apenas su brillo y su reflejo; subirlo a `2.42` —diamante— aprieta tanto la imagen interior que la esfera se oscurece por reflexión interna total. Bajar la transparencia a `0.0` devuelve una bola opaca. Cerrar con `Escape` o con el botón de cerrar de la ventana.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [minifb](https://docs.rs/minifb/)
- [nalgebra-glm](https://docs.rs/nalgebra-glm/)
- [Snell's law](https://en.wikipedia.org/wiki/Snell%27s_law)
- [Total internal reflection](https://en.wikipedia.org/wiki/Total_internal_reflection)
- [List of refractive indices](https://en.wikipedia.org/wiki/List_of_refractive_indices)
- [Scratchapixel — Reflection, Refraction and Fresnel](https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel.html)
- [tinyraytracer — el mismo camino en C++](https://github.com/ssloy/tinyraytracer)
