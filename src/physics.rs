use crate::grid::Grid;
use crate::particle::CellType;

pub fn tick(grid: &mut Grid) {
    for y in (0..grid.height).rev() {
        for x in 0..grid.width {
            if grid.get(x, y).has_moved { continue; }
            match grid.get(x, y).cell_type {
                CellType::Sand => update_sand(grid, x, y),
                _ => {}
            }
        }
    }
}

fn update_sand(grid: &mut Grid, x: usize, y: usize) {
    if y + 1 >= grid.height { return; }
    
    // dół
    if grid.get(x, y + 1).cell_type == CellType::Empty {
        grid.swap(x, y, x, y + 1);
        return;
    }

    // TODO: losowanie kolejności lewo/prawo
    let strona: bool = fastrand::bool();


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
    
}