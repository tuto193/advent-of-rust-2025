advent_of_code::solution!(9);

#[derive(Copy, Clone, Debug, PartialEq)]
struct Corner(u64, u64);

#[derive(Copy, Clone, Debug)]
struct Rect {
    top_left: Corner,
    bot_right: Corner,
}

impl Corner {
    pub fn area_with(&self, other: &Self) -> u64 {
        let x = self.0.abs_diff(other.0) + 1;
        let y = self.1.abs_diff(other.1) + 1;
        x * y
    }
}

impl Rect {
    pub fn new(c1: &Corner, c2: &Corner) -> Self {
        let top_left = Corner(c1.0.min(c2.0), c1.1.min(c2.1));
        let bot_right = Corner(c1.0.max(c2.0), c1.1.max(c2.1));
        Self {
            top_left,
            bot_right,
        }
    }
    pub fn area(&self) -> u64 {
        self.top_left.area_with(&self.bot_right)
    }

    pub fn has_corner_inside(&self, c: &Corner) -> bool {
        self.top_left.0 < c.0
            && c.0 < self.bot_right.0
            && self.top_left.1 < c.1
            && c.1 < self.bot_right.1
    }

    pub fn is_within_shape(&self, cs: &Vec<Corner>) -> bool {
        let bot_left = Corner(self.top_left.0, self.bot_right.1);
        let top_right = Corner(self.bot_right.0, self.top_left.1);
        let edges = vec![self.top_left, self.bot_right, top_right, bot_left];
        let edges_NOT_in_shape = edges
            .into_iter()
            .filter(|e| !cs.contains(e))
            .collect::<Vec<Corner>>();
        let last_edge = edges_NOT_in_shape[0];
        // It contains no other edges inside
        cs.iter().any(|c| self.has_corner_inside(c)) &&
        // At least 3 edges must be part of the whole set
        edges_NOT_in_shape.len() < 2 &&
        // Even if the last edge is not PART of the shape, somewhere
        // along the way it does still need to intersect with it

    }
}

impl From<&str> for Corner {
    fn from(value: &str) -> Self {
        let values = value
            .split(',')
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        Self(values[0], values[1])
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let corners = input
        .split('\n')
        .map(|c| Corner::from(c))
        .collect::<Vec<Corner>>();
    println!("Corners are: {corners:?}");
    let mut areas = vec![];
    corners.iter().enumerate().for_each(|(i, c1)| {
        let mut corner_areas = corners
            .iter()
            .skip(i + 1)
            .map(|c2| c2.area_with(c1))
            .collect::<Vec<u64>>();
        areas.append(&mut corner_areas);
    });
    areas.sort();
    Some(areas.pop().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let corners = input
        .split('\n')
        .map(|c| Corner::from(c))
        .collect::<Vec<Corner>>();
    // println!("Corners are: {corners:?}");
    let mut rectangles = vec![];
    corners.iter().enumerate().for_each(|(i, c1)| {
        let mut corner_areas = corners
            .iter()
            .skip(i + 1)
            .map(|c2| Rect::new(c1, c2))
            .collect::<Vec<Rect>>();
        rectangles.append(&mut corner_areas);
    });
    // println!("Rectangles:");
    // for r in rectangles.iter() {
    //     println!("--> {r:?}");
    // }
    let mut areas_of_those_without_inner_corners = rectangles
        .into_iter()
        .filter(|r| !r.is_within_shape(&corners))
        .map(|r| {
            let a = r.area();
            println!("Rect (no corners) with area {a} -> {r:?}");
            a
        })
        .collect::<Vec<u64>>();
    areas_of_those_without_inner_corners.sort();
    println!("Final areas: {areas_of_those_without_inner_corners:?}");
    Some(areas_of_those_without_inner_corners.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
