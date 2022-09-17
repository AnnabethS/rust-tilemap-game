use std::{fmt, ops::{AddAssign, Add}};

#[derive(Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point { x, y }
    }

    pub fn to_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn dist(&self, other: &Point) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }

    pub fn between(p1: &Point, p2: &Point, check: &Point) -> bool {
        (p1.x < check.x && check.x < p2.x) || (p2.x < check.x && check.x < p1.x) &&
            (p1.y < check.y && check.y < p2.y) || (p2.y < check.y && check.y < p1.y)
    }

    pub fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.add(&rhs);
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        let mut out = self.clone();
        Point::add(&mut out, &rhs);
        out
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}
