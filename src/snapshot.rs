use crate::grid::Grid;
use crate::particle::{CellType, Particle};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Snapshot {
    pub width: usize,
    pub height: usize,
    pub cell_types: Vec<String>,
    pub cell_temps: Vec<i16>,
}

pub fn export(grid: &Grid) -> String {
    let mut cell_types: Vec<String> = Vec::with_capacity(grid.height * grid.width);
    let mut cell_temps: Vec<i16> = Vec::with_capacity(grid.height * grid.width);
    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell = grid.get(x, y);
            cell_types.push(cell.cell_type.as_str().to_string());
            cell_temps.push(cell.temperature);
        }
    }

    let snapshot = Snapshot {
        width: grid.width,
        height: grid.height,
        cell_types,
        cell_temps,
    };

    serde_json::to_string(&snapshot).unwrap_or_else(|_| "{}".to_string()) //jak blad to zwroc pusty json
}

pub fn import(grid: &mut Grid, json: &str) -> Result<(), String> {
    let snapshot: Snapshot =
        serde_json::from_str(json).map_err(|e| format!("Błąd parsowania JSON: {}", e))?;

    if snapshot.width != grid.width || snapshot.height != grid.height {
        return Err(format!(
            "niezgodny rozmiar: plik ma {}x{}, oczekiwano {}x{}",
            snapshot.width, snapshot.height, grid.width, grid.height
        ));
    }

    if snapshot.cell_types.len() != grid.width * grid.height
        || snapshot.cell_temps.len() != grid.width * grid.height
    {
        return Err(format!(
            "Niezgodna liczba cząsteczek: plik ma {} i {} temperatur, oczekiwano {}",
            snapshot.cell_types.len(),
            snapshot.cell_temps.len(),
            grid.width * grid.height
        ));
    }

    for (i, type_str) in snapshot.cell_types.iter().enumerate() {
        let cell_type = CellType::from_str(type_str)?;
        let temperature = snapshot.cell_temps[i];
        let mut particle = Particle::new_empty();
        particle.cell_type = cell_type;
        particle.temperature = temperature;
        grid.cells[i] = particle;
    }

    Ok(())
}
