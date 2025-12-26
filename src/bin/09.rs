use std::collections::HashSet;

mod utils;
use crate::utils::point2d::Point2D;
use crate::utils::shapes::*;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let corners = input
        .split('\n')
        .map(|c| Point2D::from(c))
        .collect::<Vec<Point2D>>();
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
        .map(|c| Point2D::from(c))
        .collect::<Vec<Point2D>>();
    let polygon = Polygon::from_nodes(&corners);
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
    let mut areas_of_those_without_inner_nodes_or_inside_polygon = rectangles
        .into_iter()
        .filter(|r| !r.has_nodes_inside(&corners) && !polygon.is_rect_in_polygon(r))
        .map(|r| {
            let a = r.area();
            println!("Rect (no corners) with area {a} -> {r:?}");
            a
        })
        .collect::<Vec<u64>>();
    areas_of_those_without_inner_nodes_or_inside_polygon.sort();
    println!("Final areas: {areas_of_those_without_inner_nodes_or_inside_polygon:?}");
    Some(
        areas_of_those_without_inner_nodes_or_inside_polygon
            .pop()
            .unwrap(),
    )
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
