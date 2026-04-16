#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
    Empty,
    Sand,
}

#[derive(Clone, Copy)]
pub struct Particle {
    cell_type: CellType,
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