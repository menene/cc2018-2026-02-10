use crate::framebuffer::Framebuffer;
use crate::line::Line;
use nalgebra_glm::Vec3;

pub trait Polygon {
    fn polygon(&mut self, points: &[Vec3]);
    fn filled_polygon(&mut self, points: &[Vec3]);
}

impl Polygon for Framebuffer {
    fn polygon(&mut self, points: &[Vec3]) {
        if points.len() < 3 {
            return;
        }

        for i in 0..points.len() {
            let start = points[i];
            let end = points[(i + 1) % points.len()];

            self.line(start, end);
        }
    }

    fn filled_polygon(&mut self, points: &[Vec3]) {
        if points.len() < 3 {
            return;
        }

        // Límites del polígono en el eje Y.
        let min_y = points
            .iter()
            .map(|p| p.y)
            .fold(f32::INFINITY, f32::min) as usize;
        let max_y = points
            .iter()
            .map(|p| p.y)
            .fold(f32::NEG_INFINITY, f32::max) as usize;

        // Recorrer cada scanline horizontal.
        for y in min_y..=max_y {
            let mut intersections = Vec::new();

            // Buscar dónde cruza cada arista esta scanline.
            for i in 0..points.len() {
                let p1 = points[i];
                let p2 = points[(i + 1) % points.len()];

                let y_f = y as f32;

                // La arista cruza la scanline si un extremo queda a cada lado.
                if (p1.y <= y_f && p2.y > y_f) || (p2.y <= y_f && p1.y > y_f) {
                    let x = p1.x + (y_f - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
                    intersections.push(x);
                }
            }

            // Ordenar las intersecciones por X.
            intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

            // Rellenar entre cada par de intersecciones (regla par/impar).
            for pair in intersections.chunks(2) {
                if let [x1, x2] = pair {
                    for x in (*x1 as usize)..=(*x2 as usize) {
                        self.point(x, y);
                    }
                }
            }
        }
    }
}
