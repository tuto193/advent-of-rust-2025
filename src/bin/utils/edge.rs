use super::point2d::Point2D;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Orientation {
    Collinear,
    Clockwise,
    Counterclockwise,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    pub a: Point2D,
    pub b: Point2D,
}

impl Orientation {
    /// Find orientation of ordered triplet (p, self.a, self.b)
    /// 0 --> p, q and r are collinear
    /// 1 --> Clockwise
    /// 2 --> Counterclockwise
    pub fn from_points(p: Point2D, q: Point2D, r: Point2D) -> Self {
        let val = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
        match val {
            0 => Self::Collinear,
            x if x > 0 => Self::Clockwise,
            _ => Self::Counterclockwise,
        }
    }
}

impl Edge {
    fn on_segment(&self, p: Point2D) -> bool {
        p.x <= self.a.x.max(self.b.x)
            && p.x >= self.a.x.min(self.b.x)
            && p.y <= self.a.y.max(self.b.y)
            && p.y >= self.a.y.min(self.b.y)
    }
    pub fn intersects(&self, other: &Edge) -> bool {
        // find the four orientations needed
        // for general and special cases
        let o1 = Orientation::from_points(self.a, self.b, other.a);
        let o2 = Orientation::from_points(self.a, self.b, other.b);
        let o3 = Orientation::from_points(other.a, other.b, self.a);
        let o4 = Orientation::from_points(other.a, other.b, self.b);

        // general case
        if o1 != o2 && o3 != o4 {
            return true;
        }

        // special cases
        // p1, q1 and p2 are collinear and p2 lies on segment p1q1
        if o1 == Orientation::Collinear && self.on_segment(other.a) {
            return true;
        }

        // p1, q1 and q2 are collinear and q2 lies on segment p1q1
        if o2 == Orientation::Collinear && self.on_segment(other.b) {
            return true;
        }

        // p2, q2 and p1 are collinear and p1 lies on segment p2q2
        if o3 == Orientation::Collinear && other.on_segment(self.a) {
            return true;
        }

        // p2, q2 and q1 are collinear and q1 lies on segment p2q2
        if o4 == Orientation::Collinear && other.on_segment(self.b) {
            return true;
        }

        return false;
    }

    ///////// OLD IMPLEMENTATION
    // fn ccw(a: Point2D, b: Point2D, c: Point2D) -> bool {
    //     let a0 = a.x;
    //     let a1 = a.y;
    //     let b0 = b.x;
    //     let b1 = b.y;
    //     let c0 = c.x;
    //     let c1 = c.y;
    //     (c1 - a1) * (b0 - a0) > (b1 - a1) * (c0 - a0)
    // }

    // pub fn intersects(&self, other: &Self) -> bool {
    //     let (a, b) = (self.a, self.b);
    //     let (c, d) = (other.a, other.b);
    //     Self::ccw(a, c, d) != Self::ccw(b, c, d) &&
    //     // For formatting
    //     Self::ccw(a, b, c) != Self::ccw(a, b, d)
    // }
    /////////////////////////////
}
