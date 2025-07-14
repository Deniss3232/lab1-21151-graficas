use crate::punto::Punto;
use raylib::prelude::*;

pub struct Poligono {
    pub puntos: Vec<Punto>,
    pub color: Color,
}

impl Poligono {
    pub fn new(puntos: Vec<Punto>, color: Color) -> Self {
        Poligono { puntos, color }
    }

    pub fn dibujar(&self, d: &mut RaylibDrawHandle) {
        let puntos: Vec<Vector2> = self.puntos.iter()
            .map(|p| Vector2 { x: p.x, y: p.y })
            .collect();

        d.draw_triangle_fan(&puntos, self.color);
    }
}
