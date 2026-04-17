mod grid;
mod particle;

use crate::grid::Grid;
use crate::particle::{CellType, Particle};
use wasm_bindgen::prelude::*;

//wypisywanie do przeglądarki
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!($($t)*).into())
    };
}

#[wasm_bindgen]
pub fn hello() {
    log!("hello world");
}

#[wasm_bindgen]
pub struct Universe {
    grid: Grid,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: usize, height: usize) -> Universe {
        console_error_panic_hook::set_once(); //do zwracania bledow w js
        Universe {
            grid: Grid::new(width, height),
        }
    }

    pub fn tick(&mut self) {
        self.grid.render();
    }

    pub fn draw(&mut self, x: usize, y: usize, material: u8){
        if self.grid.in_bounds(x,y){
                let particle= match material {
                1=> Particle::new_sand(),
                _=> Particle::new_empty(),
            };
            self.grid.set(x, y, particle);
        }
        else {
            log!("wartosc poza zakresem?")
        }
        
    }

    pub fn clear(&mut self){
        self.grid.clear();
    }

    pub fn width(&self) ->usize{
        self.grid.width
    }
    pub fn height(&self) ->usize{
        self.grid.height
    }

    pub fn pixels_ptr(&self)-> *const u8{
        self.grid.pixels_ptr()
    }

}
