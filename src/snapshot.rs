use serde::{Serialize, Deserialize};
use crate::grid::Grid;
use crate::particle::{CellType, Particle};
use crate::snapshot;


#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub width: usize,
    pub height: usize,
    pub cell_types: Vec<String>,
    pub cell_temps: Vec<i16>,
}

pub fn export(grid: &Grid)  {
    let mut cell_types: Vec<String> =Vec::with_capacity(grid.height*grid.width);
    let mut cell_temps: Vec<i16> =Vec::with_capacity(grid.height*grid.width);
    for y in 0..grid.height{
        for x in 0..grid.width {
            let cell = grid.get(x, y);
            cell_types.push(cell.cell_type.as_str().to_string());
            cell_temps.push(cell.temperature);
        }
    }

    let snapshot = Snapshot{
        width: grid.width,
        height: grid.height,
        cell_types,
        cell_temps,
    };

    serde_json::to_string(&snapshot).unwrap_or_else(|_| "{}".to_string()); //jak blad to zwroc pusty json
}
