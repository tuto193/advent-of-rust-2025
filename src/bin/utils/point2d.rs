use std::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Point2D {
    pub x: i64,
    pub y: i64,
}

impl Add<Point2D> for Point2D {
    type Output = Point2D;

    fn add(self, rhs: Point2D) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Point2D> for Point2D {
    fn add_assign(&mut self, rhs: Point2D) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Point2D {
    pub fn area_with(&self, other: &Self) -> u64 {
        let x = self.x.abs_diff(other.x) + 1;
        let y = self.y.abs_diff(other.y) + 1;
        x * y
    }
}
