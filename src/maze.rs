use pyo3::prelude::*;

#[pyclass]
pub struct Maze {
    
}

#[pymethods]
impl Maze {
    #[new]
    pub fn new() -> Self {
        Maze {}
    }

    pub fn greet(&self) {
        println!("Hello from Rust Maze!");
    }
}
