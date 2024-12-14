use crate::runner::AocDay;
use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;
use std::ops::Deref;
use z3::ast::{Ast, Int};
use z3::SatResult;

lazy_static! {
    static ref BTN_EXTRACTOR: Regex = Regex::new(r"Button .: X\+(\d+), Y\+(\d+)").unwrap();
    static ref PRIZE_EXTRACTOR: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
}

#[derive(Hash, Copy, Clone)]
struct Vec2 {
    x: isize,
    y: isize,
}

impl<T> From<(&str, &T)> for Vec2
where
    T: Deref<Target=Regex>,
{
    fn from(value: (&str, &T)) -> Self {
        let captures = value.1.captures(value.0).unwrap();
        Vec2 {
            x: captures.get(1).unwrap().as_str().parse().unwrap(),
            y: captures.get(2).unwrap().as_str().parse().unwrap(),
        }
    }
}

#[derive(Clone)]
struct ClawMachine {
    a_dir: Vec2,
    b_dir: Vec2,
    target: Vec2,
}

impl PartialEq<Self> for Vec2 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl PartialOrd for Vec2 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x == other.x && self.y == other.y {
            Some(Ordering::Equal)
        } else if self.x < other.x && self.y < other.y {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Eq for Vec2 {}

impl Ord for Vec2 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn solve_linear_equations(a1: i64, b1: i64, c1: i64, a2: i64, b2: i64, c2: i64) -> Option<(i64, i64)> {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Optimize::new(&ctx);
    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");

    let a1 = Int::from_i64(&ctx, a1);
    let b1 = Int::from_i64(&ctx, b1);
    let c1 = Int::from_i64(&ctx, c1);
    let a2 = Int::from_i64(&ctx, a2);
    let b2 = Int::from_i64(&ctx, b2);
    let c2 = Int::from_i64(&ctx, c2);
    let cost_x = Int::from_i64(&ctx, 3);

    // a1*x + b1*y = c1
    solver.assert(&c1._eq(&((&a1 * &x) + (&b1 * &y))));
    // a2*x + b2*y = c2
    solver.assert(&c2._eq(&((&a2 * &x) + (&b2 * &y))));
    solver.minimize(&(&x * cost_x + &y));

    let result = solver.check(&[]);
    if result != SatResult::Sat {
        return None;
    }

    let model = solver.get_model().unwrap();
    let x_val = model.eval(&x, true).unwrap().as_i64().unwrap();
    let y_val = model.eval(&y, true).unwrap().as_i64().unwrap();

    println!("Model: x={}, y={}", x_val, y_val);

    Some((x_val, y_val))
}

impl ClawMachine {
    #[allow(dead_code)]
    fn search_smallest_solution(&self) -> isize {
        let mut tokens_spent: isize = isize::MAX;
        let pos = Vec2 { x: 0, y: 0 };
        let mut queue = HashMap::new();
        queue.insert(pos, 0isize);

        while !queue.is_empty() {
            let current = {
                let cq = queue.clone();
                let val = cq.iter().min_by_key(|(_, &dist)| dist).unwrap();
                (*val.0, *val.1)
            };

            queue.remove(&current.0);

            // move with A
            {
                let next = Vec2 {
                    x: current.0.x + self.a_dir.x,
                    y: current.0.y + self.a_dir.y,
                };

                if next < self.target {
                    let next_cost = current.1 + 3;
                    queue.insert(next, next_cost);
                } else if next == self.target && ((current.1 + 3) < tokens_spent) {
                    tokens_spent = current.1 + 3;
                }
            }

            // move with B
            {
                let next = Vec2 {
                    x: current.0.x + self.b_dir.x,
                    y: current.0.y + self.b_dir.y,
                };

                if next < self.target {
                    let next_cost = current.1 + 1;
                    queue.insert(next, next_cost);
                } else if next == self.target && ((current.1 + 1) < tokens_spent) {
                    tokens_spent = current.1 + 1;
                }
            }
        }

        tokens_spent
    }

    fn search_smallest_solution_linalg(&self) -> isize {
        if let Some((a, b)) = solve_linear_equations(
            self.a_dir.x as i64, self.b_dir.x as i64, self.target.x as i64,
            self.a_dir.y as i64, self.b_dir.y as i64, self.target.y as i64
        ) {
            (3*a + b) as isize
        } else {
            isize::MAX
        }
    }
}

pub struct ClawContraption {
    machines: Vec<ClawMachine>,
}

impl AocDay for ClawContraption {
    fn new(content: String) -> Self {
        let machines: Vec<ClawMachine> = content.split("\n\n")
            .map(|machine_conf| {
                let a_line = machine_conf.lines().nth(0).unwrap();
                let b_line = machine_conf.lines().nth(1).unwrap();
                let target_line = machine_conf.lines().nth(2).unwrap();

                ClawMachine {
                    a_dir: Vec2::from((a_line, &BTN_EXTRACTOR)),
                    b_dir: Vec2::from((b_line, &BTN_EXTRACTOR)),
                    target: Vec2::from((target_line, &PRIZE_EXTRACTOR)),
                }
            })
            .collect();
        ClawContraption { machines }
    }

    fn part1(&self) -> String {
        let all_machines_len = self.machines.len();
        self.machines.iter()
            .enumerate()
            .map(|(i, machine)| {
                println!("Solving machine {}/{}...", i + 1, all_machines_len);
                machine.search_smallest_solution_linalg()
            })
            .filter(|&solution| solution != isize::MAX)
            .sum::<isize>()
            .to_string()
    }

    fn part2(&self) -> String {
        let all_machines_len = self.machines.len();
        self.machines.clone().iter_mut()
            .enumerate()
            .map(|(i, machine)| {
                machine.target.x += 10000000000000;
                machine.target.y += 10000000000000;
                println!("Solving machine {}/{}...", i + 1, all_machines_len);
                machine.search_smallest_solution_linalg()
            })
            .filter(|&solution| solution != isize::MAX)
            .sum::<isize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::black_box;

    #[test]
    fn parsing() {
        let content = "Button 1: X+1, Y+1\nButton 2: X+2, Y+2\nPrize: X=3, Y=3".to_string();
        let contraption = ClawContraption::new(content);
        assert_eq!(contraption.machines.len(), 1);
        let machine = &contraption.machines[0];
        assert_eq!(machine.a_dir.x, 1);
        assert_eq!(machine.a_dir.y, 1);
        assert_eq!(machine.b_dir.x, 2);
        assert_eq!(machine.b_dir.y, 2);
        assert_eq!(machine.target.x, 3);
        assert_eq!(machine.target.y, 3);
    }

    #[test]
    fn unwinnable() {
        let content = "Button 1: X+20, Y+20\nButton 2: X+20, Y+20\nPrize: X=3, Y=3".to_string();
        let contraption = ClawContraption::new(content);
        let machine = &contraption.machines[0];
        assert_eq!(machine.search_smallest_solution_linalg(), isize::MAX);
    }

    #[bench]
    fn bench_smallest_search(b: &mut test::Bencher) {
        let content = "Button 1: X+1, Y+1\nButton 2: X+2, Y+2\nPrize: X=3, Y=3".to_string();
        let contraption = ClawContraption::new(content);
        b.iter(|| black_box(contraption.machines[0].search_smallest_solution_linalg()));
    }

    #[test]
    fn solve_lineq() {
        assert_eq!(solve_linear_equations(1, 2, 3, 4, 5, 6), Some((-1, 2)));
        assert_eq!(solve_linear_equations(1, 2, 3, 4, 5, 7), None);
    }
}