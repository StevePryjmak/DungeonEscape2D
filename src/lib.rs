use pyo3::prelude::*;
pub mod maze; 
pub mod dungeon;
pub mod entity;
use crate::maze::Maze;
use crate::dungeon::Dungeon;
use crate::entity::Entity;

#[pymodule]
fn dungeon_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Maze>()?;
    m.add_class::<Dungeon>()?;
    m.add_class::<Entity>()?;
    Ok(())
}
