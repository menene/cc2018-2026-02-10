mod camera;
mod color;
mod cylinder;
mod framebuffer;
mod light;
mod ray_intersect;
mod sphere;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{dot, normalize, Vec3};
use std::f32::consts::PI;
use std::time::Duration;

use crate::camera::Camera;
use crate::color::Color;
use crate::cylinder::Cylinder;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use crate::ray_intersect::{Intersect, Material, RayIntersect};
use crate::sphere::Sphere;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const BACKGROUND_COLOR: u32 = 0x040C24;

const FOV: f32 = PI / 3.0;

const ROTATION_SPEED: f32 = PI / 60.0;

const SHADOW_BIAS: f32 = 1e-3;
const SECONDARY_RAY_BIAS: f32 = 1e-3;

const AIR_REFRACTIVE_INDEX: f32 = 1.0;

const MAX_DEPTH: u32 = 3;

pub fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - normal * (2.0 * dot(incident, normal))
}

pub fn refract(incident: &Vec3, normal: &Vec3, refractive_index: f32) -> Option<Vec3> {
    let cosine = dot(incident, normal).clamp(-1.0, 1.0);

    let (outside_index, inside_index, surface_normal) = if cosine < 0.0 {
        (AIR_REFRACTIVE_INDEX, refractive_index, *normal)
    } else {
        (refractive_index, AIR_REFRACTIVE_INDEX, -normal)
    };

    let eta = outside_index / inside_index;
    let cosine_incident = cosine.abs();

    let cosine_squared = 1.0 - eta * eta * (1.0 - cosine_incident * cosine_incident);

    if cosine_squared < 0.0 {
        return None;
    }

    Some(incident * eta + surface_normal * (eta * cosine_incident - cosine_squared.sqrt()))
}

pub fn cast_shadow(
    intersect: &Intersect,
    light_direction: &Vec3,
    light: &Light,
    objects: &[Box<dyn RayIntersect>],
) -> f32 {
    let shadow_ray_origin = intersect.point + intersect.normal * SHADOW_BIAS;
    let light_distance = (light.position - intersect.point).magnitude();

    let mut transmission = 1.0;

    for object in objects {
        if let Some(blocker) = object.ray_intersect(&shadow_ray_origin, light_direction) {
            if blocker.distance < light_distance {
                transmission *= blocker.material.albedo[3];

                if transmission <= 0.0 {
                    return 0.0;
                }
            }
        }
    }

    transmission
}

pub fn shade(
    intersect: &Intersect,
    ray_origin: &Vec3,
    light: &Light,
    objects: &[Box<dyn RayIntersect>],
) -> Color {
    let light_direction = (light.position - intersect.point).normalize();
    let view_direction = (ray_origin - intersect.point).normalize();

    let light_intensity =
        light.intensity * cast_shadow(intersect, &light_direction, light, objects);

    let diffuse_intensity = dot(&intersect.normal, &light_direction).max(0.0);
    let diffuse = intersect.material.diffuse
        * (diffuse_intensity * intersect.material.albedo[0] * light_intensity);

    let reflect_direction = reflect(&-light_direction, &intersect.normal);
    let specular_intensity = dot(&view_direction, &reflect_direction)
        .max(0.0)
        .powf(intersect.material.specular);

    let specular =
        light.color * (specular_intensity * intersect.material.albedo[1] * light_intensity);

    diffuse + specular
}

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Box<dyn RayIntersect>],
    light: &Light,
    depth: u32,
) -> Color {
    if depth > MAX_DEPTH {
        return Color::from_hex(BACKGROUND_COLOR);
    }

    let mut closest: Option<Intersect> = None;

    for object in objects {
        if let Some(intersect) = object.ray_intersect(ray_origin, ray_direction) {
            if closest.is_none_or(|current| intersect.distance < current.distance) {
                closest = Some(intersect);
            }
        }
    }

    let Some(intersect) = closest else {
        return Color::from_hex(BACKGROUND_COLOR);
    };

    let reflectivity = intersect.material.albedo[2];
    let transparency = intersect.material.albedo[3];

    let own_weight = (1.0 - reflectivity - transparency).max(0.0);

    let mut color = shade(&intersect, ray_origin, light, objects) * own_weight;

    let refracted = if transparency > 0.0 {
        refract(
            ray_direction,
            &intersect.normal,
            intersect.material.refractive_index,
        )
    } else {
        None
    };

    // La reflexión interna total devuelve el rayo transmitido hacia adentro del
    // objeto: su peso se suma al de la reflexión en lugar de perderse.
    let reflected_weight = if transparency > 0.0 && refracted.is_none() {
        reflectivity + transparency
    } else {
        reflectivity
    };

    if reflected_weight > 0.0 {
        let reflect_direction = reflect(ray_direction, &intersect.normal).normalize();
        let reflect_origin = intersect.point + reflect_direction * SECONDARY_RAY_BIAS;

        color = color
            + cast_ray(&reflect_origin, &reflect_direction, objects, light, depth + 1)
                * reflected_weight;
    }

    if let Some(refract_direction) = refracted {
        let refract_direction = refract_direction.normalize();
        let refract_origin = intersect.point + refract_direction * SECONDARY_RAY_BIAS;

        color = color
            + cast_ray(&refract_origin, &refract_direction, objects, light, depth + 1)
                * transparency;
    }

    color
}

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Box<dyn RayIntersect>],
    camera: &Camera,
    light: &Light,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    let perspective_scale = (FOV / 2.0).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let ray_direction = camera.basis_change(&ray_direction);

            framebuffer.set_current_color(
                cast_ray(&camera.eye, &ray_direction, objects, light, 0).to_hex(),
            );
            framebuffer.point(x, y);
        }
    }
}

