// --- Chest code ---
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub enum ChestContent {
    #[pyo3(name = "Gold")]
    Gold { amount: u32 },
    #[pyo3(name = "Sword")]
    Sword { },
    #[pyo3(name = "Shield")]
    Shield { },
    #[pyo3(name = "Potion")]
    Potion { },
    #[pyo3(name = "Key")]
    Key { },
}

#[pymethods]
impl ChestContent {
    #[staticmethod]
    pub fn gold(amount: u32) -> Self {
        ChestContent::Gold { amount }
    }
    #[staticmethod]
    pub fn sword() -> Self {
        ChestContent::Sword { }
    }
    #[staticmethod]
    pub fn shield() -> Self {
        ChestContent::Shield { }
    }
    #[staticmethod]
    pub fn potion() -> Self {
        ChestContent::Potion { }
    }
    #[staticmethod]
    pub fn key() -> Self {
        ChestContent::Key { }
    }
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct Chest {
    #[pyo3(get)]
    pub row: usize,
    #[pyo3(get)]
    pub col: usize,
    #[pyo3(get)]
    pub is_open: bool,
    #[pyo3(get)]
    pub contents: Option<ChestContent>,
}

#[pymethods]
impl Chest {
    #[new]
    pub fn new(row: usize, col: usize, contents: Option<ChestContent>) -> Self {
        Chest {
            row,
            col,
            is_open: false,
            contents,
        }
    }

    pub fn open(&mut self) -> Option<ChestContent> {
        if !self.is_open {
            self.is_open = true;
            self.contents.take()
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.contents.is_none()
    }
}
