use crate::grid::Grid;
use crate::particle::CellType;

pub fn tick(grid: &mut Grid) {
    for y in (0..grid.height).rev() {
        for x in 0..grid.width {
            if grid.get(x, y).has_moved {
                continue;
            }
            match grid.get(x, y).cell_type {
                CellType::Sand => update_sand(grid, x, y),
                CellType::Water => update_water(grid,x,y),
                _ => {}
            }
        }
    }
}

fn update_sand(grid: &mut Grid, x: usize, y: usize) {
    if y + 1 >= grid.height { return; }

    let lighter_than = [CellType::Empty, CellType::Water]; //nie wiem jak to ładniej nazwać
    
    if try_swap(grid, x, y, 0, 1, &lighter_than) { return; }

    let mut candidates: Vec<(i32, i32)> = Vec::new();

    // ukos lewo
    if can_swap(grid, x, y, -1, 1,&lighter_than) { candidates.push((-1, 1)); }

    // ukos prawo
    if can_swap(grid, x, y, 1, 1,&lighter_than) { candidates.push((1, 1)); }

    if let Some(&(dx, dy)) = fastrand::choice(&candidates) {
        try_swap(grid, x, y, dx, dy, &lighter_than);
    }
}

fn update_water(grid: &mut Grid,x: usize,y: usize){
    if y + 1 >= grid.height { return; }
    let lighter_than = [CellType::Empty];
    // dół
    if try_swap(grid, x, y, 0, 1, &lighter_than) { return; }

    let mut candidates: Vec<(i32, i32)> = Vec::new();
    if can_swap(grid, x, y, -1, 0,&lighter_than) { candidates.push((-1, 0)); }
    if can_swap(grid, x, y, 1, 0,&lighter_than) { candidates.push((1, 0)); }
    if let Some(&(dx, dy)) = fastrand::choice(&candidates) {
        try_swap(grid, x, y, dx, dy, &lighter_than);
    }

    let mut candidates: Vec<(i32, i32)> = Vec::new();
    if can_swap(grid, x, y, -1, 1,&lighter_than) { candidates.push((-1, 1)); }
    if can_swap(grid, x, y, 1, 1,&lighter_than) { candidates.push((1, 1)); }
    if let Some(&(dx, dy)) = fastrand::choice(&candidates) {
        try_swap(grid, x, y, dx, dy, &lighter_than);
    }


}

fn can_move(grid: &Grid, x: usize, y: usize, dx: i32, dy: i32) -> bool {
    grid.is_type_at(x, y, dx, dy, CellType::Empty)
}
fn can_swap(grid: &Grid, x: usize, y: usize, dx: i32, dy: i32, types: &[CellType]) -> bool {
    for &t in types {
        if grid.is_type_at(x, y, dx, dy, t) { return true; }
    }
    false
}
fn try_swap(grid: &mut Grid, x: usize, y: usize, dx: i32, dy: i32, types: &[CellType]) -> bool {
    if can_swap(grid,x,y,dx,dy,types) {
        let dx = (x as i32 + dx) as usize;
        let dy = (y as i32 + dy) as usize;
        grid.swap(x, y, dx, dy);
        true
    } else {
        false
    }
}

fn try_move(grid: &mut Grid, x: usize, y: usize, dx: i32, dy: i32) -> bool {
    try_swap(grid, x, y, dx, dy, &[CellType::Empty])
}
