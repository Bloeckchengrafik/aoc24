use std::fmt::Debug;
use crate::runner::AocDay;
use crate::utils::direction::Direction;
use crate::utils::Vec2;

#[derive(Clone)]
pub struct WarehouseWoes {
    pub map: Vec<Vec<char>>,
    pub directions: Vec<Direction>,
    pub pos: Vec2,
}

impl WarehouseWoes {
    fn get(&self, pos: Vec2) -> char {
        self.map[pos.y as usize][pos.x as usize]
    }

    fn try_move(&mut self, pos: Vec2, dir: Direction) -> bool {
        let dir_vec = dir.to_point();
        let new_pos = pos + dir_vec;
        let tile = self.get(new_pos);

        if tile == '.' {
            return true;
        }

        if tile == 'O' {
            if !self.try_move(new_pos, dir) {
                return false;
            }

            // move the box
            self.map[new_pos.y as usize][new_pos.x as usize] = '.';
            self.map[(new_pos.y + dir_vec.y) as usize][(new_pos.x + dir_vec.x) as usize] = 'O';
            return true;
        }

        false
    }

    fn walk(&mut self) {
        let dir = self.directions.remove(0);
        if !self.try_move(self.pos, dir) {
            return;
        }
        self.pos += dir.to_point();
    }

    fn widen(&mut self) {
        let mut new_map = Vec::new();
        for row in self.map.iter() {
            let mut new_row = Vec::new();
            for c in row.iter() {
                if *c == 'O' {
                    new_row.push('[');
                    new_row.push(']');
                } else {
                    new_row.push(*c);
                    new_row.push(*c);
                }
            }
            new_map.push(new_row);
        }

        self.map = new_map;
        self.pos.x *= 2;
    }

    fn try_move_wide(&mut self, pos: Vec2, dir: Direction, do_move: bool) -> bool {
        let dir_vec = dir.to_point();
        let new_pos = pos + dir_vec;
        let tile = self.get(new_pos);

        if tile == '.' {
            return true;
        }

        if tile == '[' {
            if !self.try_move_wide(new_pos + Vec2::new(1, 0), dir, false) {
                return false;
            }

            // move the box
            if do_move {
                self.try_move_wide(new_pos + Vec2::new(1, 0), dir, true);

                self.map[new_pos.y as usize][new_pos.x as usize] = '.';
                self.map[(new_pos.y + dir_vec.y) as usize][(new_pos.x + dir_vec.x) as usize] = '[';
                self.map[new_pos.y as usize][(new_pos.x + 1) as usize] = '.';
                self.map[(new_pos.y + dir_vec.y) as usize][(new_pos.x + dir_vec.x + 1) as usize] = ']';
            }
            return true;
        }

        if tile == ']' {
            // check [
            if !self.try_move_wide(new_pos + Vec2::new(-1, 0), dir, false) {
                return false;
            }

            // move the box
            if do_move {
                self.try_move_wide(new_pos + Vec2::new(-1, 0), dir, true);

                self.map[new_pos.y as usize][new_pos.x as usize] = '.';
                self.map[new_pos.y as usize][(new_pos.x - 1) as usize] = '.';
                self.map[(new_pos.y + dir_vec.y) as usize][(new_pos.x + dir_vec.x) as usize] = ']';
                self.map[(new_pos.y + dir_vec.y) as usize][(new_pos.x + dir_vec.x - 1) as usize] = '[';
            }
            return true;
        }

        false
    }

    fn walk_wide(&mut self) {
        let dir = self.directions.remove(0);
        if !self.try_move_wide(self.pos, dir, true) {
            return;
        }
        self.pos += dir.to_point();
    }
}

