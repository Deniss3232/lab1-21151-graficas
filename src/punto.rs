#[derive(Copy, Clone)]
pub struct Punto {
    pub x: f32,
    pub y: f32,
}

impl Punto {
    pub fn new(x: f32, y: f32) -> Self {
        Punto { x, y }
    }
}
