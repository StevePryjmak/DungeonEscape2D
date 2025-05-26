
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    pub walls: [bool; 4], // [Top, Right, Bottom, Left]
    pub visited: bool,
}

impl Cell {
    pub fn new() -> Self {
        Self {
            walls: [true; 4],
            visited: false,
        }
    }

    pub fn has_wall(&self, dir: Direction) -> bool {
        self.walls[dir as usize]
    }

    pub fn remove_wall(&mut self, dir: Direction) {
        self.walls[dir as usize] = false;
    }

    pub fn set_visited(&mut self) {
        self.visited = true;
    }
}
