mod color;
mod framebuffer;
mod ray_intersect;
mod sphere;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{normalize, Vec3};
use std::time::Duration;

use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{Material, RayIntersect};
use crate::sphere::Sphere;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const BACKGROUND_COLOR: u32 = 0x040C24;

/// Distancia de la cámara al plano de proyección. Con el plano a una unidad
/// y el borde de la pantalla en x = ±1, el campo de visión es de 90 grados.
const PROJECTION_PLANE: f32 = 1.0;

/// Devuelve el color del objeto **más cercano** que toca el rayo. Ya no
/// basta con el primero que se encuentre: el orden del arreglo no dice
/// nada sobre qué está adelante.
pub fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> Color {
    let mut closest: Option<f32> = None;
    let mut color = Color::from_hex(BACKGROUND_COLOR);

    for object in objects {
        if let Some(intersect) = object.ray_intersect(ray_origin, ray_direction) {
            if closest.is_none_or(|distance| intersect.distance < distance) {
                closest = Some(intersect.distance);
                color = intersect.material.diffuse;
            }
        }
    }

    color
}

pub fn render(framebuffer: &mut Framebuffer, objects: &[Sphere]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    let camera = Vec3::new(0.0, 0.0, 0.0);

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            // De coordenadas de píxel a coordenadas de pantalla, de -1 a 1.
            // La y se invierte porque el píxel 0 está arriba y el eje Y
            // del mundo crece hacia arriba.
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            // Sin esto la escena se estira: la pantalla es más ancha que
            // alta, pero el rango -1..1 es el mismo en ambos ejes.
            let screen_x = screen_x * aspect_ratio;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -PROJECTION_PLANE));

            framebuffer.set_current_color(cast_ray(&camera, &ray_direction, objects).to_hex());
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new("Materials", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let ivory = Material::new(Color::new(100, 100, 80));
    let rubber = Material::new(Color::new(80, 0, 0));
    let cobalt = Material::new(Color::new(40, 80, 140));

    let objects = [
        Sphere {
            center: Vec3::new(0.0, 0.0, -4.0),
            radius: 1.0,
            material: ivory,
        },
        Sphere {
            center: Vec3::new(1.5, 0.0, -5.0),
            radius: 0.5,
            material: rubber,
        },
        // Se traslapa con la esfera de marfil y está más cerca, así que
        // debe quedar encima de ella. Es la prueba de que gana el impacto
        // más cercano y no el primero del arreglo.
        Sphere {
            center: Vec3::new(-1.0, 0.4, -3.0),
            radius: 0.6,
            material: cobalt,
        },
    ];

    // La escena no cambia y la cámara no se mueve, así que la imagen se
    // calcula una sola vez y el ciclo solo la vuelve a presentar.
    render(&mut framebuffer, &objects);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
