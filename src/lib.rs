use pyo3::prelude::*;
mod maze;
use maze::Maze;

#[pymodule]
fn dungeon_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Maze>()?;
    Ok(())
}
