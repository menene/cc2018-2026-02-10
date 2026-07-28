use nalgebra_glm::Vec2;

pub struct Player {
    /// Posición del jugador en el mundo 2D, en píxeles.
    pub pos: Vec2,
    /// Ángulo de vista, en radianes.
    pub a: f32,
}
