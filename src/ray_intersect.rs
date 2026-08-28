use nalgebra_glm::Vec3;

/// Todo objeto de la escena responde a la misma pregunta: dado un rayo,
/// ¿lo toca o no? En esta etapa la respuesta es un `bool`; más adelante
/// tendrá que devolver también dónde, a qué distancia y con qué material.
pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> bool;
}
