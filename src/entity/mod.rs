use pyo3::prelude::*;

/// Represents an entity in the dungeon (player or enemy).
#[pyclass]
#[derive(Clone, Debug)]
pub struct Entity {
    #[pyo3(get, set)]
    pub x: usize,
    #[pyo3(get, set)]
    pub y: usize,
    #[pyo3(get, set)]
    pub health: i32,
    #[pyo3(get, set)]
    pub armor: i32,
    #[pyo3(get, set)]
    pub attack: i32,
    #[pyo3(get, set)]
    pub gold: i32,
    #[pyo3(get, set)]
    pub is_player: bool, // true for player, false for enemy
}

#[pymethods]
impl Entity {
    #[new]
    pub fn new(
        x: usize,
        y: usize,
        health: i32,
        armor: i32,
        attack: i32,
        gold: i32,
        is_player: bool,
    ) -> Self {
        Entity { x, y, health, armor, attack, gold, is_player }
    }

    pub fn move_to(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn set_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;
    }

    pub fn take_damage(&mut self, amount: i32) {
        let reduced_amount = (amount - self.armor).max(0);
        self.health -= reduced_amount;
    }

}
