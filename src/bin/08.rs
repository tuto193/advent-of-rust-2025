use std::{collections::HashSet, fmt::Display};

use itertools::Itertools;

advent_of_code::solution!(8);

#[derive(Eq, Debug, Clone, Copy)]
struct Connection(usize, usize);

impl PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        let low1 = self.0.min(self.1);
        let low2 = other.0.min(other.1);
        let hi1 = self.0.max(self.1);
        let hi2 = other.0.max(other.1);
        low1 == low2 && hi1 == hi2
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct JBox {
    x: u64,
    y: u64,
    z: u64,
}

impl JBox {
    pub fn from_str(input: &str) -> Self {
        let vals = input.split(',').collect::<Vec<&str>>();
        Self {
            x: vals[0].parse::<u64>().unwrap(),
            y: vals[1].parse::<u64>().unwrap(),
            z: vals[2].parse::<u64>().unwrap(),
        }
    }

    pub fn length_squared(&self) -> u64 {
        self.x.pow(2) + self.y.pow(2) + self.z.pow(2)
    }

    pub fn positive_vector_to(&self, other: Self) -> Self {
        Self {
            x: other.x.abs_diff(self.x),
            y: other.y.abs_diff(self.y),
            z: other.z.abs_diff(self.z),
        }
    }

    pub fn add(&self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    pub fn distance_squared_to(&self, other: Self) -> u64 {
        self.positive_vector_to(other).length_squared()
    }
}

impl Display for JBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let junction_boxes: Vec<JBox> = input.split('\n').map(|l| JBox::from_str(l)).collect();
    let pairs_to_take = 1000;
    // let mut jb_pairs = vec![(Connection(1001, 1002), u64::MAX); junction_boxes.len().pow(2)];
    let mut jb_pairs: Vec<(Connection, u64)> = Vec::with_capacity(junction_boxes.len().pow(2));
    // Find closes neighbor for each box (this_box's_index, other_box's_index)
    junction_boxes.iter().enumerate().for_each(|(i, this_jb)| {
        // Find all pairs for this box with others
        let mut all_pairs = junction_boxes
            .iter()
            .enumerate()
            .skip(i + 1)
            .map(|(j, other)| {
                let distance = this_jb.distance_squared_to(*other);
                (Connection(i, j), distance)
            })
            .collect::<Vec<_>>();
        jb_pairs.append(&mut all_pairs);
    });

    let mut circuits: Vec<HashSet<usize>> = vec![];
    let mut jb_ix_and_cx = jb_pairs
        .into_iter()
        .sorted_by(|this, other| {
            let this_cx_distance = this.1;
            let other_cx_distance = other.1;

            this_cx_distance.cmp(&other_cx_distance)
        })
        .map(|(c, _len)| c)
        .dedup()
        .take(pairs_to_take)
        .collect::<Vec<Connection>>();
    jb_ix_and_cx.reverse();
    while let Some(connection) = jb_ix_and_cx.pop() {
        let (this_jb, other_jb) = (connection.0, connection.1);

        let circ_this_jb = circuits.iter().position(|c| c.contains(&this_jb));
        let circ_other_jb = circuits.iter().position(|c| c.contains(&other_jb));
        if circ_this_jb.is_some() && circ_other_jb.is_some() {
            // Both exist
            let circ_this_jb = circ_this_jb.unwrap();
            let circ_other_jb = circ_other_jb.unwrap();
            if circ_this_jb != circ_other_jb {
                // They are not in the same group, so merge the groups
                let this_circuit = circuits[circ_this_jb].clone();
                let other_circuit = circuits[circ_other_jb].clone();
                let merged_circuits = this_circuit
                    .union(&other_circuit)
                    .map(|n| *n)
                    .collect::<HashSet<usize>>();
                // Remove the higher one, then replace the lower one
                let higher = circ_this_jb.max(circ_other_jb);
                let lower = circ_this_jb.min(circ_other_jb);
                circuits.swap_remove(higher);
                circuits[lower] = merged_circuits;
            }
        } else {
            // One of them (maybe both) is None
            if circ_this_jb.is_none() && circ_other_jb.is_none() {
                // Both are None, so just create a new set with both of this then add them
                let new_circuit = HashSet::from([this_jb, other_jb]);
                circuits.push(new_circuit);
            } else {
                // Find the already existing circuit
                let circ_ix = circ_this_jb.unwrap_or(circ_other_jb.unwrap_or(69)); // It's never going to be this 69 :(
                let mut circuit = circuits[circ_ix].clone();
                circuit.insert(this_jb);
                circuit.insert(other_jb);
                circuits[circ_ix] = circuit;
            }
        }
    }

