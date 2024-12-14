use lazy_static::lazy_static;
use crate::runner::AocDay;
use crate::utils::Vec2;


#[cfg(not(test))]
fn get_field() -> Vec2 {
    Vec2::new(101, 103)
}

#[cfg(test)]
fn get_field() -> Vec2 {
    Vec2::new(11, 7)
}

lazy_static! {
    static ref RE_P: regex::Regex = regex::Regex::new(r"p=([-\d]+),([-\d]+)").unwrap();
    static ref RE_V: regex::Regex = regex::Regex::new(r"v=([-\d]+),([-\d]+)").unwrap();
}

#[derive(Clone)]
struct Robot {
    p: Vec2,
    v: Vec2,
    f: Vec2,
}

impl Robot {
    fn from_str(s: &str) -> Self {
        Robot {
            p: Vec2::parse_with_regex(s, &RE_P),
            v: Vec2::parse_with_regex(s, &RE_V),
            f: get_field(),
        }
    }

    fn step(&mut self, n: isize) {
        self.p = (self.p + &self.v * n) % self.f;
    }
}

pub struct RestroomRedoubt(Vec<Robot>);

impl RestroomRedoubt {
    fn display(robots: &[Robot]) {
        let field = get_field();
        let mut field = vec![vec![0; field.x as usize]; field.y as usize];
        for r in robots {
            field[r.p.y as usize][r.p.x as usize] += 1;
        }
        for row in field {
            for cell in row {
                print!("{} ", if cell > 0 { cell.to_string() } else { ".".to_string() });
            }
            println!();
        }
    }
}

impl AocDay for RestroomRedoubt {
    fn new(content: String) -> Self {
        RestroomRedoubt(content.lines().map(Robot::from_str).collect())
    }

    fn part1(&self) -> String {
        let field = get_field();
        let width = field.x;
        let height = field.y;
        let mut robots = self.0.clone();
        for r in &mut robots {
            r.step(100);
        }
        let lt = robots.iter().filter(|r| r.p.x != width as isize / 2 && r.p.y != height as isize / 2 && r.p.x < width as isize / 2 && r.p.y < height as isize / 2).count();
        let rt = robots.iter().filter(|r| r.p.x != width as isize / 2 && r.p.y != height as isize / 2 && r.p.x > width as isize / 2 && r.p.y < height as isize / 2).count();
        let lb = robots.iter().filter(|r| r.p.x != width as isize / 2 && r.p.y != height as isize / 2 && r.p.x < width as isize / 2 && r.p.y > height as isize / 2).count();
        let rb = robots.iter().filter(|r| r.p.x != width as isize / 2 && r.p.y != height as isize / 2 && r.p.x > width as isize / 2 && r.p.y > height as isize / 2).count();

        (lt * rt * lb * rb).to_string()
    }

    #[cfg(not(test))]
    fn part2(&self) -> String {
        let field = get_field();
        let width = field.x;
        let height = field.y;
        let mut robots = self.0.clone();
        let mut step = 0isize;
        let mut steps = 0;
        loop {
            for r in &mut robots {
                r.step(step);
            }

            steps += step;

            Self::display(&robots);
            // read one character from stdin
            println!("NEED FOR SPEED, LAST STEP: {}, CUR {}", step, steps);
            let ch = crate::utils::term::getch();
            let speed = crate::utils::term::ch_to_speed(ch);
            step = crate::utils::term::speed_scaling(speed);
        }
    }

    #[cfg(test)]
    fn part2(&self) -> String {
        0.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing() {
        let content = "p=2,4 v=2,-3".to_string();
        let robot = Robot::from_str(&content);
        assert_eq!(robot.p, Vec2::new(2, 4));
        assert_eq!(robot.v, Vec2::new(2, -3));
    }

    #[test]
    fn single() {
        let content = "p=2,4 v=2,-3".to_string();
        let mut robot = Robot::from_str(&content);
        robot.step(1);
        assert_eq!(robot.p, Vec2::new(4, 1));
        assert_eq!(robot.v, Vec2::new(2, -3));

        robot.step(1);
        assert_eq!(robot.p, Vec2::new(6, 5));
    }
}