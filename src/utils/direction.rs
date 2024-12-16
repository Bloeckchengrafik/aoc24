use crate::utils::Vec2;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn from_char(c: char) -> Self {
        match c {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            '^' => Self::Up,
            'v' => Self::Down,
            'V' => Self::Down,
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Invalid direction"),
        }
    }

    pub fn to_char(&self) -> char {
        match self {
            Self::Up => 'U',
            Self::Down => 'D',
            Self::Left => 'L',
            Self::Right => 'R',
        }
    }

    pub fn to_point(&self) -> Vec2 {
        match self {
            Self::Up => Vec2::new(0, -1),
            Self::Down => Vec2::new(0, 1),
            Self::Left => Vec2::new(-1, 0),
            Self::Right => Vec2::new(1, 0),
        }
    }

    pub fn iter() -> impl Iterator<Item = Self> {
        [Self::Up, Self::Down, Self::Left, Self::Right].iter().copied()
    }

    pub fn iter_forwards_from(&self) -> impl Iterator<Item = Self> {
        match self {
            Self::Up => [Self::Up, Self::Right, Self::Left].iter().copied(),
            Self::Down => [Self::Down, Self::Right, Self::Left].iter().copied(),
            Self::Left => [Self::Left, Self::Up, Self::Down].iter().copied(),
            Self::Right => [Self::Right, Self::Up, Self::Down].iter().copied(),
        }
    }

    pub fn orthogonal(&self) -> [Direction; 2] {
        match self {
            Self::Up => [Self::Right, Self::Left],
            Self::Down => [Self::Right, Self::Left],
            Self::Left => [Self::Up, Self::Down],
            Self::Right => [Self::Up, Self::Down],
        }
    }
}