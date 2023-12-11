use std::collections::{BTreeSet, HashSet};

use nom_supreme::multi::parse_separated_terminated;

use crate::util::prelude::*;

#[derive(Debug)]
struct Universe {
    galaxies: Vec<(usize, usize)>,
    empty_rows: BTreeSet<usize>,
    empty_cols: BTreeSet<usize>,
    expansion_factor: usize,
}

impl Universe {
    fn distance_sum(&self) -> usize {
        let mut result = 0;
        for (i, &(x1, y1)) in self.galaxies.iter().enumerate() {
            for &(x2, y2) in &self.galaxies[(i + 1)..] {
                result += x1.abs_diff(x2)
                    + y1.abs_diff(y2)
                    + (self.expansion_factor - 1)
                        * (self.empty_cols.range(x1.min(x2)..x1.max(x2)).count()
                            + self.empty_rows.range(y1.min(y2)..y1.max(y2)).count())
            }
        }
        result
    }
}

fn parse_input(input: &str) -> Option<Universe> {
    let mut galaxies = Vec::new();
    let mut empty_rows = BTreeSet::new();
    let mut empty_cols = BTreeSet::new();
    let mut lines = input.lines().enumerate();

    for (i, c) in lines.next()?.1.chars().enumerate() {
        match c {
            '.' => {
                empty_cols.insert(i);
            }
            '#' => {
                galaxies.push((i, 0));
            }
            _ => return None,
        }
    }
    if galaxies.is_empty() {
        empty_rows.insert(0);
    }

    for (j, line) in lines {
        let mut row_is_empty = true;
        for (i, c) in line.chars().enumerate() {
            match c {
                '#' => {
                    galaxies.push((i, j));
                    empty_cols.remove(&i);
                    row_is_empty = false;
                }
                '.' => {}
                _ => return None,
            }
        }
        if row_is_empty {
            empty_rows.insert(j);
        }
    }
    Some(Universe {
        galaxies,
        empty_rows,
        empty_cols,
        expansion_factor: 1,
    })
}

pub fn level1(input: &str) -> usize {
    let mut universe = parse_input(input).expect("parse error");
    universe.expansion_factor = 2;
    universe.distance_sum()
}

pub fn level2(input: &str) -> usize {
    let mut universe = parse_input(input).expect("parse error");
    universe.expansion_factor = 1_000_000;
    universe.distance_sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day11.txt");
        assert_eq!(level1(test_input), 374)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day11.txt");
        let mut universe = parse_input(test_input).expect("parse error");
        universe.expansion_factor = 100;
        assert_eq!(universe.distance_sum(), 8410);
    }
}
