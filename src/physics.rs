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
    if y + 1 >= grid.height {
        return;
    }
    let passed_types = [CellType::Empty, CellType::Water];
    // dół
    //if try_move(grid, x, y, 0, 1) { return; }
    // czy pod jest woda
    if try_swap(grid, x, y, 0, 1, &passed_types) { return; }

    let mut candidates: Vec<(i32, i32)> = Vec::new();

    // ukos lewo
    if can_swap(grid, x, y, -1, 1,&passed_types) { candidates.push((-1, 1)); }

    // ukos prawo
    if can_swap(grid, x, y, 1, 1,&passed_types) { candidates.push((1, 1)); }

    if let Some(&(dx, dy)) = fastrand::choice(&candidates) {
        try_swap(grid, x, y, dx, dy, &passed_types);
    }
}

fn update_water(grid: &mut Grid,x: usize,y: usize){
    if y + 1 >= grid.height {
        return;
    }
    // dół
    if grid.get(x, y + 1).cell_type == CellType::Empty {
        grid.swap(x, y, x, y + 1);
        return;
    }

    let strona: bool = fastrand::bool();


     if x > 0 && grid.get(x - 1, y + 1).cell_type == CellType::Empty {
        //zapisz do mozliwych ruchów

     }











    // ukos lewo
    if strona && x > 0 && grid.get(x - 1, y + 1).cell_type == CellType::Empty {
        grid.swap(x, y, x - 1, y + 1);
        return;
    }

    // ukos prawo
    if x + 1 < grid.width && grid.get(x + 1, y + 1).cell_type == CellType::Empty {
        grid.swap(x, y, x + 1, y + 1);
        return;
    }

    if strona && x > 0 && grid.get(x - 1, y).cell_type == CellType::Empty {
        grid.swap(x, y, x - 1, y);
        return;
    }

    // ukos prawo
    if x + 1 < grid.width && grid.get(x + 1, y ).cell_type == CellType::Empty {
        grid.swap(x, y, x + 1, y);
        return;
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
