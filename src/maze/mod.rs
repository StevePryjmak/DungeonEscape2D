
mod cell;
use pyo3::prelude::*;
use cell::{Cell, Direction};
use rand::seq::SliceRandom;
use rand::thread_rng;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Maze {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<Cell>>,
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
        if row >= self.height || col >= self.width {
            return Err(pyo3::exceptions::PyIndexError::new_err("Cell out of bounds"));
        }
        let direction = match dir {
            0 => Direction::Top,
            1 => Direction::Right,
            2 => Direction::Bottom,
            3 => Direction::Left,
            _ => return Err(pyo3::exceptions::PyIndexError::new_err("Invalid direction")),
        };
        self.grid[row][col].remove_wall(direction);
        Ok(())
    }

    pub fn generate_maze(&mut self, exits: Option<Vec<String>>) -> PyResult<()> {
        let exits = exits.unwrap_or_else(|| vec!["top".to_string(), "right".to_string(), "bottom".to_string(), "left".to_string()]);
        let mut rng = thread_rng();
        let mut stack = Vec::new();
        let mut visited = vec![vec![false; self.width]; self.height];

        stack.push((0, 0));
        visited[0][0] = true;

        while let Some((row, col)) = stack.pop() {
            let mut neighbors = Vec::new();

            if row > 0 && !visited[row - 1][col] {
                neighbors.push((row - 1, col, Direction::Top, Direction::Bottom));
            }
            if col + 1 < self.width && !visited[row][col + 1] {
                neighbors.push((row, col + 1, Direction::Right, Direction::Left));
            }
            if row + 1 < self.height && !visited[row + 1][col] {
                neighbors.push((row + 1, col, Direction::Bottom, Direction::Top));
            }
            if col > 0 && !visited[row][col - 1] {
                neighbors.push((row, col - 1, Direction::Left, Direction::Right));
            }

            if !neighbors.is_empty() {
                stack.push((row, col));
                let &(nrow, ncol, dir, opp_dir) = neighbors.choose(&mut rng).unwrap();
                self.grid[row][col].remove_wall(dir);
                self.grid[nrow][ncol].remove_wall(opp_dir);
                visited[nrow][ncol] = true;
                stack.push((nrow, ncol));
            }
        }

        self.add_exits(&exits)?;
        Ok(())
    }

    pub fn display(&self) {
        for row in 0..self.height {
            for col in 0..self.width {
                print!("{}", if self.grid[row][col].walls[0] { "+---" } else { "+   " });
            }
            println!("+");

            for col in 0..self.width {
                print!("{}", if self.grid[row][col].walls[3] { "|   " } else { "    " });
                if col == self.width - 1 {
                    print!("{}", if self.grid[row][col].walls[1] { "|" } else { " " });
                }
            }
            println!();
        }

        for col in 0..self.width {
            print!("{}", if self.grid[self.height - 1][col].walls[2] { "+---" } else { "+   " });
        }
        println!("+");
    }
}

// Move add_exits outside the #[pymethods] block
impl Maze {
    fn add_exits(&mut self, exits: &[String]) -> PyResult<()> {
        let mid_row = self.height / 2;
        let mid_col = self.width / 2;

        for exit in exits {
            match exit.as_str() {
                "top" => self.grid[0][mid_col].remove_wall(Direction::Top),
                "bottom" => self.grid[self.height - 1][mid_col].remove_wall(Direction::Bottom),
                "left" => self.grid[mid_row][0].remove_wall(Direction::Left),
                "right" => self.grid[mid_row][self.width - 1].remove_wall(Direction::Right),
                _ => return Err(pyo3::exceptions::PyValueError::new_err(format!("Invalid exit: {}", exit))),
            }
        }

        Ok(())
    }
    pub fn can_move(&self, row: usize, col: usize, dir: usize) -> bool {
        if row >= self.height || col >= self.width {
            return false;
        }
        let direction = match dir {
            0 => Direction::Top,
            1 => Direction::Right,
            2 => Direction::Bottom,
            3 => Direction::Left,
            _ => return false,
        };
        !self.grid[row][col].has_wall(direction)
    }
}