    println!("Final circuits formed:");
    for (i, circuit) in circuits.iter().enumerate() {
        println!("-> Circuit {i} = {circuit:?}");
    }
    println!("Largest circuits");
    for circuit in circuits
        .iter()
        .sorted_by(|i, j| i.len().cmp(&j.len()))
        .rev()
        .take(3)
    {
        println!("{circuit:?}");
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
    let junction_boxes: Vec<JBox> = input.split('\n').map(|l| JBox::from_str(l)).collect();
    // let pairs_to_take = 1000; // ALLLL
    // let mut jb_pairs = vec![(Connection(1001, 1002), u64::MAX); junction_boxes.len().pow(2)];
    let mut jb_pairs: Vec<(Connection, u64)> = Vec::with_capacity(junction_boxes.len().pow(2));
    // Find closes pairs for each box (2^n combinations)
    junction_boxes.iter().enumerate().for_each(|(i, this_jb)| {
        // Find all pairs for this box with others
        let mut all_pairs = junction_boxes
            .iter()
            .enumerate()
            .skip(i + 1)
            .map(|(j, other)| {
                let distance = this_jb.distance_squared_to(*other);
                (Connection(i, j), distance)
            })
            .collect::<Vec<_>>();
        jb_pairs.append(&mut all_pairs);
    });

    let mut circuits: Vec<HashSet<usize>> = vec![];
    let mut jb_ix_and_cx = jb_pairs
        .into_iter()
        .sorted_by(|this, other| {
            let this_cx_distance = this.1;
            let other_cx_distance = other.1;

            this_cx_distance.cmp(&other_cx_distance)
        })
        .map(|(c, _len)| c)
        .dedup()
        // .take(pairs_to_take) // we take all
        .collect::<Vec<Connection>>();
    jb_ix_and_cx.reverse();
    let mut final_products: Vec<u64> = vec![];
    while let Some(connection) = jb_ix_and_cx.pop() {
        if circuits.len() == 1
            && (0..junction_boxes.len()).all(|jb_ix| circuits.iter().any(|c| c.contains(&jb_ix)))
        {
            // We are done
            break;
        }
        let (this_jb, other_jb) = (connection.0, connection.1);

        let circ_this_jb = circuits.iter().position(|c| c.contains(&this_jb));
        let circ_other_jb = circuits.iter().position(|c| c.contains(&other_jb));
        if circ_this_jb.is_some() && circ_other_jb.is_some() {
            // Both exist
            let circ_this_jb = circ_this_jb.unwrap();
            let circ_other_jb = circ_other_jb.unwrap();
            if circ_this_jb != circ_other_jb {
                let products_of_xes = junction_boxes[this_jb].x * junction_boxes[other_jb].x;
                final_products.push(products_of_xes);
                // They are not in the same group, so merge the groups
                let this_circuit = circuits[circ_this_jb].clone();
                let other_circuit = circuits[circ_other_jb].clone();
                let merged_circuits = this_circuit
                    .union(&other_circuit)
                    .map(|n| *n)
                    .collect::<HashSet<usize>>();
                // Remove the higher one, then replace the lower one
                let higher = circ_this_jb.max(circ_other_jb);
                let lower = circ_this_jb.min(circ_other_jb);
                circuits.swap_remove(higher);
                circuits[lower] = merged_circuits;
            }
        } else {
            // One of them (maybe both) is None
            if circ_this_jb.is_none() && circ_other_jb.is_none() {
                // Both are None, so just create a new set with both of this then add them
                let new_circuit = HashSet::from([this_jb, other_jb]);
                circuits.push(new_circuit);
            } else {
                // Find the already existing circuit
                let products_of_xes = junction_boxes[this_jb].x * junction_boxes[other_jb].x;
                final_products.push(products_of_xes);
                let circ_ix = circ_this_jb.unwrap_or(circ_other_jb.unwrap_or(69)); // It's never going to be this 69 :(
                let mut circuit = circuits[circ_ix].clone();
                circuit.insert(this_jb);
                circuit.insert(other_jb);
                circuits[circ_ix] = circuit;
            }
        }
    }

    Some(final_products.pop().unwrap())
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
        assert_eq!(result, Some(25272));
    }
}
