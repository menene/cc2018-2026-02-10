# 18 — Raytracing: Reflexiones

Sexta etapa de la fase de **Raytracing** del curso **cc2018 – Gráficas por Computadora** (UVG). La etapa anterior cerró con una deuda: las sombras salían completamente negras porque el modelo solo sabía de una luz puntual, sin rebotes. Un objeto no aportaba nada a otro; lo único que podía hacer era taparle la luz.

Esta etapa paga esa deuda. Cuando un rayo choca con una superficie pulida, en lugar de terminar ahí, **rebota y sigue viajando**. El motor deja de recorrer la escena en un solo salto y empieza a recorrerla en cadena.

## Objetivo

- Convertir `cast_ray` en una función **recursiva**.
- Lanzar un **rayo reflejado** desde el punto de impacto usando la ley de la reflexión.
- Limitar la recursión con una **profundidad máxima**.
- Agregar un tercer canal al albedo para mezclar el color propio con el reflejado.

## Controles

| Tecla | Acción |
| ----- | ------ |
| `←` | Orbitar hacia la izquierda |
| `→` | Orbitar hacia la derecha |
| `↑` | Subir sobre la escena |
| `↓` | Bajar bajo la escena |
| `Escape` | Salir |

## El rayo que ya estaba escrito

Un reflejo es lo que se vería si el observador estuviera parado del otro lado del espejo. Dicho así, la pregunta que hay que responder es *«¿qué se ve desde este punto, mirando en esta otra dirección?»* — y esa es, palabra por palabra, la pregunta que `cast_ray` viene respondiendo desde la etapa 13.

Por eso la implementación no agrega un algoritmo nuevo: `cast_ray` se llama a sí misma.

```rust
let reflected = cast_ray(
    &reflect_origin,
    &reflect_direction,
    objects,
    light,
    depth + 1,
);
```

El rayo primario sale del ojo; el reflejado sale de la superficie. Fuera de eso son el mismo rayo y pasan por el mismo código, incluido el cálculo de sombras. Un reflejo, entonces, viene con sus propias sombras sin que haya que programarlas por segunda vez.

## La misma fórmula, otro vector

La función `reflect` tampoco es nueva — existe desde la etapa 16, donde se usaba para el brillo especular:

```rust
pub fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - normal * (2.0 * dot(incident, normal))
}
```

Lo que cambia es qué se le pasa. Para el especular de Phong se refleja el vector **hacia la luz**, y el resultado se compara con la dirección de la vista para ver qué tanto coinciden. Aquí se refleja el vector **de la vista**, y el resultado se convierte en un rayo que efectivamente se dispara contra la escena.

La diferencia entre ambos usos explica la diferencia entre un brillo y un reflejo. El especular es una aproximación: pinta una mancha clara donde *debería* estar la luz, sin averiguar si la luz está tapada ni qué más hay alrededor. El reflejo no aproxima nada — sale a ver.

## Cortar la cadena

Nada impide que el rayo reflejado caiga sobre otra superficie pulida y genere un tercer rayo. Con el piso reflectivo y una esfera espejo encima, la cadena es infinita: el piso refleja la esfera, que refleja el piso, que refleja la esfera. Sin un freno el programa se queda sin pila y truena.

```rust
if depth > MAX_DEPTH {
    return Color::from_hex(BACKGROUND_COLOR);
}
```

Cada llamada recibe la profundidad a la que va y se detiene al pasar el límite, devolviendo el color de fondo como si el rayo se hubiera ido al vacío. `MAX_DEPTH = 3` significa hasta cuatro rayos encadenados por píxel.

El número es bajo a propósito y alcanza porque cada rebote pesa menos que el anterior: lo que se ve en el reflejo de un reflejo va multiplicado por dos reflectividades, y con dos superficies al 0.85 y 0.2 el tercer nivel ya aporta menos de lo que distingue un canal de 8 bits. Subirlo a 10 cuesta tiempo de render y no cambia la imagen.

Vale la pena notar que aquí la recursión es **lineal**: un rayo entra, un rayo sale. Cuando en la etapa siguiente se agregue la refracción, cada impacto podrá generar dos rayos —uno reflejado y uno transmitido— y el costo pasará a ser exponencial en la profundidad. Ahí el límite deja de ser una precaución y se vuelve el presupuesto.

## El tercer canal del albedo

