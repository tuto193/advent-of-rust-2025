use std::i64;

use super::edge::Edge;
use super::point2d::Point2D;

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    top_left: Point2D,
    bot_right: Point2D,
}

#[derive(Clone, Debug)]
pub struct Polygon {
    corners: Vec<Point2D>,
    edges: Vec<Edge>,
}

impl Rect {
    pub fn new(c1: &Point2D, c2: &Point2D) -> Self {
        let top_left = Point2D {
            x: c1.x.min(c2.x),
            y: c1.y.min(c2.y),
        };
        let bot_right = Point2D {
            x: c1.x.max(c2.x),
            y: c1.y.max(c2.y),
        };
        Self {
            top_left,
            bot_right,
        }
    }
    pub fn area(&self) -> u64 {
        self.top_left.area_with(&self.bot_right)
    }

    pub fn has_node_inside(&self, c: &Point2D) -> bool {
        self.top_left.x < c.x
            && c.x < self.bot_right.x
            && self.top_left.y < c.y
            && c.y < self.bot_right.y
    }

    pub fn has_nodes_inside(&self, cs: &Vec<Point2D>) -> bool {
        // It contains no other edges inside
        return cs.iter().any(|c| self.has_node_inside(c));
    }
}

impl From<&str> for Point2D {
    fn from(value: &str) -> Self {
        let values = value
            .split(',')
            .map(|n| n.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        Self {
            x: values[0],
            y: values[1],
        }
    }
}

impl Polygon {
    fn get_edges_from_nodes(nodes: &Vec<Point2D>) -> Vec<Edge> {
        let mut result = vec![];
        for i in 0..nodes.len() - 1 {
            let a = nodes[i];
            let b = nodes[i + 1];
            result.push(Edge { a, b });
        }
        // The last Node is connected to the first one
        let a = nodes[nodes.len() - 1];
        let b = nodes[0];
        result.push(Edge { a, b });
        result
    }
    pub fn from_nodes(nodes: &Vec<Point2D>) -> Self {
        let edges = Self::get_edges_from_nodes(&nodes);
        Self {
            corners: nodes.clone(),
            edges,
        }
    }

    pub fn is_rect_in_polygon(&self, p: &Rect) -> bool {
        let origin = Point2D { x: 0, y: 0 };
        let infinite_corner = Point2D {
            x: i64::MAX,
            y: i64::MAX,
        };
        let infinite_down = Point2D { x: 0, y: i64::MAX };
        let infinite_right = Point2D { x: i64::MAX, y: 0 };
        let infinite_corners = vec![origin, infinite_corner, infinite_down, infinite_right];
        let top_l = p.top_left;
        let bot_r = p.bot_right;
        let top_r = Point2D {
            x: bot_r.x,
            y: top_l.y,
        };
        let bot_l = Point2D {
            x: top_l.x,
            y: bot_r.y,
        };
        let mut rect_corners = vec![top_l, top_r, bot_l, bot_r];
        let mut centroid = Point2D { x: 0, y: 0 };
        rect_corners.iter().for_each(|n| {
            centroid += *n;
        });
        let centroid = Point2D {
            x: centroid.x / 4,
            y: centroid.y / 4,
        };
        rect_corners.push(centroid);
        let check_segments = rect_corners
            .iter()
            .map(|n| Edge {
                a: origin,
                b: n.clone(),
            })
            .collect::<Vec<Edge>>();
        check_segments
            .iter()
            .all(|e1| self.edges.iter().filter(|e2| e2.intersects(e1)).count() % 2 == 1)
    }
}
