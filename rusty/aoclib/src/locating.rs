//locating.rs

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    #[default]
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

pub const DIRS: [Direction; 8] = [
    Direction::Up,
    Direction::UpRight,
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::DownLeft,
    Direction::Left,
    Direction::UpLeft,
];

impl Direction {
    pub fn unit(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1,0),
            Direction::UpRight => (-1,1),
            Direction::Right => (0,1),
            Direction::DownRight => (1,1),
            Direction::Down => (1,0),
            Direction::DownLeft => (1,-1),
            Direction::Left => (0,-1),
            Direction::UpLeft => (-1,-1),
        }
    }
}
