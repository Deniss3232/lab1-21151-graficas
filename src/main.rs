mod punto;
mod poligono;

use poligono::Poligono;
use punto::Punto;
use raylib::prelude::*;

fn cargar_poligonos() -> Vec<Poligono> {
    vec![
        Poligono::new(vec![
            Punto::new(165.0, 380.0), Punto::new(185.0, 360.0),
            Punto::new(180.0, 330.0), Punto::new(207.0, 345.0),
            Punto::new(233.0, 330.0), Punto::new(230.0, 360.0),
            Punto::new(250.0, 380.0), Punto::new(220.0, 385.0),
            Punto::new(205.0, 410.0), Punto::new(193.0, 383.0)
        ], Color::GREEN),
        Poligono::new(vec![
            Punto::new(321.0, 335.0), Punto::new(288.0, 286.0),
            Punto::new(339.0, 251.0), Punto::new(374.0, 302.0)
        ], Color::BLUE),
        Poligono::new(vec![
            Punto::new(377.0, 249.0), Punto::new(411.0, 197.0),
            Punto::new(436.0, 249.0)
        ], Color::PURPLE),
        Poligono::new(vec![
            Punto::new(413.0, 177.0), Punto::new(448.0, 159.0),
            Punto::new(502.0, 88.0), Punto::new(553.0, 53.0),
            Punto::new(535.0, 36.0), Punto::new(676.0, 37.0),
            Punto::new(660.0, 52.0), Punto::new(759.0, 145.0),
            Punto::new(761.0, 179.0), Punto::new(672.0, 192.0),
            Punto::new(659.0, 214.0), Punto::new(615.0, 214.0),
            Punto::new(632.0, 230.0), Punto::new(580.0, 230.0),
            Punto::new(597.0, 215.0), Punto::new(552.0, 214.0),
            Punto::new(517.0, 144.0), Punto::new(466.0, 180.0)
        ], Color::BROWN),
        Poligono::new(vec![
            Punto::new(682.0, 175.0), Punto::new(708.0, 120.0),
            Punto::new(735.0, 148.0), Punto::new(739.0, 170.0)
        ], Color::WHITE) // agujero blanco
    ]
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Polígonos Rellenos")
        .build();

    rl.set_target_fps(60);

    let poligonos = cargar_poligonos();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        for poligono in &poligonos {
            poligono.dibujar(&mut d);
        }
    }
}
