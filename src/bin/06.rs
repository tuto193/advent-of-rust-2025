use itertools::Itertools;
use std::{str::Chars, usize};

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

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn reverse_columns<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    v.into_iter().rev().collect()
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result = 0;
    let rows_and_columns: Vec<Vec<String>> = input
        .split('\n') // we have the rows here
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();
    let trans_rows_and_columns = transpose(rows_and_columns);
    let rotated_rows_and_columns = reverse_columns(trans_rows_and_columns);
    // let total_rows = rotated_rows_and_columns.len();
    let row_length = rotated_rows_and_columns[0].len();
    // let mut columns_sum_prods = vec![(0, 1); rows_and_columns[0].len()];
    // The now a set of rows
    let mut current_set_of_numbers: Vec<u64> = vec![];
    // println!("Starting");
    rotated_rows_and_columns
        .into_iter()
        .enumerate()
        .for_each(|(i, row)| {
            // println!("-> Row {i}");
            match row[0..=row_length - 2]
                .join("")
                .split_ascii_whitespace()
                .join("")
                .parse::<u64>()
            {
                Err(_) => {
                    // No number, so also no symbol... simply skip
                }
                Ok(number) => {
                    // println!("-> Push {number}");
                    current_set_of_numbers.push(number);
                    // If there is a number, then there could be a symbol
                    match row[row_length - 1].as_str() {
                        "*" => {
                            result += current_set_of_numbers.iter().product::<u64>();
                            // println!("--> Reset numbers after product");
                            current_set_of_numbers = vec![];
                        }
                        "+" => {
                            result += current_set_of_numbers.iter().sum::<u64>();
                            // println!("--> Reset numbers after sum");
                            current_set_of_numbers = vec![];
                        }
                        _ => {
                            // Do nothing
                        }
                    }
                }
            }
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
        assert_eq!(result, Some(3263827));
    }
}
