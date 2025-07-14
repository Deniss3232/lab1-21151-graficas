
from PIL import Image, ImageDraw

class Punto:
    def __init__(self, x, y):
        self.x = x
        self.y = y

class Poligono:
    def __init__(self, puntos, color):
        self.puntos = puntos  # Lista de objetos Punto
        self.color = color    # Color en tupla RGB (por ejemplo: (255, 0, 0))

    def dibujar(self, draw: ImageDraw.ImageDraw):
        puntos_convertidos = [(p.x, p.y) for p in self.puntos]
        draw.polygon(puntos_convertidos, fill=self.color, outline="black")

def cargar_poligonos():
    return [
        Poligono([
            Punto(165, 380), Punto(185, 360), Punto(180, 330), Punto(207, 345),
            Punto(233, 330), Punto(230, 360), Punto(250, 380), Punto(220, 385),
            Punto(205, 410), Punto(193, 383)
        ], (0, 102, 255)),

        Poligono([
            Punto(321, 335), Punto(288, 286), Punto(339, 251), Punto(374, 302)
        ], (0, 200, 100)),

        Poligono([
            Punto(377, 249), Punto(411, 197), Punto(436, 249)
        ], (160, 50, 200)),

        Poligono([
            Punto(413, 177), Punto(448, 159), Punto(502, 88), Punto(553, 53),
            Punto(535, 36), Punto(676, 37), Punto(660, 52), Punto(759, 145),
            Punto(761, 179), Punto(672, 192), Punto(659, 214), Punto(615, 214),
            Punto(632, 230), Punto(580, 230), Punto(597, 215), Punto(552, 214),
            Punto(517, 144), Punto(466, 180)
        ], (180, 100, 0)),

        Poligono([
            Punto(682, 175), Punto(708, 120), Punto(735, 148), Punto(739, 170)
        ], (255, 255, 255))
    ]

def main():
    ancho, alto = 800, 600
    imagen = Image.new("RGB", (ancho, alto), "white")
    draw = ImageDraw.Draw(imagen)

    for poligono in cargar_poligonos():
        poligono.dibujar(draw)

    imagen.save("out.png")
    print("✅ Imagen guardada como out.png")

if __name__ == "__main__":
    main()
