use crate::runner::AocDay;

pub struct DecemberFirst {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl AocDay for DecemberFirst {
    fn new(content: String) -> Self {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for line in content.lines() {
            let mut parts = line.split("   ");
            left.push(parts.next().unwrap().parse().unwrap());
            right.push(parts.next().unwrap().parse().unwrap());
        }

        DecemberFirst {
            left,
            right,
        }
    }

    fn part1(&self) -> String {
        let mut left_sorted = self.left.clone();
        let mut right_sorted = self.right.clone();
        left_sorted.sort();
        right_sorted.sort();
        let dist = left_sorted
            .iter()
            .zip(right_sorted.iter())
            .map(|(l, r)| (l-r).abs())
            .map(|x| x)
            .sum::<i32>();
        dist.to_string()
    }

    fn part2(&self) -> String {
        let sum = self.left
            .iter()
            .map(|l| {
                let right_count = self.right.iter().filter(|r| l == *r).count();
                l * right_count as i32
            })
            .sum::<i32>();
        sum.to_string()
    }
}