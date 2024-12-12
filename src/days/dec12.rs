use std::fmt::Debug;
use std::ops::{Add, AddAssign};
use crate::days::dec12::Dir::{Down, Up};
use crate::runner::AocDay;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    fn turn_right(&self) -> Self {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }

    fn turn_left(&self) -> Self {
        match self {
            Dir::Up => Dir::Left,
            Dir::Left => Dir::Down,
            Dir::Down => Dir::Right,
            Dir::Right => Dir::Up,
        }
    }
}

impl From<(i32, i32)> for Dir {
    fn from(value: (i32, i32)) -> Self {
        match value {
            (0, -1) => Dir::Up,
            (0, 1) => Dir::Down,
            (-1, 0) => Dir::Left,
            (1, 0) => Dir::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

impl Into<(i32, i32)> for Dir {
    fn into(self) -> (i32, i32) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }
}

#[derive(Clone)]
pub struct GroupInfo {
    pub perimeter: usize,
    pub perimeter_elems: Vec<(usize, usize, Dir)>,
    pub area: usize,
    pub char: char,
}

impl Debug for GroupInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(format!("Group[{}]", self.char).as_str())
            .field("p", &self.perimeter)
            .field("a", &self.area)
            .finish()
    }
}

impl GroupInfo {
    fn is_zero(&self) -> bool {
        self.perimeter == 0 && self.area == 0
    }

    fn price(&self) -> usize {
        self.area * self.perimeter
    }

    fn price_walked(&self) -> usize {
        self.area * self.unique_lines()
    }

    fn filter_neighbours(elems: &mut Vec<(usize, usize, Dir)>, base: &(usize, usize, Dir)) {
        elems.retain(|(x, y, dir)|
            !(*x == base.0 && *y == base.1 && *dir == base.2)
        );

        if base.2 == Up || base.2 == Down {
            if base.0 > 0 {
                let neighbour_up = (base.0 - 1, base.1, base.2);
                let exists = elems.iter().any(|(x, y, dir)| *x == neighbour_up.0 && *y == neighbour_up.1 && *dir == neighbour_up.2);
                if exists {
                    Self::filter_neighbours(elems, &neighbour_up);
                }
            }

            let neighbour_down = (base.0 + 1, base.1, base.2);
            let exists = elems.iter().any(|(x, y, dir)| *x == neighbour_down.0 && *y == neighbour_down.1 && *dir == neighbour_down.2);
            if exists {
                Self::filter_neighbours(elems, &neighbour_down);
            }
        } else {
            if base.1 > 0 {
                let neighbour_left = (base.0, base.1 - 1, base.2);
                let exists = elems.iter().any(|(x, y, dir)| *x == neighbour_left.0 && *y == neighbour_left.1 && *dir == neighbour_left.2);
                if exists {
                    Self::filter_neighbours(elems, &neighbour_left);
                }
            }

            let neighbour_right = (base.0, base.1 + 1, base.2);
            let exists = elems.iter().any(|(x, y, dir)| *x == neighbour_right.0 && *y == neighbour_right.1 && *dir == neighbour_right.2);
            if exists {
                Self::filter_neighbours(elems, &neighbour_right);
            }
        }
    }

    fn unique_lines(&self) -> usize {
        let mut elems = self.perimeter_elems.clone();
        let mut count = 0;
        while let Some(elem) = elems.clone().first() {
            Self::filter_neighbours(&mut elems, &elem);
            count += 1;
        }

        count
    }
}

impl Add for GroupInfo {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        GroupInfo {
            perimeter: self.perimeter + rhs.perimeter,
            perimeter_elems: [self.perimeter_elems, rhs.perimeter_elems].concat(),
            area: self.area + rhs.area,
            char: self.char,
        }
    }
}

impl AddAssign for GroupInfo {
    fn add_assign(&mut self, rhs: Self) {
        *self = self.clone() + rhs;
    }
}

