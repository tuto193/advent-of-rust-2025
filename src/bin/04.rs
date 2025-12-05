advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<usize>> {
    input
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    '.' => 0,
                    _ => 1,
                })
                .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let rolls_map = parse_input(input);
    let total_rows = rolls_map.len();
    let total_columns = rolls_map[0].len();
    let mut total_accessible = 0;
    for y in 0..total_rows {
        for x in 0..total_columns {
            if rolls_map[y][x] == 0 {
                print!("·");
                continue;
            }
            let top = if y > 0 { rolls_map[y - 1][x] } else { 0 };
            let bot = if y < total_rows - 1 {
                rolls_map[y + 1][x]
            } else {
                0
            };
            let left = if x > 0 { rolls_map[y][x - 1] } else { 0 };
            let right = if x < total_columns - 1 {
                rolls_map[y][x + 1]
            } else {
                0
            };
            let top_left = if x > 0 && y > 0 {
                rolls_map[y - 1][x - 1]
            } else {
                0
            };
            let top_right = if x < total_columns - 1 && y > 0 {
                rolls_map[y - 1][x + 1]
            } else {
                0
            };

            let bot_left = if x > 0 && y < total_rows - 1 {
                rolls_map[y + 1][x - 1]
            } else {
                0
            };

            let bot_right = if x < total_columns - 1 && y < total_rows - 1 {
                rolls_map[y + 1][x + 1]
            } else {
                0
            };
            let total_blocking =
                top + bot + left + right + top_left + top_right + bot_left + bot_right;
            if total_blocking < 4 {
                print!("{total_blocking}");
                total_accessible += 1;
                continue;
            }
            print!("@");
        }
        println!();
    }
    Some(total_accessible)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut rolls_map = parse_input(input);
    let total_rows = rolls_map.len();
    let total_columns = rolls_map[0].len();
    let mut total_removed = 0;
    let mut rolls_to_remove: Vec<(usize, usize)> = vec![];
    loop {
        for y in 0..total_rows {
            for x in 0..total_columns {
                if rolls_map[y][x] == 0 {
                    // print!("·");
                    continue;
                }
                let top = if y > 0 { rolls_map[y - 1][x] } else { 0 };
                let bot = if y < total_rows - 1 {
                    rolls_map[y + 1][x]
                } else {
                    0
                };
                let left = if x > 0 { rolls_map[y][x - 1] } else { 0 };
                let right = if x < total_columns - 1 {
                    rolls_map[y][x + 1]
                } else {
                    0
                };
                let top_left = if x > 0 && y > 0 {
                    rolls_map[y - 1][x - 1]
                } else {
                    0
                };
                let top_right = if x < total_columns - 1 && y > 0 {
                    rolls_map[y - 1][x + 1]
                } else {
                    0
                };

                let bot_left = if x > 0 && y < total_rows - 1 {
                    rolls_map[y + 1][x - 1]
                } else {
                    0
                };

                let bot_right = if x < total_columns - 1 && y < total_rows - 1 {
                    rolls_map[y + 1][x + 1]
                } else {
                    0
                };
                let total_blocking =
                    top + bot + left + right + top_left + top_right + bot_left + bot_right;
                if total_blocking < 4 {
                    // print!("{total_blocking}");
                    rolls_to_remove.push((x, y));
                }
                // print!("@");
            }
            // println!();
        }
        if rolls_to_remove.len() == 0 {
            // No more rolls removed
            break;
        }
        // Otherwise count, and remove the rolls
        total_removed += rolls_to_remove.len();
        rolls_to_remove
            .into_iter()
            .for_each(|(x, y)| rolls_map[y][x] = 0);
        rolls_to_remove = vec![];
    }
    Some(total_removed as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
