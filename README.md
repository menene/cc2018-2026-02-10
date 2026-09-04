# 16 — Raytracing: Iluminación

Cuarta etapa de la fase de **Raytracing** del curso **cc2018 – Gráficas por Computadora** (UVG). Las tres etapas anteriores ya sabían dónde está cada objeto y cuál está adelante, pero pintaban cada impacto con el color plano del material: la imagen era un recorte de discos de color, sin volumen. Esta etapa agrega una **luz** a la escena, y con ella la única información que faltaba para que una esfera se vea como una esfera.

Junto con la iluminación entra una **segunda primitiva** —un cilindro— que obliga a que la escena deje de ser una lista de esferas y pase a ser una lista de objetos de tipos distintos.

## Objetivo

- Modelar una luz puntual con posición, color e intensidad.
- Calcular la componente **difusa** con la ley de Lambert.
- Calcular la componente **especular** con el modelo de Phong.
- Repartir la luz que devuelve cada material con un **albedo**.
- Agregar el **cilindro** como segunda primitiva y guardar objetos de tipos distintos en la misma escena.

## Controles

| Tecla | Acción |
| ----- | ------ |
| `←` | Orbitar hacia la izquierda |
| `→` | Orbitar hacia la derecha |
| `↑` | Subir sobre la escena |
| `↓` | Bajar bajo la escena |
| `Escape` | Salir |

## Por qué la esfera se veía plana

Un objeto no se percibe por su color sino por cómo **varía** su color: la parte que ve hacia la luz llega más clara y la que ve para el otro lado más oscura, y ese degradado es lo que el ojo lee como forma. Sin luz no hay degradado, y sin degradado la esfera es un círculo.

El dato que hace posible ese degradado ya estaba calculado desde la etapa 13: la **normal** del impacto, que dice hacia dónde ve la superficie en ese punto exacto. Hasta ahora venía guardada en `Intersect` sin que nadie la usara. Esta etapa es, en buena medida, ponerla a trabajar.

## La luz puntual

```rust
pub struct Light {
    pub position: Vec3,
    pub color: Color,
    pub intensity: f32,
}
```

Es la luz más simple que existe: un punto que emite en todas las direcciones. No tiene tamaño ni forma, así que no se ve a sí misma en la imagen —no hay ningún objeto que el rayo pueda tocar donde está la luz— y tampoco produce penumbra. Lo único que aporta es, para cada punto de la escena, una **dirección** hacia dónde está la luz y una **cantidad** de luz que llega.

La intensidad es un multiplicador, no una unidad física. Con `1.0` la escena queda en su exposición «normal»; subirla quema los canales hasta saturarlos en blanco, que es justo lo que hace el `saturating_add` de `Color`.

## Difuso: la ley de Lambert

Una superficie mate —yeso, papel, hule— dispersa la luz que recibe por igual en todas las direcciones. Por eso su brillo no depende de dónde esté el observador, solo de **cuánta luz le llega**, y eso depende únicamente de qué tan de frente esté puesta:

```rust
let diffuse_intensity = dot(&intersect.normal, &light_direction).max(0.0);
```

Una superficie de frente a la luz recibe toda su energía; una de canto recibe casi nada, porque el mismo haz se reparte sobre un área mucho mayor. Ese factor es exactamente el coseno del ángulo entre la normal y la dirección de la luz — y el coseno entre dos vectores unitarios es su producto punto. Toda la ley de Lambert cabe en esa línea.

El `max(0.0)` es la parte que se olvida. Cuando la superficie ve para el lado contrario a la luz, el coseno es negativo, y sin el recorte esa cara terminaría **restando** luz al resultado: no existe la luz negativa.

## Especular: el modelo de Phong

Una superficie pulida no dispersa parejo: refleja preferentemente en la dirección del espejo. Por eso su brillo sí depende de dónde esté el observador — el punto blanco de una manzana se mueve sobre la fruta cuando uno se mueve alrededor.

Phong aproxima eso rebotando el rayo de luz contra la superficie y midiendo cuánto se parece el rebote a la dirección desde la que se está mirando:

