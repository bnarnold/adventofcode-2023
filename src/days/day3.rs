use std::ops::Range;

use itertools::Itertools;
use regex::Regex;

pub fn level1(input: &str) -> u32 {
    let grid = input.lines().map(|s| s.chars().collect_vec()).collect_vec();
    let regex = Regex::new(r"\d+").unwrap();
    let width = grid[0].len();
    regex
        .find_iter(input)
        .map(|number_match| {
            let Range { start, end } = number_match.into();
            let row = start / (width + 1);
            let start = start - row * (width + 1);
            let end = end - row * (width + 1);
            let number: u32 = number_match.as_str().parse().expect("parse number");
            let is_symbol = |c: &char| *c != '.' && c.to_digit(10).is_none();
            let mut has_symbol = false;
            if row > 0 {
                has_symbol |= (start.saturating_sub(1)..=end)
                    .filter_map(|x| grid[row - 1].get(x))
                    .any(is_symbol);
            }
            if row + 1 < grid.len() {
                has_symbol |= (start.saturating_sub(1)..=end)
                    .filter_map(|x| grid[row + 1].get(x))
                    .any(is_symbol);
            }
            if start > 0 {
                has_symbol |= is_symbol(&grid[row][start - 1])
            }
            if end < width {
                has_symbol |= is_symbol(&grid[row][end])
            }

            if has_symbol {
                number
            } else {
                0
            }
        })
        .sum()
}

pub fn level2(input: &str) -> u32 {
    let width = input.lines().next().unwrap().len() + 1;
    let gear_regex = Regex::new(r"\*").unwrap();
    let number_regex = Regex::new(r"\d+").unwrap();

    let mut result = 0;
    for gear_match in gear_regex.find_iter(input) {
        let row = gear_match.start() / width;
        let column = gear_match.start() - row * width;
        let mut surrounding_numbers = Vec::new();

        if row > 0 {
            surrounding_numbers.extend(
                number_regex
                    .find_iter(&input[((row - 1) * width)..(row * width)])
                    .filter(|number_match| {
                        number_match.start() < column + 2 && number_match.end() >= column
                    })
                    .map(|number_match| number_match.as_str().parse::<u32>().expect("parse error")),
            )
        }
        if (row + 1) * width < input.len() {
            surrounding_numbers.extend(
                number_regex
                    .find_iter(&input[((row + 1) * width)..((row + 2) * width).min(input.len())])
                    .filter(|number_match| {
                        number_match.start() < column + 2 && number_match.end() >= column
                    })
                    .map(|number_match| number_match.as_str().parse::<u32>().expect("parse error")),
            )
        }
        if column > 0 {
            let slice_before =
                match input[..gear_match.start()].rsplit_once(|c: char| !c.is_ascii_digit()) {
                    Some((_, postfix)) => postfix,
                    None => &input[(row * width)..gear_match.start()],
                };
            if !slice_before.is_empty() {
                surrounding_numbers.push(slice_before.parse().expect("parse error"))
            }
        }
        if column + 2 < width {
            let slice_after =
                match input[gear_match.end()..].split_once(|c: char| !c.is_ascii_digit()) {
                    Some((prefix, _)) => prefix,
                    None => &input[gear_match.end()..((row + 1) * width)],
                };
            if !slice_after.is_empty() {
                surrounding_numbers.push(slice_after.parse().expect("parse error"))
            }
        }

        if surrounding_numbers.len() == 2 {
            result += surrounding_numbers.into_iter().product::<u32>();
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day3.txt");
        assert_eq!(level1(test_input), 4361)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day3.txt");
        assert_eq!(level2(test_input), 467835)
    }
}
