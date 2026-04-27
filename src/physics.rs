use crate::grid::Grid;
use crate::particle::{CellType, Particle};

pub fn tick(grid: &mut Grid) {
    let reverse_x = fastrand::bool();

    for y in (0..grid.height).rev() {
        if reverse_x {
            for x in (0..grid.width).rev() {
                update_cell(grid, x, y);
            }
        } else {
            for x in 0..grid.width {
                update_cell(grid, x, y);
            }
        }
    }
}

fn update_cell(grid: &mut Grid, x: usize, y: usize) {
    if grid.get(x, y).has_moved {
        return;
    }
    match grid.get(x, y).cell_type {
        CellType::Sand => update_sand(grid, x, y),
        CellType::Water => update_water(grid, x, y),
        CellType::Lava => update_lava(grid, x, y),
        CellType::Steam => update_steam(grid, x, y),
        CellType::Fire => update_fire(grid, x, y),
        _ => {}
    }
}

fn update_sand(grid: &mut Grid, x: usize, y: usize) {
    if y + 1 >= grid.height {
        return;
    }

    let lighter_than = [
        CellType::Empty,
        CellType::Water,
        CellType::Lava,
        CellType::Steam,
    ]; //nie wiem jak to ładniej nazwać

    if try_swap(grid, x, y, DOWN, &lighter_than) {
        return;
    }

    let mut candidates: Vec<(i32, i32)> = Vec::new();

    // ukos lewo
    if can_swap_diagonal(grid, x, y, DOWN_LEFT, &lighter_than) {
        candidates.push(DOWN_LEFT);
    }

    // ukos prawo
    if can_swap_diagonal(grid, x, y, DOWN_RIGHT, &lighter_than) {
        candidates.push(DOWN_RIGHT);
    }

    if let Some(&dir) = fastrand::choice(&candidates) {
        try_swap(grid, x, y, dir, &lighter_than);
    }
}

fn update_water(grid: &mut Grid, x: usize, y: usize) {
    update_liquid(grid, x, y);
}

fn update_lava(grid: &mut Grid, x: usize, y: usize) {
    update_liquid(grid, x, y);
}

fn update_steam(grid: &mut Grid, x: usize, y: usize) {
    update_gas(grid, x, y);
}

fn update_fire(grid: &mut Grid, x: usize, y: usize) {
    //fn update_fire(grid: &mut Grid, x: usize, y: usize) {
    let lighter_than = [CellType::Empty];

    let mut candidates: Vec<(i32, i32)> = Vec::new();
    
    candidates.push((0,0));
    candidates.push((0,0));
    candidates.push((0,0));

    if can_swap(grid, x, y, UP, &lighter_than) { candidates.push(UP); }
    if can_swap_diagonal(grid, x, y, UP_LEFT, &lighter_than) { candidates.push(UP_LEFT); }
    if can_swap_diagonal(grid, x, y, UP_RIGHT, &lighter_than) { candidates.push(UP_RIGHT); }
    if can_swap_diagonal(grid, x, y, DOWN_LEFT, &lighter_than) { candidates.push(DOWN_LEFT); }
    if can_swap_diagonal(grid, x, y, DOWN_RIGHT, &lighter_than) { candidates.push(DOWN_RIGHT); }
    if can_swap(grid, x, y, LEFT, &lighter_than) { candidates.push(LEFT); }
    if can_swap(grid, x, y, RIGHT, &lighter_than) { candidates.push(RIGHT); }

    if let Some(&dir) = fastrand::choice(&candidates) {
        try_swap(grid, x, y, dir, &lighter_than);
    }
}

