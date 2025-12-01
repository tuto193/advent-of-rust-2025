advent_of_code::solution!(1);

fn parse_input_to_vec(input: &str, clip: bool) -> Vec<i32> {
    input
        .lines()
        .map(|letter_num| {
            let (letter, num) = letter_num.split_at(1);
            let num = num.parse::<i32>().unwrap() % 100;
            let num = if clip { num % 100 } else { num };
            if letter == "L" {
                return num * -1;
            }
            num
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut current_position = 50;
    let mut result = 0;
    // let mut last_position;
    let dial_moves = parse_input_to_vec(input, true);
    for number in dial_moves.into_iter() {
        // last_position = current_position;
        current_position += number;
        // println!("{last_position} + {number} = {current_position}");
        match current_position {
            on_top if on_top % 100 == 0 => {
                result += 1;
                current_position = 0;
            }
            min if min < 0 => current_position = 100 + min,
            _ => current_position = current_position % 100,
        }
        // println!("\tCurrent position adjusted to => {current_position}");
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut current_position = 50;
    let mut result = 0;
    let mut last_position;
    let mut last_was_positive = true;
    let dial_moves = parse_input_to_vec(input, false);
    for number in dial_moves.into_iter() {
        last_position = current_position;
        let full_dial_turns = number / 100;
        let number = number % 100;
        current_position += number;
        print!(" {last_position} + {number} = {current_position}");
        let to_add = full_dial_turns.abs();
        result += to_add;
        let remainder = current_position % 100;
        if remainder.is_positive() {
            result += current_position / 100;
            current_position = remainder;
            last_was_positive = true;
        } else if remainder.is_negative() {
            if last_was_positive {
                result += 1;
            }
            last_was_positive = false;
            current_position = 100 + remainder;
        } else {
            result += 1;
            current_position = 0;
            last_was_positive = false;
        }

        println!(" => {current_position}");
        // if new_result != result {
        println!("|··>Result change after {full_dial_turns} full dial turns-> {result}");
        // result = new_result;
        // }
    }
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