```rust
let reflect_direction = reflect(&-light_direction, &intersect.normal);
let specular_intensity = dot(&view_direction, &reflect_direction)
    .max(0.0)
    .powf(intersect.material.specular);
```

Si las dos direcciones coinciden, el observador está parado justo donde la luz se refleja y ve el punto brillante. El producto punto vuelve a dar el coseno, y el exponente es el que **cierra el cono**: elevar un número entre 0 y 1 a una potencia alta lo derrumba salvo muy cerca de 1, así que un exponente grande deja un reflejo pequeño y duro —metal, vidrio— y uno chico deja un brillo ancho y suave —plástico, hule—.

El reflejo se pinta del color de la **luz**, no del objeto:

```rust
let specular = light.color * (specular_intensity * ... );
```

Eso no es un detalle arbitrario. El brillo especular es luz que rebota en la superficie sin llegar a penetrarla, así que no se tiñe del pigmento del material — por eso la esfera de hule rojo tiene un punto **blanco** y no uno rojo claro.

## Albedo: repartir la luz

Cada material devuelve la luz que recibe por los dos caminos anteriores, y `albedo` decide en qué proporción:

```rust
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 2],
}
```

La primera componente pesa el difuso y la segunda el especular. Ese par, junto con el exponente, es lo que separa un material de otro más que el color:

| Material | Difuso | Especular | Exponente | Se ve como |
| --- | --- | --- | --- | --- |
| Hule | 0.9 | 0.1 | 10 | Mate, apenas un brillo ancho |
| Marfil | 0.6 | 0.3 | 50 | Opaco con un reflejo definido |
| Jade | 0.8 | 0.25 | 30 | Piedra pulida |
| Cobalto | 0.7 | 0.4 | 80 | Esmalte, reflejo chico y duro |

Vale la pena subir el albedo especular del hule a 0.8 y ver cómo deja de parecer hule sin haberle cambiado el color.

## Una segunda primitiva: el cilindro

El trait `RayIntersect` existe desde la etapa 13 pero hasta ahora lo implementaba un solo tipo, así que no estaba probando nada. `Cylinder` es la primera primitiva que lo usa de verdad:

```rust
pub struct Cylinder {
    pub base: Vec3,
    pub axis: Vec3,
    pub height: f32,
    pub radius: f32,
    pub material: Material,
}
```

La diferencia de fondo con la esfera es que el cilindro **no es simétrico en todas las direcciones**: tiene una orientación propia y un largo. Eso trae dos consecuencias, y las dos se resuelven con la misma idea.

### Separar el vector en dos partes

Cada vector del problema se parte en cuánto avanza **a lo largo** del eje y cuánto se aleja de él:

```rust
let d_axis = dot(ray_direction, &self.axis);
let d_perp = ray_direction - d_axis * self.axis;
```

Visto desde el eje, la sección del cilindro es un círculo, y el problema perpendicular es la misma cuadrática de la esfera pero con los vectores ya sin su componente axial. La parte paralela, mientras tanto, dice **a qué altura** ocurrió el impacto, que es lo que permite recortar el tubo infinito al tramo entre 0 y `height`.

### Las dos raíces sí importan

En la esfera bastaba la raíz menor, la de entrada. Aquí hay que revisar las dos:

```rust
for t in [(-b - root) / (2.0 * a), (-b + root) / (2.0 * a)] {
```

Si la entrada cae fuera del tramo, la salida todavía puede caer dentro — es lo que pasa cuando el rayo entra por encima del borde y sale por el costado, o cuando se mira el cilindro desde adentro. Descartar la segunda raíz deja agujeros en la silueta que solo aparecen desde ciertos ángulos.

### Las tapas

El tubo por sí solo es hueco: mirándolo desde arriba se vería el fondo de la escena por dentro. Las tapas son dos planos perpendiculares al eje, uno en cada extremo, recortados al disco de radio `radius`:

```rust
let t = (height - oc_axis) / d_axis;
let radial = point - height * self.axis;

if dot(&radial, &radial) <= self.radius * self.radius {
```

La comparación se hace contra el radio **al cuadrado** para no calcular una raíz que solo se iba a usar para comparar.

### Los casos degenerados

