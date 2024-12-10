use std::collections::HashSet;
use crate::runner::AocDay;

pub struct HoofIt {
    map: Vec<Vec<usize>>,
}

impl HoofIt {
    fn find_trails(&self, x: usize, y: usize, n: usize) -> HashSet<(usize, usize)> {
        let tile = self.map[y][x];
        if tile != n {
            return HashSet::new();
        }

        if tile == 9 && n == 9 {
            let mut set = HashSet::new();
            set.insert((x, y));
            return set;
        }

        let mut sum = HashSet::new();
        if x > 0 {
            sum.extend(self.find_trails(x - 1, y, n + 1));
        }

        if x < self.map[y].len() - 1 {
            sum.extend(self.find_trails(x + 1, y, n + 1));
        }

        if y > 0 {
            sum.extend(self.find_trails(x, y - 1, n + 1));
        }

        if y < self.map.len() - 1 {
            sum.extend(self.find_trails(x, y + 1, n + 1));
        }

        sum
    }

    fn find_trails_unique(&self, x: usize, y: usize, n: usize) -> usize {
        let tile = self.map[y][x];
        if tile != n {
            return 0;
        }

        if tile == 9 && n == 9 {
            return 1;
        }

        let mut sum = 0;
        if x > 0 {
            sum += self.find_trails_unique(x - 1, y, n + 1);
        }

        if x < self.map[y].len() - 1 {
            sum += self.find_trails_unique(x + 1, y, n + 1);
        }

        if y > 0 {
            sum += self.find_trails_unique(x, y - 1, n + 1);
        }

        if y < self.map.len() - 1 {
            sum += self.find_trails_unique(x, y + 1, n + 1);
        }

        sum
    }
}

impl AocDay for HoofIt {
    fn new(content: String) -> Self {
        // map is a grid of numbers
        let map = content
            .lines()
            .map(|l|
                l.split("")
                    .filter(|n| !n.is_empty())
                    .map(|n| n.parse().unwrap())
                    .collect()
            ).collect();
        Self { map }
    }

    fn part1(&self) -> String {
        let mut sum = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                let tile = self.map[y][x];
                if tile == 0 {
                    sum += self.find_trails(x, y, 0).len();
                }
            }
        }
        sum.to_string()
    }

    fn part2(&self) -> String {
        let mut sum = 0;
        for y in 0..self.map.len() {
            for x in 0..self.map[y].len() {
                let tile = self.map[y][x];
                if tile == 0 {
                    sum += self.find_trails_unique(x, y, 0);
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
    fn small_test() {
        let content = "0123\n1234\n8765\n9876".to_string();
        let day = HoofIt::new(content);
        let trails = day.find_trails(0, 0, 0);
        assert_eq!(trails.len(), 1);
    }
}