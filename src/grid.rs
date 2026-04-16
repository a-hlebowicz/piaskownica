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
}

