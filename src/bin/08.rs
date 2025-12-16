use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Copy, Clone, Debug)]
pub struct JBox {
    x: i64,
    y: i64,
    z: i64,
}

impl JBox {
    pub fn from_str(input: &str) -> Self {
        let vals = input.split(',').collect::<Vec<&str>>();
        Self {
            x: vals[0].parse::<i64>().unwrap(),
            y: vals[1].parse::<i64>().unwrap(),
            z: vals[2].parse::<i64>().unwrap(),
        }
    }

    pub fn length(&self) -> u64 {
        (((self.x.pow(2) + self.y.pow(2) + self.z.pow(2)) as f64).sqrt()) as u64
    }

    pub fn vector_to(&self, other: Self) -> Self {
        Self {
            x: other.x - self.x,
            y: other.y - self.y,
            z: other.z - self.z,
        }
    }

    pub fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn distance_to(&self, other: Self) -> u64 {
        self.vector_to(other).length()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let junction_boxes: Vec<JBox> = input.split('\n').map(|l| JBox::from_str(l)).collect();
    let mut jb_ix_and_connection = vec![junction_boxes.len(); junction_boxes.len()];

    junction_boxes.iter().enumerate().for_each(|(i, jb)| {
        // Find closest neighbor for each JBox
        let j = junction_boxes
            .iter()
            .position_min_by_key(|&other| other.distance_to(*jb) as u64)
            .unwrap();
        jb_ix_and_connection[i] = j;
    });
    let mut circuits: Vec<HashSet<usize>> = vec![];
    let mut jb_ix_and_connection = jb_ix_and_connection
        .into_iter()
        .enumerate()
        .sorted_by(|this, other| {
            junction_boxes[this.0]
                .distance_to(junction_boxes[this.1])
                .cmp(&junction_boxes[other.0].distance_to(junction_boxes[other.1]))
        })
        .collect::<Vec<(usize, usize)>>();
    while let Some((current_jb_index, current_jb_connected_to)) = jb_ix_and_connection.pop() {
        let mut current_group: HashSet<usize> = HashSet::new();
        current_group.insert(current_jb_index);
        current_group.insert(current_jb_connected_to);
        // We popped the last element, so we filter to see what we can add where
        'current_group: loop {
            let new_connections_to_add = jb_ix_and_connection
                .clone()
                .into_iter()
                .filter(|(index, connected_to)| {
                    current_group.contains(&connected_to) ^ current_group.contains(&index)
                })
                .collect::<Vec<(usize, usize)>>();
            if new_connections_to_add.is_empty() {
                break 'current_group;
            }
            new_connections_to_add
                .into_iter()
                .for_each(|(new_index, connected_to)| {
                    current_group.insert(new_index);
                    current_group.insert(connected_to);
                });
        }
        circuits.push(current_group);
        let new_connected_to = jb_ix_and_connection
            .clone()
            .into_iter()
            .filter(|(i, connected_to)| {
                !circuits
                    .iter()
                    .any(|g| g.contains(i) || g.contains(connected_to))
            })
            .collect::<Vec<(usize, usize)>>();
        jb_ix_and_connection = new_connected_to;
    }

    println!("{circuits:?}");

    Some(
        circuits
            .iter()
            .map(|g| g.len())
            .sorted()
            .rev()
            .take(3)
            .product::<usize>() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let junction_boxes: Vec<JBox> = input.split('\n').map(|l| JBox::from_str(l)).collect();
    let mut jb_ix_and_connection = vec![junction_boxes.len(); junction_boxes.len()];

    junction_boxes.iter().enumerate().for_each(|(i, jb)| {
        // Find closest neighbor
        let j = junction_boxes
            .iter()
            .position_min_by_key(|&other| other.distance_to(*jb) as u64)
            .unwrap();
        jb_ix_and_connection[i] = j;
    });
    let mut circuits: Vec<HashSet<usize>> = vec![];
    let mut jb_ix_and_connection = jb_ix_and_connection
        .into_iter()
        .enumerate()
        .collect::<Vec<(usize, usize)>>();
    while let Some((current_jb_index, current_jb_connected_to)) = jb_ix_and_connection.pop() {
        let mut current_group: HashSet<usize> = HashSet::new();
        current_group.insert(current_jb_index);
        current_group.insert(current_jb_connected_to);
        // We popped the last element, so we filter to see what we can add where
        'current_group: loop {
            let new_connections_to_add = jb_ix_and_connection
                .clone()
                .into_iter()
                .filter(|(index, connected_to)| {
                    current_group.contains(&connected_to) ^ current_group.contains(&index)
                })
                .collect::<Vec<(usize, usize)>>();
            if new_connections_to_add.is_empty() {
                break 'current_group;
            }
            new_connections_to_add
                .into_iter()
                .for_each(|(new_index, connected_to)| {
                    current_group.insert(new_index);
                    current_group.insert(connected_to);
                });
        }
        circuits.push(current_group);
        let new_connected_to = jb_ix_and_connection
            .clone()
            .into_iter()
            .filter(|(i, connected_to)| {
                !circuits
                    .iter()
                    .any(|g| g.contains(i) || g.contains(connected_to))
            })
            .collect::<Vec<(usize, usize)>>();
        jb_ix_and_connection = new_connected_to;
    }

    println!("{circuits:?}");

    Some(
        circuits
            .iter()
            .map(|g| g.len())
            .sorted()
            .rev()
            .take(3)
            .product::<usize>() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
