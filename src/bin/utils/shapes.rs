use std::collections::HashSet;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Node(u64, u64);

#[derive(Copy, Clone, Debug)]
pub struct Rect {
    top_left: Node,
    bot_right: Node,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Edge {
    a: Node,
    b: Node,
}

#[derive(Clone, Debug)]
pub struct Polygon {
    corners: Vec<Node>,
    edges: Vec<Edge>,
}

impl Node {
    pub fn area_with(&self, other: &Self) -> u64 {
        let x = self.0.abs_diff(other.0) + 1;
        let y = self.1.abs_diff(other.1) + 1;
        x * y
    }
}

impl Edge {
    fn ccw(a: Node, b: Node, c: Node) -> bool {
        (c.1 - a.1) * (b.0 - a.0) > (b.1 - a.1) * (c.0 - a.0)
    }

    pub fn intersects(&self, other: &Self) -> bool {
        let (a, b) = (self.a, self.b);
        let (c, d) = (other.a, other.b);
        Self::ccw(a, c, d) != Self::ccw(b, c, d) && Self::ccw(a, b, c) != Self::ccw(a, b, d)
    }
}

impl Rect {
    pub fn new(c1: &Node, c2: &Node) -> Self {
        let top_left = Node(c1.0.min(c2.0), c1.1.min(c2.1));
        let bot_right = Node(c1.0.max(c2.0), c1.1.max(c2.1));
        Self {
            top_left,
            bot_right,
        }
    }
    pub fn area(&self) -> u64 {
        self.top_left.area_with(&self.bot_right)
    }

    pub fn has_node_inside(&self, c: &Node) -> bool {
        self.top_left.0 < c.0
            && c.0 < self.bot_right.0
            && self.top_left.1 < c.1
            && c.1 < self.bot_right.1
    }

    pub fn has_nodes_inside(&self, cs: &Vec<Node>) -> bool {
        // It contains no other edges inside
        return cs.iter().any(|c| self.has_node_inside(c));
    }
}

impl From<&str> for Node {
    fn from(value: &str) -> Self {
        let values = value
            .split(',')
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        Self(values[0], values[1])
    }
}

impl Polygon {
    fn get_edges_from_nodes(nodes: &Vec<Node>) -> Vec<Edge> {
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
    pub fn from_nodes(nodes: &Vec<Node>) -> Self {
        let edges = Self::get_edges_from_nodes(&nodes);
        Self {
            corners: nodes.clone(),
            edges,
        }
    }

    pub fn is_rect_in_polygon(&self, p: &Rect) -> bool {
        let origin = Node(0, 0);
        let top_l = p.top_left;
        let bot_r = p.bot_right;
        let top_r = Node(bot_r.0, top_l.1);
        let bot_l = Node(top_l.0, bot_r.1);
        let mut base_nodes = vec![top_l, top_r, bot_l, bot_r];
        let mut centroid = Node(0, 0);
        base_nodes.iter().for_each(|n| {
            centroid.0 += n.0;
            centroid.1 += n.1;
        });
        let centroid = Node(centroid.0 / 4, centroid.1 / 4);
        base_nodes.push(centroid);
        let check_segments = base_nodes
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
