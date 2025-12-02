advent_of_code::solution!(1);

fn parse_input_to_vec(input: &str, clip: bool) -> Vec<i32> {
    input
        .lines()
        .map(|letter_num| {
            let (letter, num) = letter_num.split_at(1);
            let num = num.parse::<i32>().unwrap();
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
    let mut current_dial_pos: i32 = 50;
    let mut result = 0;
    let dial_moves = parse_input_to_vec(input, false);
    for maybe_big_rotation in dial_moves.into_iter() {
        // > 0
        let last_pos_was_positive = current_dial_pos.is_positive();
        let full_dial_turns = (maybe_big_rotation / 100).abs();
        let mut add_to_result = 0;
        // Number is clipped and can be negative
        let small_rotation = maybe_big_rotation % 100;
        let mut new_dial_position = current_dial_pos + small_rotation;
        print!(
            " {current_dial_pos} + {small_rotation} ({maybe_big_rotation}) = {new_dial_position}"
        );
        add_to_result += (new_dial_position / 100).abs();
        add_to_result += full_dial_turns;
        let mut clipped_dial_position = false;
        let remainder = new_dial_position % 100;
        if remainder.is_positive() {
            if new_dial_position != remainder {
                // needs to be clipped
                new_dial_position = remainder;
                clipped_dial_position = true;
            }
        } else if remainder.is_negative() {
            if last_pos_was_positive {
                add_to_result += 1;
            }
            clipped_dial_position = true;
            new_dial_position = 100 + remainder;
        } else {
            if (current_dial_pos == 0 || new_dial_position == 0) && last_pos_was_positive {
                add_to_result += 1;
            }
            if new_dial_position != 0 {
                new_dial_position = 0;
                clipped_dial_position = true;
            }
        }
        if clipped_dial_position {
            print!(" => {new_dial_position}");
        }
        println!();
        // if new_result != result {
        if add_to_result != 0 {
            result += add_to_result;
            println!("|··>Result········>{result}");
        }
        // result = new_result;
        // }
        current_dial_pos = new_dial_position;
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
