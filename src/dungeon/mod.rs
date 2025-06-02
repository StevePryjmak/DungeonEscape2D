use crate::maze::Maze;
use crate::entity::Entity;
use pyo3::prelude::*;
use pyo3::Py;
use pyo3::Python;
use rand::Rng; // Add rand = "0.8" to Cargo.toml
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Dungeon {
    #[pyo3(get, set)]
    pub mazes: Vec<Vec<Maze>>,
    #[pyo3(get, set)]
    pub player: Entity,
    #[pyo3(get, set)]
    pub current_room_row: usize,
    #[pyo3(get, set)]
    pub current_room_col: usize,
    #[pyo3(get, set)]
    pub enemies: Vec<Entity>, // Add this field
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
            current_room_row: rows / 2,
            current_room_col: cols / 2,
            enemies: Vec::new(),
        }
    }

    /// Call this when entering a new room
    pub fn spawn_enemies(&mut self, count: usize) {
        let maze = &self.mazes[self.current_room_row][self.current_room_col];
        let mut rng = rand::thread_rng();
        self.enemies.clear();
        for _ in 0..count {
            // Random position not on player
            let mut x;
            let mut y;
            loop {
                x = rng.gen_range(0..maze.width);
                y = rng.gen_range(0..maze.height);
                if x != self.player.x || y != self.player.y {
                    break;
                }
            }
            // Calculate "floor" as the Manhattan distance from spawn room (middle) to current room
            let spawn_row = self.mazes.len() / 2;
            let spawn_col = self.mazes[0].len() / 2;
            let floor = manhattan(self.current_room_col, self.current_room_row, spawn_col, spawn_row);

            // Scale enemy stats based on floor (stronger on higher floors/rings)
            let base_health = 3;
            let base_attack = 1;
            let health = base_health + floor * 1;
            let attack = base_attack + (floor / 2);

            self.enemies.push(Entity {
                x,
                y,
                health: health as i32,
                armor: 0,
                attack: attack as i32,
                gold: 0,
                is_player: false,
            });
        }
    }

    pub fn move_enemies(&mut self) {
        let maze = &self.mazes[self.current_room_row][self.current_room_col];
        let player_pos = (self.player.x, self.player.y);

        // Track where enemies will move to (not where they are now)
        let mut planned_positions: HashSet<(usize, usize)> = HashSet::new();

        // Collect original positions for pathfinding
        let original_positions: Vec<(usize, usize)> = self.enemies.iter().map(|e| (e.x, e.y)).collect();

        for i in 0..self.enemies.len() {
            // If enemy is adjacent to player, do not move
            if is_adjacent(self.enemies[i].x, self.enemies[i].y, player_pos.0, player_pos.1, maze) {
                planned_positions.insert((self.enemies[i].x, self.enemies[i].y));
                continue;
            }

            // Check if adjacent to another enemy (not self)
            let mut attacked = false;
            for j in 0..self.enemies.len() {
                if i != j {
                    // Use split_at_mut to avoid double-borrow
                    let (left, right) = if i < j {
                        let (left, right) = self.enemies.split_at_mut(j);
                        (&mut left[i], &mut right[0])
                    } else {
                        let (left, right) = self.enemies.split_at_mut(i);
                        (&mut right[0], &mut left[j])
                    };

                    if is_adjacent(
                        left.x,
                        left.y,
                        right.x,
                        right.y,
                        maze,
                    ) {
                        // Attack the other enemy (for demonstration, just reduce health)
                        right.take_damage(left.attack);
                        attacked = true;
                        break;
                    }
                }
            }
            if attacked {
                planned_positions.insert((self.enemies[i].x, self.enemies[i].y));
                continue;
            }

            // A* pathfinding to player
            fn astar<F, H>(
                start: (usize, usize),
                goal: (usize, usize),
                mut neighbors: F,
                mut heuristic: H,
            ) -> Option<Vec<(usize, usize)>>
            where
                F: FnMut((usize, usize)) -> Vec<((usize, usize), usize)>,
                H: FnMut((usize, usize)) -> usize,
            {
                #[derive(Eq)]
                struct Node {
                    pos: (usize, usize),
                    cost: usize,
                    est: usize,
                }
                impl Ord for Node {
                    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                        (other.cost + other.est).cmp(&(self.cost + self.est))
                    }
                }
                impl PartialOrd for Node {
                    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                        Some(self.cmp(other))
                    }
                }
                impl PartialEq for Node {
                    fn eq(&self, other: &Self) -> bool {
                        self.pos == other.pos && self.cost + self.est == other.cost + other.est
                    }
                }

                let mut heap = BinaryHeap::new();
                let mut came_from = HashMap::new();
                let mut cost_so_far = HashMap::new();

                heap.push(Node { pos: start, cost: 0, est: heuristic(start) });
                cost_so_far.insert(start, 0);

                while let Some(Node { pos, cost, .. }) = heap.pop() {
                    if pos == goal {
                        // Reconstruct path
                        let mut path = vec![pos];
                        let mut current = pos;
                        while let Some(&prev) = came_from.get(&current) {
                            path.push(prev);
                            current = prev;
                        }
                        path.reverse();
                        return Some(path);
                    }
                    for (next, step_cost) in neighbors(pos) {
                        let new_cost = cost + step_cost;
                        if cost_so_far.get(&next).map_or(true, |&c| new_cost < c) {
                            cost_so_far.insert(next, new_cost);
                            let est = heuristic(next);
                            heap.push(Node { pos: next, cost: new_cost, est });
                            came_from.insert(next, pos);
                        }
                    }
                }
                None
            }

            let path = astar(
                (self.enemies[i].x, self.enemies[i].y),
                player_pos,
                |pos| {
                    let mut neighbors = Vec::new();
                    let directions = [
                        (0isize, -1isize, 0),  // up
                        (1, 0, 1),             // right
                        (0, 1, 2),             // down
                        (-1, 0, 3),            // left
                    ];

                    for &(dx, dy, dir_idx) in &directions {
                        let nx = pos.0 as isize + dx;
                        let ny = pos.1 as isize + dy;

                        if nx < 0 || ny < 0 || nx >= maze.width as isize || ny >= maze.height as isize {
                            continue;
                        }

                        if !maze.can_move(pos.1, pos.0, dir_idx) {
                            continue;
                        }

                        let next_pos = (nx as usize, ny as usize);

                        neighbors.push((next_pos, 1));
                    }

                    neighbors
                },
                |pos| manhattan(pos.0, pos.1, player_pos.0, player_pos.1),
            );

            if let Some(path) = path {
                if path.len() > 1 {
                    let next = path[1];
                    if !planned_positions.contains(&next) {
                        self.enemies[i].x = next.0;
                        self.enemies[i].y = next.1;
                        planned_positions.insert(next);
                    } else {
                        planned_positions.insert((self.enemies[i].x, self.enemies[i].y));
                    }
                } else {
                    planned_positions.insert((self.enemies[i].x, self.enemies[i].y));
                }
            } else {
                planned_positions.insert((self.enemies[i].x, self.enemies[i].y));
            }
        }

        // Remove dead enemies after all moves/attacks
        self.enemies.retain(|e| e.health > 0);
    }

    /// Player move, with enemy logic
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

        // Check if moving into enemy
        if let Some(enemy) = self.enemies.iter_mut().find(|e| e.x as isize == new_x && e.y as isize == new_y) {
            // Attack enemy
            enemy.take_damage(self.player.attack);
            if enemy.health <= 0 {
                // Remove dead enemy
                self.enemies.retain(|e| e.health > 0);
            }
            // Player does not move
            self.enemy_attack_player();
            self.move_enemies();
            return Ok(true);
        }

        // Normal move
        if new_x >= 0 && new_x < maze.width as isize && new_y >= 0 && new_y < maze.height as isize {
            if maze.can_move(self.player.y, self.player.x, dir_idx) {
                self.player.x = new_x as usize;
                self.player.y = new_y as usize;
                self.enemy_attack_player();
                self.move_enemies();
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
            // Spawn enemies when entering a new room
            // let floor = manhattan(self.current_room_col, self.current_room_row, , spawn_row);
            self.spawn_enemies(3); // You can adjust the number as needed
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

    /// Enemies attack player if adjacent (not blocked by wall)
    fn enemy_attack_player(&mut self) {
        let maze = &self.mazes[self.current_room_row][self.current_room_col];
        for enemy in &self.enemies {
            if is_adjacent(enemy.x, enemy.y, self.player.x, self.player.y, maze) {
                self.player.take_damage(enemy.attack);
            }
        }
    }
}

/// Check if two positions are adjacent and not blocked by wall
pub fn is_adjacent(x1: usize, y1: usize, x2: usize, y2: usize, maze: &Maze) -> bool {
    let dx = x2 as isize - x1 as isize;
    let dy = y2 as isize - y1 as isize;
    let (dir_idx, valid) = match (dx, dy) {
        (0, -1) => (0, true), // up
        (1, 0) => (1, true),  // right
        (0, 1) => (2, true),  // down
        (-1, 0) => (3, true), // left
        _ => (0, false),
    };
    valid && maze.can_move(y1, x1, dir_idx)
}

pub fn manhattan(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    x1.abs_diff(x2) + y1.abs_diff(y2)
}