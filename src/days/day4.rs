use std::{
    collections::{HashSet, VecDeque},
    num::ParseIntError,
};

use crate::util::prelude::*;
use anyhow::Result;
use itertools::Either;

fn parse_line(line: &str) -> Result<usize> {
    let line = line.split_once(": ").context("no prefix")?.1;
    let (first, last) = line.split_once(" | ").ok_or_else(|| anyhow!("no pipe"))?;
    let first_numbers = first
        .split_whitespace()
        .map(|s| s.parse().context("number parse"))
        .collect::<Result<HashSet<u8>>>()?;
    let last_numbers = last
        .split_whitespace()
        .map(|s| s.parse().context("number parse"))
        .collect::<Result<HashSet<u8>>>()?;
    Ok(first_numbers.intersection(&last_numbers).count())
}

pub fn level1(input: &str) -> usize {
    input
        .lines()
        .map(|l| parse_line(l).expect("line parse"))
        .map(|count| if count == 0 { 0 } else { 1 << (count - 1) })
        .sum()
}

pub fn level2(input: &str) -> usize {
    let mut total: usize = 0;
    let mut previous_card_info = Vec::new();
    for line in input.lines() {
        let count = parse_line(line).expect("line parse");
        let mut copies = 1;
        previous_card_info = previous_card_info
            .into_iter()
            .filter_map(|(remaining_winners, card_copies)| {
                copies += card_copies;
                if remaining_winners > 1 {
                    Some((remaining_winners - 1, card_copies))
                } else {
                    None
                }
            })
            .collect();
        total += copies;
        if count > 0 {
            previous_card_info.push((count, copies))
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day4.txt");
        assert_eq!(level1(test_input), 13)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day4.txt");
        assert_eq!(level2(test_input), 30)
    }
}
