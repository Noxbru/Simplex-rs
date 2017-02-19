use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::Sub;

#[derive(Clone,Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl<'a> AddAssign<&'a Point> for Point {
    fn add_assign(&mut self, other: &Point) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, other: f64) -> Point {
        Point {
            x: self.x / other,
            y: self.y / other
        }
    }
}

impl DivAssign<f64> for Point {
    fn div_assign(&mut self, other: f64) {
        *self =Point {
            x: self.x / other,
            y: self.y / other
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;

    fn mul(self, other: f64) -> Point {
        Point {
            x: self.x * other,
            y: self.y * other
        }
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {
            x: self * other.x,
            y: self * other.y
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}
