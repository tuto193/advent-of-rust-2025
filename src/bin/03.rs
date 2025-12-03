advent_of_code::solution!(3);

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            let line_length = line.len();
            line.split("")
                .skip(1)
                .take(line_length)
                .map(|num_str| num_str.parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines_of_numbers = parse_input(input);
    let mut highest_numbers = vec![];

    for line in lines_of_numbers.into_iter() {
        let mut last_highest_index_and_digit = line
            .clone()
            .into_iter()
            .enumerate()
            .max_by(|&i_num, &j_num| i_num.1.cmp(&j_num.1))
            .unwrap();
        // println!(
        //     "Latest largest digit (index, digit): {:?}",
        //     last_highest_index_and_digit
        // );
        // We check the reversed iterator to get the highest and most to
        // the front
        let first_highest_index_and_digit = line
            .clone()
            .into_iter()
            .enumerate()
            .rev()
            .max_by(|&i_num, &j_num| i_num.1.cmp(&j_num.1))
            .unwrap();
        // println!(
        //     "Earliest largest digit (index, digit): {:?}",
        //     first_highest_index_and_digit
        // );
        if last_highest_index_and_digit.0 == first_highest_index_and_digit.0 {
            // println!("|•• They are the same",);
            // Need to find the second highest. Start searching to the
            // right of the first highest
            last_highest_index_and_digit = if first_highest_index_and_digit.0 != line.len() - 1 {
                line.into_iter()
                    .enumerate()
                    .skip(first_highest_index_and_digit.0)
                    .filter(|i_num| i_num.1 != last_highest_index_and_digit.1)
                    .max_by(|&i_num, &j_num| i_num.1.cmp(&j_num.1))
                    .unwrap()
            } else {
                // If
                line.into_iter()
                    .enumerate()
                    .filter(|i_num| i_num.1 != last_highest_index_and_digit.1)
                    .max_by(|&i_num, &j_num| i_num.1.cmp(&j_num.1))
                    .unwrap()
            };
            // println!("|••• Second largest {:?}", last_highest_index_and_digit);
        }
        // Need to put them in the right order
        let as_string = if last_highest_index_and_digit.0 < first_highest_index_and_digit.0 {
            format!(
                "{}{}",
                last_highest_index_and_digit.1, first_highest_index_and_digit.1
            )
        } else {
            format!(
                "{}{}",
                first_highest_index_and_digit.1, last_highest_index_and_digit.1
            )
        };
        let as_joltage = as_string.parse::<u64>().unwrap();
        // println!("|···-> Joltage: {as_joltage}");
        highest_numbers.push(as_joltage);
    }
    Some(highest_numbers.into_iter().sum())
}

fn make_spacey_string_from_numbers(i_numbers: Vec<(usize, u8)>, string_length: usize) -> String {
    let mut spacey_string = " ".repeat(string_length);
    for (index, digit) in i_numbers.into_iter() {
        spacey_string.insert(index, digit.to_string().char_indices().next().unwrap().1);
    }
    return spacey_string;
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines_of_numbers = parse_input(input);
    let mut highest_joltages: Vec<u64> = vec![];
    let line_length = lines_of_numbers[0].len();

    for line in lines_of_numbers.into_iter() {
        let mut highest_12_digits: Vec<(usize, u8)> = vec![];
        let line_as_string = line
            .clone()
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("");
        println!("Line···>{line_as_string}");
        // Fill the vector, with the 12 highest digits
        // Populate with the leftmost highest digit
        let i_number = line
            .clone()
            .into_iter()
            .enumerate()
            .rev()
            .max_by(|&i_num, &j_num| i_num.1.cmp(&j_num.1))
            .unwrap();
        highest_12_digits.push(i_number);
        let spacey_string = make_spacey_string_from_numbers(vec![i_number], line_length);
        println!("  Max··>{spacey_string}");
        'new_digit: for d in 1..12 {
            let remaining_to_find = 12 - d;
            'right_of_current_digits: for (index, _) in highest_12_digits.clone().into_iter().rev()
            {
                // Check if we should even check to the right:
                // if there are enough digits to the right, our number will
                // be highest, if we fill it up with the highest numbers from that side
                // otherwise, find one (1) next highest to the left
                if (line_length - index - 1) >= remaining_to_find {
                    let right_of_rightmost_highest_digit = line
                        .clone()
                        .into_iter()
                        .enumerate()
                        .skip(index + 1)
                        // .rev()
                        .filter(|i_num| !highest_12_digits.contains(i_num))
                        .max_by(|&i_num, &j_num| i_num.1.cmp(&j_num.1))
                        .unwrap();
                    let spacey_string = make_spacey_string_from_numbers(
                        vec![right_of_rightmost_highest_digit],
                        line_length,
                    );
                    println!("  Max··>{spacey_string}");
                    highest_12_digits.push(right_of_rightmost_highest_digit);
                    // Sort them by index
                    highest_12_digits.sort_by(|&a, &b| a.0.cmp(&b.0));
                    // Found digit, get another one
                    continue 'new_digit;
                }
                // To the right there isn't even enough space, we'll
                // need to simply find the highest to the left of us
                let left_of_rightmost_highest_digit = line
                    .clone()
                    .into_iter()
                    .enumerate()
                    .skip(highest_12_digits[0].0 + 1)
                    .take(index)
                    .rev()
                    .filter(|i_num| !highest_12_digits.contains(i_num))
                    .max_by(|&i_num, &j_num| i_num.1.cmp(&j_num.1))
                    .unwrap();

                let spacey_string = make_spacey_string_from_numbers(
                    vec![left_of_rightmost_highest_digit],
                    line_length,
                );
                println!("  Max··>{spacey_string}");
                highest_12_digits.push(left_of_rightmost_highest_digit);
                // Sort them by index
                highest_12_digits.sort_by(|&a, &b| a.0.cmp(&b.0));
                continue 'new_digit;
            }
            // If the loop before here ends up landing here, it means we
            // didn't find anything to the right, so we need to check to the left

            // let almost_rightmost_highest_digit = line
            //     .clone()
            //     .into_iter()
            //     .enumerate()
            //     // .skip(*index + 1)
            //     .rev()
            //     .filter(|i_num| !highest_12_digits.contains(i_num))
            //     .max_by(|&i_num, &j_num| i_num.1.cmp(&j_num.1))
            //     .unwrap();

            // print!("{} ", almost_rightmost_highest_digit.1);
            // highest_12_digits.push(almost_rightmost_highest_digit);
            // // Sort them by index
            // highest_12_digits.sort_by(|&a, &b| a.0.cmp(&b.0));
        }

        let sorted_digits: Vec<String> = highest_12_digits
            .clone()
            .into_iter() // Extract only the digits
            .map(|i_num: (usize, u8)| i_num.1.to_string())
            .collect();
        let as_string = sorted_digits.join("");
        // println!("|··> Sorted into ->{as_string}");
        let mut spacey_string = " ".repeat(line_length);
        for (index, digit) in highest_12_digits.into_iter() {
            spacey_string.insert(index, digit.to_string().char_indices().next().unwrap().1);
        }
        println!("|======= {line_as_string}");
        println!("|======= {spacey_string}");
        let as_joltage = as_string.parse::<u64>().unwrap();
        // println!("|···-> Joltage: {as_joltage}");
        highest_joltages.push(as_joltage);
        println!();
    }
    Some(highest_joltages.into_iter().sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
