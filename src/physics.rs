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
    
    if try_swap(grid, x, y, DOWN, &lighter_than) { return; }

    let mut candidates: Vec<(i32, i32)> = Vec::new();

    // ukos lewo
    if can_swap(grid, x, y, DOWN_LEFT,&lighter_than) { candidates.push(DOWN_LEFT); }

    // ukos prawo
    if can_swap(grid, x, y,DOWN_RIGHT,&lighter_than) { candidates.push(DOWN_RIGHT); }

    if let Some(&dir) = fastrand::choice(&candidates) {
        try_swap(grid, x, y, dir, &lighter_than);
    }
}

fn update_water(grid: &mut Grid,x: usize,y: usize){
    if y + 1 >= grid.height { return; }
    let lighter_than = [CellType::Empty];
    // dół
    if try_swap(grid, x, y, DOWN, &lighter_than) { return; }

    let mut candidates: Vec<(i32, i32)> = Vec::new();
    if can_swap(grid, x, y, LEFT,&lighter_than) { candidates.push(LEFT); }
    if can_swap(grid, x, y, RIGHT,&lighter_than) { candidates.push(RIGHT); }
    if let Some(&dir) = fastrand::choice(&candidates) {
        try_swap(grid, x, y, dir, &lighter_than);
        return;
    }

    candidates.clear();
    if can_swap(grid, x, y, DOWN_LEFT,&lighter_than) { candidates.push(DOWN_LEFT); }
    if can_swap(grid, x, y,DOWN_RIGHT,&lighter_than) { candidates.push(DOWN_RIGHT); }
    if let Some(&dir) = fastrand::choice(&candidates) {
        try_swap(grid, x, y, dir, &lighter_than);
    }


}
/*
fn can_move(grid: &Grid, x: usize, y: usize, dir: (i32, i32)) -> bool {
    let (dx, dy) = dir;
    grid.is_type_at(x, y, dx, dy, CellType::Empty)
}
    */
fn can_swap(grid: &Grid, x: usize, y: usize, dir: (i32, i32) , types: &[CellType]) -> bool {
    let (dx, dy) = dir;
    for &t in types {
        if grid.is_type_at(x, y, dx, dy, t) { return true; }
    }
    false
}
fn try_swap(grid: &mut Grid, x: usize, y: usize, dir: (i32, i32), types: &[CellType]) -> bool {
    if can_swap(grid,x,y,dir,types) {
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