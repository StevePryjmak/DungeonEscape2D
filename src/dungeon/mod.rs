use crate::maze::Maze;
use crate::entity::Entity;
use pyo3::prelude::*;
use pyo3::Py;
use pyo3::Python;



#[pyclass]
#[derive(Clone, Debug)]
pub struct Dungeon {
    #[pyo3(get, set)]
    pub mazes: Vec<Vec<Maze>>, // 2D grid of maze rooms
    #[pyo3(get, set)]
    pub player: Entity, // Player entity in the dungeon
    #[pyo3(get, set)]
    pub current_room_row: usize,
    #[pyo3(get, set)]
    pub current_room_col: usize,
}


#[pymethods]
impl Dungeon {
    #[new]
    pub fn new(rows: usize, cols: usize, maze_width: usize, maze_height: usize, player: Entity) -> Self {
        let mut mazes = Vec::with_capacity(rows);
        for r in 0..rows {
            let mut row_vec = Vec::with_capacity(cols);
            for c in 0..cols {
                // Determine exits for this room
                let mut exits = Vec::new();

                // Check for outer rooms (on the edge)
                let on_top = r == 0;
                let on_bottom = r == rows - 1;
                let on_left = c == 0;
                let on_right = c == cols - 1;

                // Corners: 3 exits, one is either "left" or "right" based on position
                if (on_top && on_left) || (on_top && on_right) || (on_bottom && on_left) || (on_bottom && on_right) {
                    // Corner logic
                    if on_top && on_left {
                        exits.push("left".to_string());
                        exits.push("bottom".to_string());
                        exits.push("right".to_string()); // right is the only possible extra exit
                    } else if on_top && on_right {
                        exits.push("right".to_string());
                        exits.push("bottom".to_string());
                        exits.push("left".to_string()); // left is the only possible extra exit
                    } else if on_bottom && on_left {
                        exits.push("left".to_string());
                        exits.push("top".to_string());
                        exits.push("right".to_string()); // right is the only possible extra exit
                    } else if on_bottom && on_right {
                        exits.push("right".to_string());
                        exits.push("top".to_string());
                        exits.push("left".to_string()); // left is the only possible extra exit
                    }
                } else if on_top || on_bottom || on_left || on_right {
                    // Edge but not corner: 3 exits, no "final"
                    if on_top {
                        exits.push("left".to_string());
                        exits.push("right".to_string());
                        exits.push("bottom".to_string());
                    } else if on_bottom {
                        exits.push("left".to_string());
                        exits.push("right".to_string());
                        exits.push("top".to_string());
                    } else if on_left {
                        exits.push("top".to_string());
                        exits.push("bottom".to_string());
                        exits.push("right".to_string());
                    } else if on_right {
                        exits.push("top".to_string());
                        exits.push("bottom".to_string());
                        exits.push("left".to_string());
                    }
                } else {
                    // Inner room: 4 exits
                    exits.push("top".to_string());
                    exits.push("bottom".to_string());
                    exits.push("left".to_string());
                    exits.push("right".to_string());
                }

                let mut maze = Maze::new(maze_width, maze_height);
                // Pass exits to maze generation
                maze.generate_maze(Some(exits)).expect("Failed to generate maze");
                row_vec.push(maze);
            }
            mazes.push(row_vec);
        }
        Dungeon {
            mazes,
            player,
            current_room_row: rows / 2, // Start in the middle room
            current_room_col: cols / 2, // Start in the middle room
        }
    }

    /// Move player to another room (if possible)
    pub fn move_to_room(&mut self, row: usize, col: usize) -> PyResult<()> {
        if row < self.mazes.len() && col < self.mazes[0].len() {
            self.current_room_row = row;
            self.current_room_col = col;
            Ok(())
        } else {
            Err(pyo3::exceptions::PyIndexError::new_err("Invalid room coordinates"))
        }
    }

    pub fn move_player(&mut self, direction: &str) -> PyResult<bool> {
        let maze = &self.mazes[self.current_room_row][self.current_room_col];
        let (dx, dy, dir_idx) = match direction {
            "up" => (0isize, -1isize, 0),
            "right" => (1, 0, 1),
            "down" => (0, 1, 2),
            "left" => (-1, 0, 3),
            _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid direction")),
        };

        let new_x = self.player.x as isize + dx;
        let new_y = self.player.y as isize + dy;

        // Check if move is within current maze bounds
        if new_x >= 0 && new_x < maze.width as isize && new_y >= 0 && new_y < maze.height as isize {
            // Check if the move is not blocked by a wall
            if maze.can_move(self.player.y, self.player.x, dir_idx) {
                self.player.x = new_x as usize;
                self.player.y = new_y as usize;
                return Ok(true);
            } else {
                return Ok(false);
            }
        }

        // Handle moving to another room via exit
        let (next_room_row, next_room_col, next_x, next_y) = match direction {
            "up" if self.player.y == 0 && self.current_room_row > 0 => (
                self.current_room_row - 1,
                self.current_room_col,
                self.player.x,
                self.mazes[self.current_room_row - 1][self.current_room_col].height - 1,
            ),
            "down" if self.player.y + 1 == maze.height && self.current_room_row + 1 < self.mazes.len() => (
                self.current_room_row + 1,
                self.current_room_col,
                self.player.x,
                0,
            ),
            "left" if self.player.x == 0 && self.current_room_col > 0 => (
                self.current_room_row,
                self.current_room_col - 1,
                self.mazes[self.current_room_row][self.current_room_col - 1].width - 1,
                self.player.y,
            ),
            "right" if self.player.x + 1 == maze.width && self.current_room_col + 1 < self.mazes[0].len() => (
                self.current_room_row,
                self.current_room_col + 1,
                0,
                self.player.y,
            ),
            _ => return Ok(false),
        };

        // Check if exit is open in current maze
        if maze.can_move(self.player.y, self.player.x, dir_idx) {
            self.current_room_row = next_room_row;
            self.current_room_col = next_room_col;
            self.player.x = next_x;
            self.player.y = next_y;
            Ok(true)
        } else {
            Ok(false)
        }
    }
    /// Get the current maze room
    pub fn current_maze<'py>(&self, py: Python<'py>) -> PyResult<Py<Maze>> {
        let maze = self.mazes[self.current_room_row][self.current_room_col].clone();
        Py::new(py, maze)
    }
}