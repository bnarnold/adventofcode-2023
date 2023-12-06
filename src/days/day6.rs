use std::iter::zip;

use crate::util::prelude::*;

fn do_level2(input: &str) -> Option<u64> {
    let (time_line, dist_line) = input.split_once('\n')?;
    let t: String = time_line
        .strip_prefix("Time:")?
        .split_whitespace()
        .collect();
    let d: String = dist_line
        .strip_prefix("Distance:")?
        .split_whitespace()
        .collect();

    number_of_possible_times((
        t.parse().expect("time parse"),
        d.parse().expect("distance parse"),
    ))
}

fn do_level1(input: &str) -> Option<u64> {
    let (time_line, dist_line) = input.split_once('\n')?;
    let ts = time_line.strip_prefix("Time:")?.split_whitespace();
    let ds = dist_line.strip_prefix("Distance:")?.split_whitespace();

    Some(
        zip(ts, ds)
            .map(|(t, d)| {
                (
                    t.parse().expect("time parse"),
                    d.parse().expect("distance parse"),
                )
            })
            .filter_map(number_of_possible_times)
            .product(),
    )
}

fn number_of_possible_times((t, d): (u64, u64)) -> Option<u64> {
    // want to find integers x such that x(t-x) > d
    // The distance between the roots is √(t² - 4d)
    // The rest is bookkeeping for finding the integer points: 2 less if the roots are integers,
    // an even amount iff t is odd
    let disc = (t * t).checked_sub(4 * d)? as f64;
    let even_offset = if t % 2 == 0 { 0.5 } else { 0.0 };
    if disc == 0.0 {
        return Some((2.0 * even_offset) as u64);
    }
    let num_of_things = (disc.sqrt() / 2.0 + 0.5 - even_offset).ceil() - 1.0 + even_offset;
    Some((2.0 * num_of_things).floor() as u64)
}

pub fn level1(input: &str) -> u64 {
    do_level1(input).expect("parse error")
}

pub fn level2(input: &str) -> u64 {
    do_level2(input).expect("parse error")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day6.txt");
        assert_eq!(level1(test_input), 288)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day6.txt");
        assert_eq!(level2(test_input), 71503)
    }
}
