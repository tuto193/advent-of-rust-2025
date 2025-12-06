use std::{str::Chars, usize};
use itertools::Itertools;


advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result = 0;
    let rows_and_columns: Vec<Vec<&str>> = input
        .split('\n') // we have the rows here
        .map(|line| line.split_ascii_whitespace().collect())
        .collect();
    let row_length = rows_and_columns[0].len();
    let all_rows_are_equally_long = rows_and_columns.iter().all(|r| r.len() == row_length);
    let mut columns_sum_prods = vec![(0, 1); rows_and_columns[0].len()];
    rows_and_columns.into_iter().for_each(|row| {
        row.into_iter()
            .enumerate()
            .for_each(|(column, maybe_number)| {
                // print!("Trying to access column #{column}");
                let mut interim_results = columns_sum_prods[column];
                match maybe_number.parse::<u64>() {
                    Ok(number) => {
                        interim_results.0 += number;
                        interim_results.1 *= number;
                        columns_sum_prods[column] = interim_results;
                    }
                    Err(_) => {
                        // Not a number, so we are actually adding the results.
                        match maybe_number {
                            "*" => result += interim_results.1,
                            _ => result += interim_results.0,
                        }
                    }
                }
            })
    });
    Some(result)
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone {
    (0..v[0].len())
            .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
            .collect()
}

fn reverse_columns(v: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    v.into_iter().rev().collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let mut rows_and_columns: Vec<Vec<String>> = input
        .split('\n') // we have the rows here
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();
    rows_and_columns
    let total_rows = rows_and_columns.len();
    let mut columns_sum_prods = vec![(0, 1); rows_and_columns[0].len()];

    rows_and_columns.into_iter().for_each(|row| {
        row.into_iter()
            .enumerate()
            .for_each(|(column, maybe_number)| {
                // print!("Trying to access column #{column}");
                let mut interim_results = columns_sum_prods[column];
                match maybe_number.parse::<u64>() {
                    Ok(number) => {
                        interim_results.0 += number;
                        interim_results.1 *= number;
                        columns_sum_prods[column] = interim_results;
                    }
                    Err(_) => {
                        // Not a number, so we are actually adding the results.
                        match maybe_number {
                            "*" => result += interim_results.1,
                            _ => result += interim_results.0,
                        }
                    }
                }
            })
    });
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
