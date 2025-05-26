
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