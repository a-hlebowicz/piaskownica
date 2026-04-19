use std::vec;

use crate::particle::{CellType, Particle};

pub struct Grid {
    pub width: usize,
    pub height: usize,
    cells: Vec<Particle>,
    pixels: Vec<u8>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid {
        let total = width * height;
        let cells = vec![Particle::new_empty(); total];
        let pixels = vec![0u8; total * 4]; //RGBA
        Grid {
            width,
            height,
            cells,
            pixels,
        }
    }
    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn get(&self, x: usize, y: usize) -> Particle {
        self.cells[self.index(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, particle: Particle) {
        let i = self.index(x, y);
        self.cells[i] = particle;
    }

    pub fn swap(&mut self, x1:usize, y1:usize, x2:usize, y2:usize){
        let tmp = self.get(1, y1);
        self.set(x1, y1, self.get(2, y2));
        self.set(x2, y2, tmp);
    }

    pub fn clear(&mut self) {
        for cell in self.cells.iter_mut() {
            *cell = Particle::new_empty();
        }
    }

    pub fn resed_moved_flags(&mut self){
        for cell in self.cells.iter_mut() {
            cell.has_moved=false;
        }
    }

    pub fn in_bounds(&self,x:usize, y:usize) -> bool{
        x < self.width && y < self.height
    }

    //pixele
    pub fn render(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let i = self.index(x, y);
                let particle = self.get(x, y);

                let (r, g, b, a) = particle.cell_type.color();
                let i4 = i * 4; //tablica pikseli ma 4 pola na każdy pixel, wiec iterujemy co 4
                self.pixels[i4] = r;
                self.pixels[i4 + 1] = g;
                self.pixels[i4 + 2] = b;
                self.pixels[i4 + 3] = a;
            }
        }
    }

    pub fn pixels_ptr(&self) -> *const u8 {
        self.pixels.as_ptr()
    }

    //Algorytm Bresenhama
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, particle: Particle) {
        let (mut x, mut y) = (x1, y1);

        // ustalenie kierunku i różnic
        let (dx, xi) = if x1 < x2 { (x2 - x1, 1) } else { (x1 - x2, -1) };
        let (dy, yi) = if y1 < y2 { (y2 - y1, 1) } else { (y1 - y2, -1) };

    // pomocnicza funkcja do ustawienia piksela z bounds check
        let set_pixel = |grid: &mut Grid, x: i32, y: i32| {
            if x >= 0 && y >= 0 && (x as usize) < grid.width && (y as usize) < grid.height {
                grid.set(x as usize, y as usize, particle);
            }
        };

        set_pixel(self, x, y);

        if dx > dy {
            // oś wiodąca: X
            let ai = (dy - dx) * 2;
            let bi = dy * 2;
            let mut d = bi - dx;

            while x != x2 {
                if d >= 0 {
                    x += xi;
                    y += yi;
                    d += ai;
                } else {
                    d += bi;
                    x += xi;
                }
                set_pixel(self, x, y);
            }
        } else {
            // oś wiodąca: Y
            let ai = (dx - dy) * 2;
            let bi = dx * 2;
            let mut d = bi - dy;

            while y != y2 {
                if d >= 0 {
                    x += xi;
                    y += yi;
                    d += ai;
                } else {
                    d += bi;
                    y += yi;
                }
                set_pixel(self, x, y);
            }
        }
    }
}
