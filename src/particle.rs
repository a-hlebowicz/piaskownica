use std::cell::Cell;

#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
    Empty,
    Sand,
}
impl CellType {
    pub fn color(&self) -> (u8,u8,u8,u8) {
        match self {
            CellType::Empty => (1,1,1,1),
            CellType::Sand => (2,2,2,2), 
        }
    }
}

#[derive(Clone, Copy)]
pub struct Particle {
    pub cell_type: CellType,
}

impl Particle {
    pub fn new_empty() -> Particle {
        Particle {
            cell_type: CellType::Empty,
        }
    }
    pub fn new_sand() -> Particle {
        Particle {
            cell_type: CellType::Sand,
        }
    }
    
}

//dodac Clone, Copy i PartialEq