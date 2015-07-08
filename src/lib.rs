use std::ops::{ Sub, Add, Mul, Div };
use std::clone::Clone;

#[derive(Copy, Clone)]
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
