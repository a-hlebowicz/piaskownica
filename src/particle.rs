#[derive(Clone, Copy, PartialEq)]
pub enum CellType {
    Empty,
    Sand,
    Water,
    Stone,
    Wood,
    Lava,
    Metal,
    Ice,
    Steam,
    Fire,
}
impl CellType {
    pub fn color(&self, temperature: i16) -> (u8, u8, u8, u8) {
        let color = self.base_color();
        if *self == CellType::Metal {
            return metal_glow(color, temperature);
        }
        color
    }
    pub fn base_color(&self) -> (u8, u8, u8, u8) {
        match self {
            CellType::Empty => (40, 40, 40, 255),
            CellType::Sand => (255, 228, 0, 255),
            CellType::Water => (47, 190, 255, 255),
            CellType::Stone => (171, 164, 164, 255),
            CellType::Wood => (133, 74, 30, 255),
            CellType::Lava => (255, 104, 0, 255),
            CellType::Metal => (140, 140, 150, 255),
            CellType::Ice => (120, 255, 255, 255),
            CellType::Steam => (200, 210, 220, 200),
            CellType::Fire => (255, 140, 30, 255),
        }
    }

    pub fn conductivity(&self) -> u8 {
        match self {
            CellType::Empty => 30,
            CellType::Sand => 30,
            CellType::Water => 100,
            CellType::Stone => 40,
            CellType::Wood => 10,
            CellType::Lava => 100,
            CellType::Metal => 250,
            CellType::Ice => 40,
            CellType::Steam => 30,
            CellType::Fire => 80,
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
            temperature: 1500,
        }
    }
    pub fn new_metal() -> Particle {
        Particle {
            cell_type: CellType::Metal,
            has_moved: false,
            temperature: 20,
        }
    }
    pub fn new_ice() -> Particle {
        Particle {
            cell_type: CellType::Ice,
            has_moved: false,
            temperature: -100,
        }
    }
    pub fn new_steam() -> Particle {
        Particle {
            cell_type: CellType::Steam,
            has_moved: false,
            temperature: 110,
        }
    }
    pub fn new_fire() -> Particle {
        Particle {
            cell_type: CellType::Fire,
            has_moved: false,
            temperature: 600,
        }
    }
}

fn metal_glow(base: (u8, u8, u8, u8), temp: i16) -> (u8, u8, u8, u8) {
    if temp < 100 {
        return base;
    }

    let intensity = ((temp - 100).max(0) as u32 * 255 / 900).min(255) as u8;

    // dodaj czerwony, zmniejsz zielony i niebieski
    let r = base.0.saturating_add(intensity);
    let g = base.1.saturating_sub(intensity / 2);
    let b = base.2.saturating_sub(intensity / 2);

    (r, g, b, 255)
}
