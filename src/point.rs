//! This should be in the GUI module.

use std::ops::{Add, Sub, Div};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Point {
    pub x: u16,
    pub y: u16
}
impl Point {
    pub fn new(x: u16, y: u16) -> Point {
        Point {
            x: x,
            y: y
        }
    }
    pub fn empty() -> Point {
        Point {
            x: 0,
            y: 0
        }
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point {x: self.x - other.x, y: self.y - other.y}
    }
}
impl Div<u16> for Point {
    type Output = Self;

    fn div(self, div: u16) -> Self {
        Point::new(self.x / 2, self.y / 2)
    }
}
