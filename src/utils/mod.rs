pub mod term;
pub mod direction;

use std::fmt::Debug;
use std::ops::{Add, AddAssign, Mul, Rem};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec2<T = isize> {
    pub x: T,
    pub y: T,
}

impl Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "V({}, {})", self.x, self.y)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Vec2> for Vec2 {
    type Output = Self;

    fn add(self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<&Vec2> for &Vec2 {
    type Output = Vec2;

    fn add(self, other: &Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}


impl Mul<isize> for &Vec2<isize>
{
    type Output = Vec2<isize>;

    fn mul(self, rhs: isize) -> Self::Output {
        Vec2 { x: self.x * rhs, y: self.y * rhs }
    }
}


impl AddAssign<&Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: &Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}


impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Rem<Self> for Vec2 {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        let mut new_x = self.x % other.x;
        let mut new_y = self.y % other.y;

        while new_x < 0 {
            new_x += other.x;
        }

        while new_y < 0 {
            new_y += other.y;
        }

        Self {
            x: new_x,
            y: new_y,
        }
    }
}


impl Vec2 {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn parse_with_regex(input: &str, regex: &regex::Regex) -> Self {
        let captures = regex.captures(input).unwrap();
        let x = captures.get(1).unwrap().as_str().parse().unwrap();
        let y = captures.get(2).unwrap().as_str().parse().unwrap();
        Self { x, y }
    }

    pub fn length(&self) -> f64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }

    pub fn manhattan_length(&self) -> isize {
        self.x.abs() + self.y.abs()
    }

    pub fn rotate(&self, degrees: isize) -> Self {
        let radians = degrees as f64 * std::f64::consts::PI / 180.0;
        let x = (self.x as f64 * radians.cos() - self.y as f64 * radians.sin()).round() as isize;
        let y = (self.x as f64 * radians.sin() + self.y as f64 * radians.cos()).round() as isize;
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn modulo_test_noop() {
        let v = super::Vec2::new(3, 3);
        let f = super::Vec2::new(5, 5);
        let r = v % f;
        assert_eq!(r, super::Vec2::new(3, 3));
    }

    #[test]
    fn modulo_test_negative() {
        let v = super::Vec2::new(-3, -3);
        let f = super::Vec2::new(5, 5);
        let r = v % f;
        assert_eq!(r, super::Vec2::new(2, 2));
    }

    #[test]
    fn modulo_test_wrap() {
        let v = super::Vec2::new(3, 3);
        let f = super::Vec2::new(2, 2);
        let r = v % f;
        assert_eq!(r, super::Vec2::new(1, 1));
    }
}