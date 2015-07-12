use std::ops::{ Sub, Add, Mul, Div };
use std::clone::Clone;
use std::cmp::{PartialEq};

#[derive(Copy, Clone, Debug)]
pub struct Point<T> {
    pub x : T,
    pub y : T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x: x, y: y }
    }
}

impl<T: Add<T, Output=T> + Clone> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Point<T> {
        Point { x: self.x + rhs.x,
                y: self.y + rhs.y }
    }
}

impl<T: Sub<T, Output=T> + Clone> Sub for Point<T> {
    type Output = Point<T>;

    fn sub(self, rhs: Point<T>) -> Point<T> {
        Point { x: self.x - rhs.x,
                y: self.y - rhs.y }
    }
}

/// Multiply each element by a constant
impl<T: Mul<T, Output=T> + Copy> Mul<T> for Point<T> {
    type Output = Point<T>;

    fn mul(self, rhs: T) -> Point<T> {
        Point { x: self.x * rhs,
                y: self.y * rhs }
    }
}

/// Divide each element by a constant
impl<T: Div<T, Output=T> + Copy> Div<T> for Point<T> {
    type Output = Point<T>;

    fn div(self, rhs: T) -> Point<T> {
        Point { x: self.x / rhs,
                y: self.y / rhs }
    }
}

impl<T: PartialEq> PartialEq for Point<T> {
    fn eq(&self, other: &Point<T>) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[test]
fn test_eq() {
    assert_eq!(Point::new(1, 2), Point::new(1, 2));

    assert!(Point::new(1, 3) != Point::new(1, 2));
    assert!(Point::new(3, 2) != Point::new(1, 2));
    assert!(Point::new(1, 1) != Point::new(2, 2));

    assert_eq!(Point::new(1f32, 2f32), Point::new(1f32, 2f32));

    assert!(Point::new(1f32, 3f32) != Point::new(1f32, 2f32));
    assert!(Point::new(3f32, 2f32) != Point::new(1f32, 2f32));
    assert!(Point::new(1f32, 1f32) != Point::new(2f32, 2f32));
}

#[test]
fn test_sub() {
    assert_eq!(Point::new(1, 2) - Point::new(1, 2), Point::new(0, 0));
    assert_eq!(Point::new(1f64, 2f64) - Point::new(0f64, 4f64), Point::new(1f64, -2f64));
}

#[test]
fn test_add() {
    assert_eq!(Point::new(1, 2) + Point::new(1, 2), Point::new(2, 4));
}

#[test]
fn test_mul() {
    assert_eq!(Point::new(1, 2) * 5, Point::new(5, 10));
}

#[test]
fn test_div() {
    assert_eq!(Point::new(1, 2) / 5, Point::new(0, 0));
    assert_eq!(Point::new(1f32, 2f32) / 5f32, Point::new(1f32/5f32, 2f32/5f32));
}
