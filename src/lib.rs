use pyo3::prelude::*;
pub mod maze; 
use crate::maze::Maze;

#[pymodule]
fn dungeon_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Maze>()?;
    Ok(())
}