fn update_liquid(grid: &mut Grid, x: usize, y: usize) {
    if y + 1 >= grid.height {
        return;
    }
    let lighter_than = [CellType::Empty, CellType::Steam];
    // dół
    if try_swap(grid, x, y, DOWN, &lighter_than) {
        return;
    }

    let mut candidates: Vec<(i32, i32)> = Vec::new();
    if can_swap_diagonal(grid, x, y, DOWN_LEFT, &lighter_than) {
        candidates.push(DOWN_LEFT);
    }
    if can_swap_diagonal(grid, x, y, DOWN_RIGHT, &lighter_than) {
        candidates.push(DOWN_RIGHT);
    }
    if let Some(&dir) = fastrand::choice(&candidates) {
        try_swap(grid, x, y, dir, &lighter_than);
        return;
    }

    candidates.clear();
    if can_swap(grid, x, y, LEFT, &lighter_than) {
        candidates.push(LEFT);
    }
    if can_swap(grid, x, y, RIGHT, &lighter_than) {
        candidates.push(RIGHT);
    }
    if let Some(&dir) = fastrand::choice(&candidates) {
        try_swap(grid, x, y, dir, &lighter_than);
    }
}

fn update_gas(grid: &mut Grid, x: usize, y: usize) {
    if y == 0 {
        return;
    }
    let lighter_than = [CellType::Empty];

    if try_swap(grid, x, y, UP, &lighter_than) {
        return;
    }

    let mut candidates: Vec<(i32, i32)> = Vec::new();
    if can_swap_diagonal(grid, x, y, UP_LEFT, &lighter_than) {
        candidates.push(UP_LEFT);
    }
    if can_swap_diagonal(grid, x, y, UP_RIGHT, &lighter_than) {
        candidates.push(UP_RIGHT);
    }
    if let Some(&dir) = fastrand::choice(&candidates) {
        try_swap(grid, x, y, dir, &lighter_than);
        return;
    }

    candidates.clear();
    if can_swap(grid, x, y, LEFT, &lighter_than) {
        candidates.push(LEFT);
    }
    if can_swap(grid, x, y, RIGHT, &lighter_than) {
        candidates.push(RIGHT);
    }
    if let Some(&dir) = fastrand::choice(&candidates) {
        try_swap(grid, x, y, dir, &lighter_than);
    }
}

fn can_swap(grid: &Grid, x: usize, y: usize, dir: (i32, i32), types: &[CellType]) -> bool {
    let (dx, dy) = dir;
    for &t in types {
        if grid.is_type_at(x, y, dx, dy, t) {
            return true;
        }
    }
    false
}
fn can_swap_diagonal(grid: &Grid, x: usize, y: usize, dir: (i32, i32), types: &[CellType]) -> bool {
    if !can_swap(grid, x, y, dir, types) {
        return false;
    }
    let (dx, dy) = dir;
    let side = (dx, 0);
    let vertical = (0, dy);
    can_swap(grid, x, y, side, types) || can_swap(grid, x, y, vertical, types)
}
fn try_swap(grid: &mut Grid, x: usize, y: usize, dir: (i32, i32), types: &[CellType]) -> bool {
    if can_swap(grid, x, y, dir, types) {
        let (dx, dy) = dir;
        let dx = (x as i32 + dx) as usize;
        let dy = (y as i32 + dy) as usize;
        grid.swap(x, y, dx, dy);
        true
    } else {
        false
    }
}

//fn try_move(grid: &mut Grid, x: usize, y: usize, dx: i32, dy: i32) -> bool {
//    try_swap(grid, x, y, dx, dy, &[CellType::Empty])
//}

const DOWN: (i32, i32) = (0, 1);
const DOWN_LEFT: (i32, i32) = (-1, 1);
const DOWN_RIGHT: (i32, i32) = (1, 1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);
const UP: (i32, i32) = (0, -1);
const UP_LEFT: (i32, i32) = (-1, -1);
const UP_RIGHT: (i32, i32) = (1, -1);

