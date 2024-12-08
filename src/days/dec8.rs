use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Sub};
use crate::runner::AocDay;

#[derive(PartialOrd, PartialEq, Copy, Clone, Eq, Hash)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl Debug for Vec2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vec2 {
    fn new(x: isize, y: isize) -> Self {
        Vec2 { x, y }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

#[derive(Clone)]
struct Map {
    frequency: char,
    nodes: Vec<Vec2>,
    antinodes: HashSet<Vec2>,
    width: usize,
    height: usize,
}

impl Map {
    fn all_from(string: &str) -> Vec<Self> {
        let mut maps = HashMap::new();
        let width = string.lines().next().unwrap().len();
        let height = string.lines().count();
        for (y, line) in string.lines().enumerate() {
            for (x, frequency) in line.chars().enumerate() {
                if frequency == '.' {
                    continue;
                }
                let map = maps.entry(frequency).or_insert(Map {
                    frequency,
                    nodes: Vec::new(),
                    antinodes: HashSet::new(),
                    width,
                    height,
                });
                let node = Vec2 { x: x as isize, y: y as isize };
                map.nodes.push(node);
            }
        }

        maps.into_values().collect()
    }

    fn is_in_bounds(&self, pos: Vec2) -> bool {
        pos.x >= 0 && pos.y >= 0 && pos.x < self.width as isize && pos.y < self.height as isize
    }

    fn calculate_antinodes(&mut self, any: bool) {
        for node_a in &self.nodes {
            for node_b in &self.nodes {
                if node_a == node_b {
                    continue;
                }

                let diff = *node_a - *node_b;
                let a_vec: Vec2 = (*node_a).into();
                let b_vec: Vec2 = (*node_b).into();
                let mut next_pos = a_vec + diff;
                if any {
                    self.antinodes.insert(a_vec);
                    self.antinodes.insert(b_vec);
                    while self.is_in_bounds(next_pos) {
                        self.antinodes.insert(next_pos);
                        next_pos = next_pos + diff;
                    }
                } else {
                    if self.is_in_bounds(next_pos) && b_vec != next_pos {
                        self.antinodes.insert(next_pos);
                    }
                }
            }
        }
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Vec2::new(x as isize, y as isize);
                if self.nodes.contains(&pos) {
                    write!(f, "{} ", self.frequency)?;
                } else if self.antinodes.contains(&pos) {
                    write!(f, "# ")?;
                } else {
                    write!(f, ". ")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}


pub struct DecemberEighth {
    maps: Vec<Map>,
}

impl AocDay for DecemberEighth {
    fn new(content: String) -> Self {
        DecemberEighth {
            maps: Map::all_from(&content),
        }
    }

    fn part1(&self) -> String {
        let mut maps = self.maps.clone();
        let mut antinodes: HashSet<Vec2> = HashSet::new();
        for map in &mut maps {
            map.calculate_antinodes(false);
            antinodes.extend(map.antinodes.iter());
        }

        antinodes.len().to_string()
    }

    fn part2(&self) -> String {
        let mut maps = self.maps.clone();
        let mut antinodes: HashSet<Vec2> = HashSet::new();
        for map in &mut maps {
            map.calculate_antinodes(true);
            antinodes.extend(map.antinodes.iter());
        }

        antinodes.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let content = include_str!("../../inputs/8_test1_small.txt").to_string();
        let maps = Map::all_from(&content);
        assert_eq!(maps.len(), 1);
        let map = &maps[0];
        assert_eq!(map.frequency, 'a');
    }

    #[test]
    fn antinode_two() {
        let content = include_str!("../../inputs/8_test1_small.txt").to_string();
        let mut maps = Map::all_from(&content);
        let mut map = maps.pop().unwrap();
        map.calculate_antinodes(false);
        println!("{:?}", map);
        assert_eq!(map.antinodes.len(), 2);
        assert_eq!(map.antinodes.contains(&Vec2::new(3, 1)), true);
        assert_eq!(map.antinodes.contains(&Vec2::new(6, 7)), true);
    }

    #[test]
    fn antinode_three() {
        let content = include_str!("../../inputs/8_test1_large.txt").to_string();
        let mut maps = Map::all_from(&content);
        let mut map = maps.pop().unwrap();
        map.calculate_antinodes(false);
        assert_eq!(map.antinodes.len(), 4);
        assert_eq!(map.antinodes.contains(&Vec2::new(0, 2)), true);
        assert_eq!(map.antinodes.contains(&Vec2::new(6, 7)), true);
        assert_eq!(map.antinodes.contains(&Vec2::new(2, 6)), true);
        assert_eq!(map.antinodes.contains(&Vec2::new(3, 1)), true);
    }


    #[test]
    fn antinode_any_three() {
        let content = include_str!("../../inputs/8_test2_small.txt").to_string();
        let mut maps = Map::all_from(&content);
        let mut map = maps.pop().unwrap();
        map.calculate_antinodes(true);
        assert_eq!(map.antinodes.len(), 9);
    }
}