fn main() {
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new("Lakitu", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let ivory = Material::new(Color::new(100, 100, 80), 50.0, [0.6, 0.3, 0.1, 0.0], 1.0);
    let rubber = Material::new(Color::new(80, 0, 0), 10.0, [0.9, 0.1, 0.0, 0.0], 1.0);
    let cobalt = Material::new(Color::new(40, 80, 140), 80.0, [0.7, 0.4, 0.15, 0.0], 1.0);
    let jade = Material::new(Color::new(60, 130, 100), 30.0, [0.8, 0.25, 0.05, 0.0], 1.0);
    let slate = Material::new(Color::new(80, 80, 92), 15.0, [0.85, 0.1, 0.2, 0.0], 1.0);
    let mirror = Material::new(Color::new(255, 255, 255), 1425.0, [0.0, 10.0, 0.85, 0.0], 1.0);
    let glass = Material::new(Color::new(255, 255, 255), 125.0, [0.0, 0.5, 0.1, 0.8], 1.5);

    let objects: Vec<Box<dyn RayIntersect>> = vec![
        Box::new(Cylinder::new(
            Vec3::new(0.0, -2.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            0.25,
            6.0,
            slate,
        )),
        Box::new(Sphere {
            center: Vec3::new(0.0, -0.75, 0.0),
            radius: 1.0,
            material: ivory,
        }),
        Box::new(Sphere {
            center: Vec3::new(1.9, -1.25, -0.9),
            radius: 0.5,
            material: rubber,
        }),
        Box::new(Sphere {
            center: Vec3::new(-1.5, -1.25, 1.1),
            radius: 0.5,
            material: cobalt,
        }),
        Box::new(Cylinder::new(
            Vec3::new(-2.3, -1.75, -0.6),
            Vec3::new(0.28, 1.0, -0.12),
            2.0,
            0.35,
            jade,
        )),
        Box::new(Sphere {
            center: Vec3::new(2.35, -1.0, 1.45),
            radius: 0.75,
            material: mirror,
        }),
        Box::new(Sphere {
            center: Vec3::new(-0.10, -0.95, 2.6),
            radius: 0.8,
            material: glass,
        }),
    ];

    let light = Light::new(Vec3::new(-6.0, 6.0, 8.0), Color::new(255, 255, 255), 1.5);

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.4, 6.0),
        Vec3::new(0.0, -0.7, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut camera_moved = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let orbit = [
            (Key::Left, ROTATION_SPEED, 0.0),
            (Key::Right, -ROTATION_SPEED, 0.0),
            (Key::Up, 0.0, -ROTATION_SPEED),
            (Key::Down, 0.0, ROTATION_SPEED),
        ];

        for (key, delta_yaw, delta_pitch) in orbit {
            if window.is_key_down(key) {
                camera.orbit(delta_yaw, delta_pitch);
                camera_moved = true;
            }
        }

        if camera_moved {
            render(&mut framebuffer, &objects, &camera, &light);
            camera_moved = false;
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
