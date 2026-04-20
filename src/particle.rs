#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
    Empty,
    Sand,
    Water,
}
impl CellType {
    pub fn color(&self) -> (u8, u8, u8, u8) {
        match self {
            CellType::Empty => (40, 40, 40, 255),
            CellType::Sand => (255, 228, 0, 255),
            CellType::Water => (47, 190, 255, 255),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Particle {
    pub cell_type: CellType,
    pub has_moved: bool,
    pub temperature: i16,
}

impl Particle {
    pub fn new_empty() -> Particle {
        Particle {
            cell_type: CellType::Empty,
            has_moved: false,
            temperature: 20,
        }
    }
    pub fn new_sand() -> Particle {
        Particle {
            cell_type: CellType::Sand,
            has_moved: false,
            temperature: 20,
        }
    }
    pub fn new_water() -> Particle {
        Particle {
            cell_type: CellType::Water,
            has_moved: false,
            temperature: 20,
        }
    }
}