El material tenía dos coeficientes: cuánto responde a la luz difusa y cuánto al brillo especular. Ahora tiene tres.

```rust
pub albedo: [f32; 3],
```

El tercero es la reflectividad, y decide la mezcla entre el color que la superficie produce por su cuenta y el que le llega del rayo reflejado:

```rust
color * (1.0 - reflectivity) + reflected * reflectivity
```

La mezcla está escrita para que los pesos sumen 1. Un material con reflectividad 0.85 conserva apenas el 15% de su propia respuesta a la luz, y esa es justamente la razón por la que un espejo no tiene color propio: casi todo lo que muestra es prestado. En el otro extremo, la goma roja tiene reflectividad 0.0, el `if` corta antes de lanzar nada y la etapa no le cuesta un solo rayo.

Escribirlo como suma pura —`color + reflected * reflectivity`, sin restarle nada al color propio— también produce una imagen aceptable, pero agrega energía que no existe: una superficie terminaría devolviendo más luz de la que recibe, y las zonas claras se saturan.

## El mismo bug, el mismo remedio

El rayo reflejado sale del punto de impacto y arrastra el mismo error de redondeo que el rayo de sombra de la etapa anterior: nace un pelo por debajo de la superficie, choca de inmediato contra su propia cara y devuelve un color negro moteado.

```rust
let reflect_origin = intersect.point + intersect.normal * REFLECTION_BIAS;
```

El remedio es idéntico — desplazar el origen en la dirección de la normal — y aparece por segunda vez porque la causa no era de las sombras, sino de la aritmética de punto flotante. Todo rayo secundario que salga de una superficie va a necesitar este empujón.

## La escena

El material nuevo es el espejo, con un especular altísimo para que la luz se refleje en un punto pequeño y duro, sin difuso propio:

```rust
let mirror = Material::new(Color::new(255, 255, 255), 1425.0, [0.0, 10.0, 0.85]);
```

El piso de piedra pasó a `0.2` de reflectividad. No es un espejo — es piedra pulida — pero basta para que la escena aparezca repetida debajo de los objetos y para que estos dejen de verse pegados encima de un disco gris.

La esfera espejo queda a la derecha, y en su costado izquierdo se alcanzan a ver la esfera roja y el reflejo curvado del horizonte. Su mitad superior sale casi negra: está reflejando fielmente un fondo que casi no tiene luz.

## Estructura

```
.
├── Cargo.toml            # Manifiesto del proyecto (minifb, nalgebra-glm)
├── Cargo.lock            # Versiones exactas de las dependencias
└── src
    ├── main.rs           # Generación de rayos, rayo reflejado, rayo de sombra, Phong y ciclo de eventos
    ├── camera.rs         # Base de la cámara, cambio de base y órbita
    ├── light.rs          # Luz puntual
    ├── framebuffer.rs    # Buffer de píxeles en memoria
    ├── color.rs          # Color por canales, suma y escalado
    ├── ray_intersect.rs  # Material con albedo de tres canales, Intersect y el trait común
    ├── sphere.rs         # Esfera, solución de la cuadrática y normal
    └── cylinder.rs       # Cilindro finito: costado, tapas y normales
```

## Cómo correr

1. Clonar el repositorio y cambiar a esta rama:
    ```bash
    git clone https://github.com/menene/cc2018-2026-02-10.git
    cd cc2018-2026-02-10
    git checkout 18-RT-06-REFLECTIONS
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Al orbitar con las flechas conviene fijarse en que el reflejo **sí se mueve** con la cámara, al revés de las sombras de la etapa anterior: lo que muestra un espejo depende de dónde está parado quien lo mira. Poner `MAX_DEPTH` en `0` deja los espejos ciegos —devuelven fondo— y en `1` aparece el primer rebote pero no el reflejo dentro del reflejo. Subir la reflectividad del piso a `0.9` convierte el disco en un lago. Cerrar con `Escape` o con el botón de cerrar de la ventana.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [minifb](https://docs.rs/minifb/)
- [nalgebra-glm](https://docs.rs/nalgebra-glm/)
- [Specular reflection](https://en.wikipedia.org/wiki/Specular_reflection)
- [Ray tracing — recursión](https://en.wikipedia.org/wiki/Ray_tracing_(graphics))
- [Scratchapixel — Reflection, Refraction and Fresnel](https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel.html)
- [tinyraytracer — el mismo camino en C++](https://github.com/ssloy/tinyraytracer)