Las dos divisiones del método pueden partir de cero, y las dos tienen un significado geométrico claro:

- `a` se anula cuando el rayo corre paralelo al eje. Nunca se acerca ni se aleja del tubo, así que solo puede entrar por las tapas.
- `d_axis` se anula cuando el rayo corre paralelo a las tapas, y entonces no las cruza nunca.

Cada caso se descarta antes de dividir, comparando contra un `EPSILON` en lugar de contra cero: en punto flotante «casi paralelo» produce divisiones enormes y artefactos igual de visibles que un `NaN`.

### La normal

En la esfera la normal era trivial. Aquí depende de por dónde entró el rayo: en el costado sale del eje hacia el punto, perpendicular al eje; en las tapas es el eje mismo, con signo según cuál de las dos sea.

```rust
let normal = (point - height * self.axis) / self.radius;
```

Dividir entre el radio la deja unitaria sin llamar a `normalize`: el vector ya mide exactamente `radius` por construcción.

## Objetos de tipos distintos en la misma escena

Con dos primitivas, `&[Sphere]` deja de servir. La escena pasa a guardar **objetos de tipo abstracto**:

```rust
let objects: Vec<Box<dyn RayIntersect>> = vec![
    Box::new(Sphere { ... }),
    Box::new(Cylinder::new( ... )),
];
```

`dyn RayIntersect` significa «algo que sabe contestar `ray_intersect`, sin decir qué es». Como cada tipo ocupa un tamaño distinto en memoria, no caben directamente en un `Vec`; el `Box` deja en el vector una referencia de tamaño fijo y manda el objeto al heap. La llamada se resuelve entonces en tiempo de ejecución, con una indirección más por rayo.

El `cast_ray` no cambió ni una línea por esto: siempre pidió lo mismo del trait. Ese es el punto de haberlo definido desde el principio — agregar una primitiva nueva es escribir un archivo y una línea en la escena, sin tocar el motor.

## Lo que todavía falta

La escena está iluminada pero los objetos no se estorban entre sí: el cilindro no proyecta nada sobre la esfera de marfil, porque nadie está preguntando si el camino hacia la luz está libre. Esa pregunta —un segundo rayo, lanzado del punto de impacto hacia la luz— es la etapa siguiente.

También se nota que la cara oscura está **completamente** negra. En el mundo real siempre llega algo de luz rebotada de las demás superficies; aquí no hay término ambiental ni rebotes, así que lo que no ve la luz directa no recibe nada.

## Estructura

```
.
├── Cargo.toml            # Manifiesto del proyecto (minifb, nalgebra-glm)
├── Cargo.lock            # Versiones exactas de las dependencias
└── src
    ├── main.rs           # Generación de rayos, sombreado de Phong y ciclo de eventos
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
    git checkout 16-RT-04-LIGHT
    ```

2. Compilar y ejecutar:
    ```bash
    cargo run
    ```

3. Se abre la escena de la etapa anterior, ahora con volumen: cada objeto tiene un lado iluminado, un degradado hacia el lado oscuro y un punto de brillo blanco. Con las flechas se orbita alrededor; conviene fijarse en que el brillo especular **se desplaza** sobre la superficie mientras la cámara se mueve, mientras que el degradado difuso se queda quieto — esa es la diferencia entre los dos términos, vista directamente. Subiendo con `↑` aparece la tapa del cilindro. Cerrar con `Escape` o con el botón de cerrar de la ventana.

## Recursos

- [Rust Programming Language](https://www.rust-lang.org/)
- [minifb](https://docs.rs/minifb/)
- [nalgebra-glm](https://docs.rs/nalgebra-glm/)
- [Lambertian reflectance](https://en.wikipedia.org/wiki/Lambertian_reflectance)
- [Phong reflection model](https://en.wikipedia.org/wiki/Phong_reflection_model)
- [Specular highlight](https://en.wikipedia.org/wiki/Specular_highlight)
- [Trait objects — The Rust Programming Language](https://doc.rust-lang.org/book/ch18-02-trait-objects.html)
- [Scratchapixel — Introduction to Shading](https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/diffuse-lambertian-shading.html)