pub fn propagate_heat(grid: &mut Grid) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell = grid.get(x, y);

            if cell.cell_type == CellType::Empty {
                let i = grid.index(x, y);
                grid.temperatures_next[i] = cell.temperature;
                continue;
            }

            let total_flow = total_flow(grid, x, y, cell);
            let mut new_temp = cell.temperature as i32 + total_flow;
            new_temp = apply_passive_cooling(cell.cell_type, new_temp);

            let i = grid.index(x, y);
            grid.temperatures_next[i] = new_temp as i16;
        }
    }

    for i in 0..grid.cells.len() {
        grid.cells[i].temperature = grid.temperatures_next[i];
    }
}
fn total_flow(grid: &Grid, x: usize, y: usize, cell: Particle) -> i32 {
    let cell_temp = cell.temperature as i32;
    let cell_cond = cell.cell_type.conductivity() as i32;
    let mut total = 0;

    for (dx, dy, is_diagonal) in NEIGHBORS {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if !grid.in_bounds_i32(nx, ny) {
            continue;
        }
        let ncell = grid.get(nx as usize, ny as usize);
        if ncell.cell_type == CellType::Empty {
            continue;
        }

        total += flow(
            cell_temp,
            cell_cond,
            ncell.temperature as i32,
            ncell.cell_type.conductivity() as i32,
            is_diagonal,
        );
    }
    total
}
fn flow(cell_temp: i32, cell_cond: i32, n_temp: i32, n_cond: i32, is_diagonal: bool) -> i32 {
    let temp_diff = n_temp - cell_temp;
    let divisor = if is_diagonal {
        FLOW_DIVISOR * 2
    } else {
        FLOW_DIVISOR
    };
    let exchange_rate = (cell_cond * n_cond) / divisor;
    let flow = temp_diff * exchange_rate / 100;
    if flow == 0 && temp_diff != 0 {
        temp_diff.signum()
    } else {
        flow
    }
}
const NEIGHBORS: [(i32, i32, bool); 8] = [
    (-1, 0, false),
    (1, 0, false),
    (0, -1, false),
    (0, 1, false),
    (-1, -1, true),
    (1, -1, true),
    (-1, 1, true),
    (1, 1, true),
];
const FLOW_DIVISOR: i32 = 3000;

fn apply_passive_cooling(cell_type: CellType, temp: i32) -> i32 {
    match cell_type {
        CellType::Fire => {
            temp - fastrand::i32(1..5)
        },
        _ => temp,
    }
}

pub fn apply_temperature_effects(grid: &mut Grid) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell = grid.get(x, y);
            let new_type = match (cell.cell_type, cell.temperature) {
                (CellType::Lava, t) if t < 800 => Some(CellType::Stone),
                (CellType::Ice, t) if t > 10 => Some(CellType::Water),
                (CellType::Water, t) if t < -10 => Some(CellType::Ice),
                (CellType::Water, t) if t > 110 => Some(CellType::Steam),
                (CellType::Steam, t) if t < 90 => Some(CellType::Water),
                (CellType::Fire, t) if t < 300 => Some(CellType::Empty),
                (CellType::Wood, t) if t > 50 => {
                    ignite_neighbors(grid, x, y);
                    Some(CellType::Fire)
                },
                _ => None,
            };
            if let Some(new) = new_type {
                let mut new_cell = match new {
                    CellType::Stone => Particle::new_stone(),
                    CellType::Water => Particle::new_water(),
                    CellType::Ice => Particle::new_ice(),
                    CellType::Steam => Particle::new_steam(),
                    CellType::Fire => Particle::new_fire(),
                    _ => Particle::new_empty(),
                };
                if !matches!(new, CellType::Fire) {
                    new_cell.temperature = cell.temperature;
                }
                grid.set(x, y, new_cell);
            }
        }
    }
}

fn ignite_neighbors(grid: &mut Grid, x: usize, y: usize) {
    for (dx, dy, _) in NEIGHBORS {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if !grid.in_bounds_i32(nx, ny) { continue; }
        let neighbor = grid.get(nx as usize, ny as usize);
        if neighbor.cell_type == CellType::Empty && fastrand::f32() < 0.5 {
            grid.set(nx as usize, ny as usize, Particle::new_fire());
        }
    }
}
