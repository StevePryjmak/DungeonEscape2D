use pyo3::prelude::*;

/// Represents the player in the dungeon.
#[pyclass]
#[derive(Clone, Debug)]
pub struct Player {
    #[pyo3(get, set)]
    pub x: usize,
    #[pyo3(get, set)]
    pub y: usize,
    #[pyo3(get, set)]
    pub health: i32,
}

#[pymethods]
impl Player {
    #[new]
    pub fn new(x: usize, y: usize, health: i32) -> Self {
        Player { x, y, health }
    }

    pub fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }
}
