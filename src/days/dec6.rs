use std::cmp::PartialEq;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use anyhow::bail;
use crate::runner::AocDay;

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Step(i32, i32);
#[derive(Copy, Clone, Default, PartialEq, Debug)]
struct Pos(i32, i32);

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Pos(x as i32, y as i32)
    }
}

impl Add<Step> for Pos {
    type Output = Pos;

    fn add(self, rhs: Step) -> Self::Output {
        Pos(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "^"),
            Direction::Down => write!(f, "v"),
            Direction::Left => write!(f, "<"),
            Direction::Right => write!(f, ">")
        }
    }
}

impl Into<Step> for Direction {
    fn into(self) -> Step {
        match self {
            Direction::Up => Step(0, -1),
            Direction::Down => Step(0, 1),
            Direction::Left => Step(-1, 0),
            Direction::Right => Step(1, 0)
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum Tile {
    Wall,
    Open,
    OutOfBounds,
}

#[derive(Clone, )]
struct Guard {
    map: Vec<Vec<Tile>>,
    guard: Pos,
    direction: Direction,
}

impl From<String> for Guard {
    fn from(value: String) -> Self {
        let mut map = Vec::new();
        let mut guard = Pos::default();
        for (y, line) in value.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => row.push(Tile::Wall),
                    '.' => row.push(Tile::Open),
                    '^' => {
                        row.push(Tile::Open);
                        guard = Pos::new(x, y);
                    }
                    _ => panic!("Invalid character in map")
                }
            }
            map.push(row);
        }
        Guard {
            map,
            guard,
            direction: Direction::Up,
        }
    }
}

impl Guard {
    fn tile_at(&self, pos: Pos) -> Tile {
        if pos.0 < 0 || pos.1 < 0 || pos.0 >= self.map[0].len() as i32 || pos.1 >= self.map.len() as i32 {
            Tile::OutOfBounds
        } else {
            self.map[pos.1 as usize][pos.0 as usize]
        }
    }

    fn next_tile(&self) -> Tile {
        let step: Step = self.direction.into();
        self.tile_at(self.guard + step)
    }

    pub fn set_wall_forwards(&mut self) {
        let step: Step = self.direction.into();
        let next_pos = self.guard + step;
        self.map[next_pos.1 as usize][next_pos.0 as usize] = Tile::Wall;
    }

    fn step(&mut self) -> anyhow::Result<bool> {
        let step: Step = self.direction.into();
        let next_pos = self.guard + step;
        let tile = self.tile_at(next_pos);
        if tile == Tile::Wall {
            self.direction = self.direction.rotate_right();
            Ok(true)
        } else if tile == Tile::Open {
            self.guard = next_pos;
            Ok(false)
        } else {
            bail!("OOB at {:?}", next_pos);
        }
    }

    fn get_pos(&self) -> Pos {
        self.guard
    }
}

impl Display for Guard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if Pos::new(x, y) == self.guard {
                    write!(f, "{}", self.direction)?;
                } else {
                    match tile {
                        Tile::Wall => write!(f, "#")?,
                        Tile::Open => write!(f, ".")?,
                        Tile::OutOfBounds => write!(f, "-")?,
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct DecemberSixth(Guard);

impl DecemberSixth {
    fn check_loop(guard: &mut Guard, mut positions: Vec<Vec<Vec<Direction>>>) -> bool {
        assert_eq!((&guard).next_tile(), Tile::Wall, "Next tile is not wall");

        let mut max_steps = 10000;
        loop {
            let result = guard.step();
            if result.is_err() {
                return false;
            }

            let pos = guard.get_pos();
            let new_pos_directions = positions[pos.1 as usize][pos.0 as usize].clone();
            // Loop = we have been here before and we are facing the same direction
            if new_pos_directions.contains(&guard.direction) {
                return true;
            }

            positions[pos.1 as usize][pos.0 as usize].push(guard.direction);

            max_steps -= 1;
            if max_steps == 0 {
                panic!("Max steps reached");
            }
        }
    }
}

impl AocDay for DecemberSixth {
    fn new(content: String) -> Self {
        let guard = Guard::from(content);
        DecemberSixth(guard)
    }

    fn part1(&self) -> String {
        let mut guard_positions = Vec::new();
        let mut guard = self.0.clone();
        guard_positions.push(guard.get_pos());
        let mut max_steps = 10000;
        loop {
            let result = guard.step();
            if result.is_err() {
                break;
            }
            #[cfg(test)]
            if result.unwrap() {
                println!("\n{}", guard);
            }
            let pos = guard.get_pos();
            if !guard_positions.contains(&pos) {
                guard_positions.push(pos);
            }

            max_steps -= 1;
            if max_steps == 0 {
                assert_eq!(0, 1, "Max steps reached");
                break;
            }
        }

        guard_positions.len().to_string()
    }

    fn part2(&self) -> String {
        let mut loops = 0;
        let mut guard = self.0.clone();
        let h = guard.map.len();
        let w = guard.map[0].len();
        let mut positions = vec![vec![vec![]; w]; h];
        let start_pos = guard.get_pos();

        let mut max_steps = 10000;
        loop {
            max_steps -= 1;
            if max_steps == 0 {
                panic!("Max steps reached");
            }
            let position = guard.get_pos();
            positions[position.1 as usize][position.0 as usize].push(guard.direction);
            let next_tile = guard.next_tile();
            match next_tile {
                Tile::Wall => {
                    guard.direction = guard.direction.rotate_right();
                }
                Tile::Open => {
                    if position != start_pos {
                        let mut new_track_guard = guard.clone();
                        new_track_guard.set_wall_forwards();
                        if Self::check_loop(&mut new_track_guard, positions.clone()) {
                            loops += 1;
                        }
                    }
                    guard.step().unwrap(); // we know we are on open tile
                }
                Tile::OutOfBounds => {
                    // solution space escaped
                    break;
                }
            }
        }

        loops.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_pos() {
        let map = include_str!("../../inputs/6_test1.txt");
        let guard = Guard::from(map.to_string());
        assert_eq!(guard.get_pos(), Pos(4, 6));
    }

    #[test]
    fn part1_step() {
        let map = include_str!("../../inputs/6_test1.txt");
        let mut guard = Guard::from(map.to_string());
        let _ = guard.step();
        assert_eq!(guard.get_pos(), Pos(4, 5));
    }

    #[test]
    fn part1_rotate() {
        let map = include_str!("../../inputs/6_test1_urotate.txt");
        let mut guard = Guard::from(map.to_string());
        assert_eq!(guard.get_pos(), Pos(4, 1));
        let _ = guard.step();
        assert_eq!(guard.get_pos(), Pos(4, 1));
        let _ = guard.step();
        assert_eq!(guard.get_pos(), Pos(5, 1));
    }
}