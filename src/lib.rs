use pyo3::prelude::*;
pub mod maze; 
pub mod dungeon;
use crate::maze::Maze;
use crate::dungeon::Dungeon;

#[pymodule]
fn dungeon_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Maze>()?;
    m.add_class::<Dungeon>()?;
    m.add_class::<dungeon::Player>()?;
    Ok(())
}
