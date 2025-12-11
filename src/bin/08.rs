use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(8);

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

    pub fn length(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2) + self.z.pow(2)) as f64).sqrt()
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

    pub fn distance_to(&self, other: Self) -> f64 {
        self.vector_to(other).length()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let single_boxes: Vec<JBox> = input.split('\n').map(|l| JBox::from_str(l)).collect();
    // let mut grouped_boxes: Vec<Vec<JBox>> = vec![];
    // let mut connected_to_and_group = &[(1001usize, 1001usize); 1000];
    let mut connected_to = vec![1001usize; 1000];

    single_boxes.iter().enumerate().for_each(|(i, jb)| {
        // Find closest neighbor
        // let this_jbs_potential_group = connected_to_and_group[i].1;
        let closest_jb_index = single_boxes
            .iter()
            .position_min_by_key(|&other| other.distance_to(*jb) as u64)
            .unwrap();
        connected_to[i] = closest_jb_index;
    });
    let mut groups: Vec<HashSet<usize>> = vec![];
    let mut grouped: HashSet<usize> = HashSet::new();
    'new_group: loop {
        let this_jb = connected_to.len() - 1;
        let mut current_group: HashSet<usize> = HashSet::new();
        if let Some(other_jb) = connected_to.pop() {
            current_group.insert(this_jb);
            current_group.insert(other_jb);
        } else {
            // all empty
            break;
        }
        // We popped the last element, so we filter to see what we can add where
        'current_group: loop {
            connected_to
                .iter()
                .enumerate()
                .filter(|(index, &connected)| {
                    current_group.contains(&connected) && !current_group.contains(&index)
                })
                .for_each(|(new_index, _)| current_group.insert(new_index););
        }
    }
    Some(69)
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