impl Debug for WarehouseWoes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if Vec2::new(x as isize, y as isize) == self.pos {
                    write!(f, "@")?;
                    continue;
                }
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl AocDay for WarehouseWoes {
    fn new(content: String) -> Self {
        let mut map_vec = Vec::new();
        let mut directions = Vec::new();
        let mut pos = Vec2::new(0, 0);
        let (map, line) = content.split_once("\n\n").unwrap();
        for (y, line) in map.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    pos = Vec2::new(x as isize, y as isize);
                    row.push('.');
                } else {
                    row.push(c);
                }
            }
            map_vec.push(row);
        }
        for line in line.lines() {
            for c in line.chars() {
                directions.push(Direction::from_char(c));
            }
        }
        Self { map: map_vec, directions, pos }
    }

    fn part1(&self) -> String {
        let mut day = self.clone();
        while !day.directions.is_empty() {
            day.walk();
        }

        let mut sum = 0;
        for (y, row) in day.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == 'O' {
                    sum += y * 100 + x;
                }
            }
        }

        sum.to_string()
    }

    fn part2(&self) -> String {
        let mut day = self.clone();
        // day.widen();
        while !day.directions.is_empty() {
            day.walk();
        }

        let mut sum = 0;
        for (y, row) in day.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == '[' {
                    sum += y * 100 + x;
                }
            }
        }

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_move() {
        let content = "#########\n\
                              #...O@..#\n\
                              #########\n\n<".to_string();
        let mut day = WarehouseWoes::new(content);
        println!("{:?}", day);
        day.walk();
        println!("{:?}", day);
        assert_eq!(day.map[1][3], 'O');
    }

    #[test]
    fn double_move() {
        let content = "#########\n\
                              #..OO@..#\n\
                              #########\n\n<".to_string();
        let mut day = WarehouseWoes::new(content);
        println!("{:?}", day);
        day.walk();
        println!("{:?}", day);
        assert_eq!(day.map[1][3], 'O');
        assert_eq!(day.map[1][2], 'O');
    }

    #[test]
    fn blocked_move() {
        let content = "#########\n\
                              #OOOO@..#\n\
                              #########\n\n<".to_string();
        let mut day = WarehouseWoes::new(content);
        println!("{:?}", day);
        day.walk();
        println!("{:?}", day);
        assert_eq!(day.pos, Vec2::new(5, 1));
    }

    #[test]
    fn open_move() {
        let content = "#########\n\
                              #....@..#\n\
                              #########\n\n<".to_string();
        let mut day = WarehouseWoes::new(content);
        println!("{:?}", day);
        day.walk();
        println!("{:?}", day);
        assert_eq!(day.pos, Vec2::new(4, 1));
    }

    #[test]
    fn simple_wide_move() {
        let content = "#########\n\
                              #...O@..#\n\
                              #########\n\n<".to_string();
        let mut day = WarehouseWoes::new(content);
        println!("{:?}", day);
        day.widen();
        println!("{:?}", day);
        assert_eq!(day.map[1][8], '[');
        assert_eq!(day.map[1][9], ']');
        day.walk_wide();
        println!("{:?}", day);
        assert_eq!(day.map[1][7], '[');
        assert_eq!(day.map[1][8], ']');
    }

    #[test]
    fn blocked_wide_move() {
        let content = "#########\n\
                              #OOOO@..#\n\
                              #########\n\n<".to_string();
        let mut day = WarehouseWoes::new(content);
        println!("{:?}", day);
        day.widen();
        println!("{:?}", day);
        assert_eq!(day.map[1][8], '[');
        assert_eq!(day.map[1][9], ']');
        day.walk_wide();
        println!("{:?}", day);
        assert_eq!(day.map[1][8], '[');
        assert_eq!(day.map[1][9], ']');
    }

    #[test]
    fn double_wide_move() {
        let content = "#########\n\
                              #..OO@..#\n\
                              #########\n\n<".to_string();
        let mut day = WarehouseWoes::new(content);
        println!("{:?}", day);
        day.widen();
        println!("{:?}", day);
        assert_eq!(day.map[1][8], '[');
        assert_eq!(day.map[1][9], ']');
        day.walk_wide();
        println!("{:?}", day);
        assert_eq!(day.map[1][7], '[');
        assert_eq!(day.map[1][8], ']');
    }

    #[test]
    fn open_wide_move() {
        let content = "#########\n\
                              #....@..#\n\
                              #########\n\n<".to_string();
        let mut day = WarehouseWoes::new(content);
        println!("{:?}", day);
        day.widen();
        println!("{:?}", day);
        assert_eq!(day.pos, Vec2::new(10, 1));
        day.walk_wide();
        println!("{:?}", day);
        assert_eq!(day.pos, Vec2::new(9, 1));
    }
}