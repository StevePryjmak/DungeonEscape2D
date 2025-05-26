use pyo3::types::PyTuple;
use cell::{Cell, Direction};

use pyo3::prelude::*;
pub mod cell;

#[pyclass]
pub struct Maze {
    width: usize,
    height: usize,
    grid: Vec<Vec<Cell>>,
}

#[pymethods]
impl Maze {
    #[new]
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Cell::new(); width]; height];
        Maze { width, height, grid }
    }

    pub fn greet(&self) {
        println!("Hello from Rust Maze!");
    }

    pub fn get_cell_walls(&self, row: usize, col: usize) -> PyResult<(bool, bool, bool, bool)> {
        if row >= self.height || col >= self.width {
            return Err(pyo3::exceptions::PyIndexError::new_err("Cell out of bounds"));
        }
        let cell = &self.grid[row][col];
        Ok((cell.walls[0], cell.walls[1], cell.walls[2], cell.walls[3]))
    }

    pub fn set_cell_visited(&mut self, row: usize, col: usize) -> PyResult<()> {
        if row >= self.height || col >= self.width {
            return Err(pyo3::exceptions::PyIndexError::new_err("Cell out of bounds"));
        }
        self.grid[row][col].set_visited();
        Ok(())
    }

    pub fn remove_wall(&mut self, row: usize, col: usize, dir: usize) -> PyResult<()> {
        if row >= self.height || col >= self.width || dir > 3 {
            return Err(pyo3::exceptions::PyIndexError::new_err("Invalid cell or direction"));
        }
        self.grid[row][col].remove_wall(match dir {
            0 => Direction::Top,
            1 => Direction::Right,
            2 => Direction::Bottom,
            3 => Direction::Left,
            _ => unreachable!(),
        });
        Ok(())
    }
}
