use std::cmp::Ordering;
use crate::runner::AocDay;

#[derive(Clone)]
struct PageRule {
    left: i32,
    right: i32,
}

trait Orders<T> {
    fn cmp(&self, a: &T, b: &T) -> i32;
}

impl TryFrom<&str> for PageRule {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut parts = value.split('|');
        let left = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        let right = parts.next().ok_or(())?.parse().map_err(|_| ())?;
        Ok(Self { left, right })
    }
}

impl PageRule {
    fn matches(&self, update: &PageUpdate) -> bool {
        if !update.0.contains(&self.left) || !update.0.contains(&self.right) {
            return true;
        }

        let left_index = update.0.iter().position(|x| *x == self.left).unwrap();
        let right_index = update.0.iter().position(|x| *x == self.right).unwrap();
        left_index < right_index
    }
}

impl Orders<i32> for PageRule {
    fn cmp(&self, a: &i32, b: &i32) -> i32 {
        if a == &self.left && b == &self.right {
            return -1;
        }
        if a == &self.right && b == &self.left {
            return 1;
        }

        0
    }
}

struct PageUpdate(Vec<i32>);

impl TryFrom<&str> for PageUpdate {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts = value.split(',').map(|x| x.parse().map_err(|_| ()));
        let mut vec = Vec::new();
        for part in parts {
            vec.push(part?);
        }
        Ok(Self(vec))
    }
}

impl PageUpdate {
    fn get_middle(&self) -> i32 {
        // get the middle value (0, 3, 9, **6**, 4, 2, 12)
        let len = self.0.len();
        let middle = len / 2;
        self.0[middle]
    }

    fn sort(&self, rules: Vec<impl Orders<i32> + Clone>) -> Self {
        let mut vec = self.0.clone();
        vec.sort_by(|a, b| {
            let mut overall_effect = 0;
            for rule in rules.clone() {
                let effect = rule.cmp(a, b);
                if effect != 0 {
                    overall_effect += effect;
                }
            }

            if overall_effect == 0 {
                Ordering::Equal
            } else if overall_effect < 0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        Self(vec)
    }
}

pub struct PrintQueue {
    rules: Vec<PageRule>,
    updates: Vec<PageUpdate>,
}

impl AocDay for PrintQueue {
    fn new(content: String) -> Self {
        let mut lines = content.lines();
        let mut rules = Vec::new();
        let mut updates = Vec::new();
        loop {
            let line = lines.next();
            if line.is_none() {
                break;
            }
            if let Ok(rule) = PageRule::try_from(line.unwrap()) {
                rules.push(rule);
            } else {
                break;
            }
        }

        for line in lines {
            if let Ok(update) = PageUpdate::try_from(line) {
                updates.push(update);
            }
        }

        Self { rules, updates }
    }

    fn part1(&self) -> String {
        let sum: i32 = self.updates
            .iter()
            .filter(|update| self.rules.iter().all(|r| r.matches(update)))
            .map(|update| update.get_middle())
            .sum();

        sum.to_string()
    }

    fn part2(&self) -> String {
        let sum: i32 = self.updates
            .iter()
            .filter(|update| self.rules.iter().any(|r| !r.matches(update)))
            .map(|u| u.sort(self.rules.clone()))
            .map(|update| update.get_middle())
            .sum();

        sum.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn middle_value_1() {
        let update = PageUpdate::try_from("75,47,61,53,29").unwrap();
        assert_eq!(update.get_middle(), 61);
    }

    #[test]
    fn middle_value_2() {
        let update = PageUpdate::try_from("97,61,53,29,13").unwrap();
        assert_eq!(update.get_middle(), 53);
    }

    #[test]
    fn middle_value_3() {
        let update = PageUpdate::try_from("75,29,13").unwrap();
        assert_eq!(update.get_middle(), 29);
    }
}