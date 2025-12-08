use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_splits = 0;
    let mut grid: Vec<Vec<u8>> = input.split("\n").map(|row| row.bytes().collect()).collect();
    let starting_x = grid[0].iter().find_position(|&s| *s == b'S').unwrap().0;
    grid[1][starting_x] = b'|';
    let mut x_with_beam_behind: Vec<usize> = vec![starting_x];
    for row in 2..grid.len() - 1 {
        let mut new_affected_xes = vec![];
        for x_beam in x_with_beam_behind.iter() {
            match grid[row][*x_beam] {
                b'^' => {
                    // Will split the beam!
                    total_splits += 1;
                    // Copy it it to the left, right, down-left and down-right
                    grid[row][*x_beam - 1] = b'|';
                    grid[row][*x_beam + 1] = b'|';
                    grid[row + 1][*x_beam - 1] = b'|';
                    grid[row + 1][*x_beam + 1] = b'|';
                    new_affected_xes.push(*x_beam + 1);
                    new_affected_xes.push(*x_beam - 1);
                }
                _ => {
                    // This means, we are either empty or on a beam, so we just
                    // need to pass it as is
                    grid[row][*x_beam] = b'|';
                    new_affected_xes.push(*x_beam);
                }
            }
        }
        new_affected_xes.sort();
        new_affected_xes.dedup();
        x_with_beam_behind = new_affected_xes;
    }
    Some(total_splits)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total_timelines = 1;
    let mut grid: Vec<Vec<u8>> = input.split("\n").map(|row| row.bytes().collect()).collect();
    let starting_x = grid[0].iter().find_position(|&s| *s == b'S').unwrap().0;
    grid[1][starting_x] = b'|';
    let mut x_with_beam_behind: Vec<usize> = vec![starting_x];
    for row in 2..grid.len() - 1 {
        let mut new_affected_xes = vec![];
        for x_beam in x_with_beam_behind.iter() {
            match grid[row][*x_beam] {
                b'^' => {
                    // Will split the beam!
                    total_timelines += 1;
                    // Copy it it to the left, right, down-left and down-right
                    grid[row][*x_beam - 1] = b'|';
                    grid[row][*x_beam + 1] = b'|';
                    grid[row + 1][*x_beam - 1] = b'|';
                    grid[row + 1][*x_beam + 1] = b'|';
                    new_affected_xes.push(*x_beam + 1);
                    new_affected_xes.push(*x_beam - 1);
                }
                _ => {
                    // This means, we are either empty or on a beam, so we just
                    // need to pass it as is
                    grid[row][*x_beam] = b'|';
                    new_affected_xes.push(*x_beam);
                }
            }
        }
        new_affected_xes.sort();
        new_affected_xes.dedup();
        x_with_beam_behind = new_affected_xes;
    }
    Some(total_timelines - 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
