mod grid;
mod particle;
mod physics;

use crate::grid::Grid;
use crate::particle::Particle;
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
        self.grid.resed_moved_flags();
        physics::tick(&mut self.grid);
        physics::propagate_heat(&mut self.grid);
        physics::apply_temperature_effects(&mut self.grid);
        self.grid.render();
    }

    pub fn draw(&mut self, x: usize, y: usize, material: u8) {
        if self.grid.in_bounds(x, y) {
            let particle = material_to_particle(material);
            self.grid.set(x, y, particle);
        } else {
            log!("wartosc poza zakresem?");
        }
    }

    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, material: u8) {
        let particle = material_to_particle(material);
        self.grid.draw_line(x1, y1, x2, y2, particle);
    }

    pub fn clear(&mut self) {
        self.grid.clear();
    }

    pub fn width(&self) -> usize {
        self.grid.width
    }
    pub fn height(&self) -> usize {
        self.grid.height
    }

    pub fn pixels_ptr(&self) -> *const u8 {
        self.grid.pixels_ptr()
    }
    pub fn debug_at(&self, x: usize, y: usize) -> i16 {
    if self.grid.in_bounds(x, y) {
        self.grid.get(x, y).temperature
    } else {
        0
    }
}
}
fn material_to_particle(material: u8) -> Particle {
    match material {
        1 => Particle::new_sand(),
        2 => Particle::new_water(),
        3 => Particle::new_stone(),
        4 => Particle::new_wood(),
        5 => Particle::new_lava(),
        6 => Particle::new_metal(),
        7 => Particle::new_ice(),
        _ => Particle::new_empty(),
    }
}
