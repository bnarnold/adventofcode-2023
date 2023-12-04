use std::{
    collections::{BTreeMap, HashSet, VecDeque},
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
        .filter_map(|count| count.checked_sub(1).map(|x| 1 << x))
        .sum()
}

pub fn level2(input: &str) -> usize {
    let mut total: usize = 0;
    let mut copy_count = 0;
    let mut copy_count_offsets = BTreeMap::new();
    for (i, line) in input.lines().enumerate() {
        // get copy_count cards from previous, and another one because we already have it
        let count = parse_line(line).expect("line parse");
        total += copy_count + 1;
        if count > 0 {
            *copy_count_offsets.entry(i + count).or_default() += copy_count + 1;
            copy_count = 2 * copy_count + 1;
        }
        if let Some(min) = copy_count_offsets.first_entry() {
            if *min.key() == i {
                copy_count -= min.get();
                min.remove();
            }
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
