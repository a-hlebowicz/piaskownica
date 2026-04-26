#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
    Empty,
    Sand,
    Water,
    Stone,
    Wood,
    Lava,
    Metal,
}
impl CellType {
    pub fn color(&self) -> (u8, u8, u8, u8) {
        match self {
            CellType::Empty => (40, 40, 40, 255),
            CellType::Sand => (255, 228, 0, 255),
            CellType::Water => (47, 190, 255, 255),
            CellType::Stone => (171, 164, 164, 255),
            CellType::Wood => (133, 74, 30, 255),
            CellType::Lava => (255, 104, 0, 255),
            CellType::Metal => (140, 140, 150, 255),
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
    pub fn new_stone() -> Particle {
        Particle {
            cell_type: CellType::Stone,
            has_moved: false,
            temperature: 20,
        }
    }
    pub fn new_wood() -> Particle {
        Particle {
            cell_type: CellType::Wood,
            has_moved: false,
            temperature: 20,
        }
    }
    pub fn new_lava() -> Particle {
        Particle {
            cell_type: CellType::Lava,
            has_moved: false,
            temperature: 1000,
        }
    }
    pub fn new_metal() -> Particle{
        Particle { 
            cell_type: CellType::Metal, 
            has_moved: false, 
            temperature: 20, 
        }
    }
}
