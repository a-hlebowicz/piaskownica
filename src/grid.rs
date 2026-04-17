use std::vec;

use crate::particle::{Particle, CellType};

pub struct Grid {
    pub width: usize,
    pub height: usize,
    cells: Vec<Particle>,
    pixels: Vec<u8>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Grid{
        let total = width * height;
        let cells = vec![Particle::new_empty(); total];
        let pixels = vec![0u8; total*4]; //RGBA
        Grid {
            width,
            height,
            cells,
            pixels,
        }
    }
    pub fn index(&self, x:usize, y:usize) -> usize{
        y * self.width + x
    }

    pub fn get(&self, x:usize, y:usize) -> Particle{
        self.cells[self.index(x,y)]
    }

    pub fn set(&mut self, x:usize, y :usize, particle: Particle){
        let i = self.index(x,y);
        self.cells[i] = particle;
    }
    

    //pixele
    pub fn render(&mut self){
        for y in 0..self.height{
            for x in 0..self.width{
                let i = self.index(x, y);
                let particle =self.get(x, y);

                let (r, g, b, a) = particle.cell_type.color();
                let i4 =i*4; //tablica pikseli ma 4 pola na każdy pixel, wiec iterujemy co 4
                self.pixels[i4]=r;
                self.pixels[i4+1]=g;
                self.pixels[i4+2]=b;
                self.pixels[i4+3]=a;
            }
        }
    }

    pub fn pixels_ptr(&self) -> *const u8 {
        self.pixels.as_ptr()
    }
}