#[derive(Clone)]
pub struct GardenGroups(Vec<Vec<char>>);

impl GardenGroups {
    fn segment_at(&mut self, x: usize, y: usize, c: char, dir: Dir) -> GroupInfo {
        let tile = &self.0[y][x];

        if *tile != c {
            let is_perimeter = (*tile).to_ascii_uppercase() != c;
            if is_perimeter {
                let (dx, dy): (i32, i32) = dir.into();
                return GroupInfo {
                    perimeter: 1,
                    perimeter_elems: vec![((x as i32 - dx) as usize, (y as i32 - dy) as usize, dir)],
                    area: 0,
                    char: c,
                };
            }
            return GroupInfo {
                perimeter: 0,
                perimeter_elems: vec![],
                area: 0,
                char: c,
            };
        }

        self.0[y][x] = self.0[y][x].to_ascii_lowercase();

        let mut group = GroupInfo {
            perimeter: 0,
            perimeter_elems: vec![],
            area: 1,
            char: c,
        };

        if x > 0 {
            group += self.segment_at(x - 1, y, c, Dir::Left);
        } else {
            group.perimeter += 1;
            group.perimeter_elems.push((x, y, Dir::Left));
        }

        if y > 0 {
            group += self.segment_at(x, y - 1, c, Dir::Up);
        } else {
            group.perimeter += 1;
            group.perimeter_elems.push((x, y, Dir::Up));
        }

        if x < self.0[y].len() - 1 {
            group += self.segment_at(x + 1, y, c, Dir::Right);
        } else {
            group.perimeter += 1;
            group.perimeter_elems.push((x, y, Dir::Right));
        }

        if y < self.0.len() - 1 {
            group += self.segment_at(x, y + 1, c, Dir::Down);
        } else {
            group.perimeter += 1;
            group.perimeter_elems.push((x, y, Dir::Down));
        }

        group
    }

    fn segment(&self) -> Vec<GroupInfo> {
        let mut groups = Vec::new();
        let mut me = self.clone();

        for y in 0..me.0.len() {
            for x in 0..me.0[y].len() {
                let tile = me.0[y][x];

                if tile.is_lowercase() {
                    continue;
                }

                let segment = me.segment_at(x, y, tile, Dir::Up);
                if !segment.is_zero() {
                    groups.push(segment);
                }
            }
        }

        groups
    }
}

impl AocDay for GardenGroups {
    fn new(content: String) -> Self {
        GardenGroups(content.lines().map(|l| l.chars().collect()).collect())
    }

    fn part1(&self) -> String {
        let groups = self.segment();
        let area = groups.iter().map(|g| g.price()).sum::<usize>();
        area.to_string()
    }

    fn part2(&self) -> String {
        let groups = self.segment();
        let area = groups.iter().map(|g| g.price_walked()).sum::<usize>();
        area.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small() {
        let content = "AAA\nABA\nBBA".to_string();
        let gg = GardenGroups::new(content);
        let mut segments = gg.segment();
        let b_seg = segments.pop().unwrap();
        let a_seg = segments.pop().unwrap();

        assert_eq!(a_seg.char, 'A');
        assert_eq!(b_seg.char, 'B');
        assert_eq!(a_seg.area, 6);
        assert_eq!(b_seg.area, 3);
        assert_eq!(a_seg.perimeter, 14);
        assert_eq!(a_seg.perimeter_elems.len(), 14);
        assert_eq!(b_seg.perimeter, 8);
        assert_eq!(b_seg.perimeter_elems.len(), 8);
    }

    #[test]
    fn e() {
        let content = include_str!("../../inputs/12_test2_e.txt").to_string();
        let gg = GardenGroups::new(content);
        let segments = gg.segment();
        let e_seg = &segments[0];

        assert_eq!(e_seg.area, 17);
        assert_eq!(e_seg.unique_lines(), 12);
    }
}