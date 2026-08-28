use crate::ray_intersect::RayIntersect;
use nalgebra_glm::{dot, Vec3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> bool {
        // Sustituir el rayo `origen + t · dirección` en la ecuación de la
        // esfera deja una cuadrática en t: a·t² + b·t + c = 0.
        let oc = ray_origin - self.center;

        let a = dot(ray_direction, ray_direction);
        let b = 2.0 * dot(&oc, ray_direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;

        // El discriminante dice cuántas soluciones reales tiene: ninguna si
        // el rayo pasa de largo, dos si atraviesa la esfera.
        let discriminant = b * b - 4.0 * a * c;

        discriminant > 0.0
    }
}
