advent_of_code::solution!(2);

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        // .strip_suffix("\n")
        // .unwrap()
        .split(',')
        .into_iter()
        .map(|range| {
            if let Some((low, high)) = range.split_once('-') {
                // println!("Trying to parse {range}: {low} - {high}");
                return (low.parse::<u64>().unwrap(), high.parse::<u64>().unwrap());
            }
            panic!("Could not split range in string");
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let mut invalid_ids = vec![];
    for (low_id_range, high_id_range) in ranges.into_iter() {
        for id in low_id_range..=high_id_range {
            let id_as_string = format!("{id}");
            // If it's not splittable into two, it's always valid
            if id_as_string.len() % 2 != 0 {
                continue;
            }
            let (first_half, second_half) = id_as_string.split_at(id_as_string.len() / 2);
            if first_half == second_half {
                // invalid ID
                invalid_ids.push(id);
            }
        }
    }
    Some(invalid_ids.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let mut invalid_ids = vec![];
    for (low_id_range, high_id_range) in ranges.into_iter() {
        'id_loop: for id in low_id_range..=high_id_range {
            let id_as_string = id.to_string();
            let id_string_length = id_as_string.len();
            // Split as much as possible
            'validation_loop: for chunk_size in 1..=id_string_length / 2 {
                let id_as_bytes = id_as_string.as_bytes();
                let mut chunks = id_as_bytes.chunks(chunk_size);
                let first_chunk = chunks.next().unwrap();
                for next_chunk in chunks {
                    if next_chunk != first_chunk {
                        continue 'validation_loop; // it's maybe a valid ID
                    }
                }
                // Loop ended, so it is indeed a valid ID
                invalid_ids.push(id);
                // No need to check this ID further.
                continue 'id_loop;
            }
        }
    }
    Some(invalid_ids.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
