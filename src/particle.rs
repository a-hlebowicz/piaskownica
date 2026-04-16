
pub enum CellType {
    Empty,
    Sand,
}

pub struct Paricle {
    cell_type: CellType,
}

impl Particle {
    pub fn new_empty()-> Particle {
        Particle {
            cell_type: CellType::Empty,
        }
    }
    pub fn new_sand() -> Paricle {
        Particle {
            cell_type: CellType::Sand,
        }
    }
}