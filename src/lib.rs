use pyo3::prelude::*;
pub mod maze; 
pub mod dungeon;
pub mod player;
use crate::maze::Maze;
use crate::dungeon::Dungeon;
use crate::player::Player;

#[pymodule]
fn dungeon_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Maze>()?;
    m.add_class::<Dungeon>()?;
    m.add_class::<Player>()?;
    Ok(())
}
