use std::{collections::HashSet, fmt::Display};

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

impl Display for JBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let junction_boxes: Vec<JBox> = input.split('\n').map(|l| JBox::from_str(l)).collect();
    let mut jb_connected_to = vec![junction_boxes.len(); junction_boxes.len()];
    // Find closes neighbor for each box (this_box's_index, other_box's_index)
    junction_boxes.iter().enumerate().for_each(|(i, this_jb)| {
        // Find closest neighbor for each JBox
        let (j, _other_jb) = junction_boxes
            .iter()
            .enumerate()
            .filter(|other| other.0 != i)
            .min_by_key(|(j, other)| other.distance_to(*this_jb))
            .unwrap();
        jb_connected_to[i] = j;
    });

    let mut circuits: Vec<HashSet<usize>> = vec![];
    let mut jb_ix_and_connection = jb_connected_to
        .into_iter()
        .enumerate()
        .dedup_by(|this, other| this.0 == other.1 && this.1 == other.0)
        .sorted_by(|this, other| {
            let this_distance = junction_boxes[this.0].distance_to(junction_boxes[this.1]);
            let other_distance = junction_boxes[other.0].distance_to(junction_boxes[other.1]);

            this_distance.cmp(&other_distance)
        })
        // .take(10)
        .collect::<Vec<(usize, usize)>>();
    println!("Sorted connections: {jb_ix_and_connection:?}");
    for (ix, (i, j)) in jb_ix_and_connection.iter().enumerate() {
        let first = junction_boxes[*i];
        let second = junction_boxes[*j];
        println!("--> {ix} -> {first}···{second}");
    }

    // Take only the first 10 ones
    let mut already_taken = 0;
    // Only connect the first closest
    while already_taken < 10 {
        let (current_jb_index, current_jb_connected_to) = jb_ix_and_connection.pop().unwrap();
        if let Some(_circuit) = circuits
            .iter()
            .find(|c| c.contains(&current_jb_index) || c.contains(&current_jb_connected_to))
        {
            // None of the current circuits have them inside
            let c_index = circuits
                .iter()
                .position(|c| c.contains(&current_jb_index) || c.contains(&current_jb_connected_to))
                .unwrap();
            if circuits[c_index].insert(current_jb_index)
                || circuits[c_index].insert(current_jb_connected_to)
            {
                already_taken += 1;
            }
        } else {
            let mut new_set: HashSet<usize> = HashSet::new();
            new_set.insert(current_jb_index);
            new_set.insert(current_jb_connected_to);
            circuits.push(new_set);
            already_taken += 1;
        }
    }

    println!("Final circuits formed:");
    for (i, circuit) in circuits.iter().enumerate() {
        println!("-> Circuit {i} = {circuit:?}");
    }

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
    None
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
