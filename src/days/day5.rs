use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    iter::once,
};

use nom::{
    character::{
        complete::{char, newline, space1, u64},
        streaming::alpha1,
    },
    combinator::{eof, map_opt, success},
    multi::count,
    sequence::{separated_pair, tuple},
    Parser,
};
use nom_supreme::{
    final_parser::final_parser, multi::collect_separated_terminated, tag::complete::tag, ParserExt,
};

use crate::util::prelude::*;

#[derive(Debug)]
struct Segment {
    target: usize,
    len: usize,
}

#[derive(Debug, Default)]
struct RangeMap(BTreeMap<usize, Segment>);

impl RangeMap {
    fn get(&self, x: usize) -> Option<usize> {
        let (source, Segment { target, len }) = self.0.range(..=x).next_back()?;
        (x < source + len).then_some(x + target - source)
    }
}

impl Extend<(usize, usize, usize)> for RangeMap {
    fn extend<T: IntoIterator<Item = (usize, usize, usize)>>(&mut self, iter: T) {
        self.0.extend(
            iter.into_iter()
                .map(|(target, source, len)| (source, Segment { target, len })),
        )
    }
}

fn parse_usize(input: &str) -> ParseResult<usize> {
    map_opt(u64, |x| -> Option<usize> { x.try_into().ok() })
        .context("usize")
        .parse(input)
}

fn parse_seeds(input: &str) -> ParseResult<Vec<usize>> {
    collect_separated_terminated(parse_usize, space1, newline)
        .preceded_by(tag("seeds: "))
        .context("seeds")
        .parse(input)
}

fn parse_block_description(input: &str) -> ParseResult<(&str, &str)> {
    separated_pair(alpha1, tag("-to-"), alpha1)
        .terminated(tag(" map:"))
        .terminated(newline)
        .context("mapping description")
        .parse(input)
}

fn parse_block(input: &str) -> ParseResult<RangeMap> {
    collect_separated_terminated(
        tuple((parse_usize, space1, parse_usize, space1, parse_usize)).map(|t| (t.0, t.2, t.4)),
        newline,
        tag("\n").terminated(tag("\n").or(eof)),
    )
    .context("mapping block")
    .parse(input)
}

type MapConfig<'a> = (Vec<usize>, HashMap<&'a str, (&'a str, RangeMap)>);

fn parse_input(input: &str) -> ParseFinalResult<MapConfig> {
    final_parser(tuple((
        parse_seeds.terminated(newline),
        collect_separated_terminated(
            parse_block_description
                .and(parse_block)
                .map(|((src, tgt), map)| (src, (tgt, map))),
            success(()),
            eof,
        ),
    )))(input)
}

pub fn level1(input: &str) -> usize {
    let (mut offsets, maps) = parse_input(input).expect("parse error");
    let mut map_type = "seed";
    while let Some((new_map_type, map)) = maps.get(map_type) {
        offsets = offsets
            .into_iter()
            .map(|x| map.get(x).unwrap_or(x))
            .collect();
        map_type = new_map_type
    }
    assert_eq!(map_type, "location");
    offsets.into_iter().min().expect("empty offsets")
}

#[derive(Default, Debug)]
struct Intervals(Vec<(usize, usize)>);

impl FromIterator<(usize, usize)> for Intervals {
    fn from_iter<T: IntoIterator<Item = (usize, usize)>>(iter: T) -> Self {
        let mut result = Self::default();
        result.extend(iter);
        result
    }
}

impl Extend<(usize, usize)> for Intervals {
    fn extend<T: IntoIterator<Item = (usize, usize)>>(&mut self, iter: T) {
        for (a, b) in iter {
            self.push(a, b)
        }
    }
}

impl Intervals {
    fn push(&mut self, a: usize, b: usize) {
        if a == b {
            return;
        }
        debug_assert!(a < b);
        let last_before = self
            .0
            .binary_search_by(|(_, end)| {
                if *end < a {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            })
            .unwrap_err(); // Never return Equal, so must get Err
        let first_after = self
            .0
            .binary_search_by(|(start, _)| {
                if *start >= b {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            })
            .unwrap_err();
        let start = if let Some((start, _)) = self.0.get(last_before) {
            a.min(*start)
        } else {
            a
        };
        let end = if let Some((_, end)) = first_after
            .checked_sub(1)
            .and_then(|last_in_range| self.0.get(last_in_range))
        {
            b.max(*end)
        } else {
            b
        };
        self.0.splice(last_before..first_after, once((start, end)));
    }
}

pub fn level2(input: &str) -> usize {
    let (seeds, maps) = parse_input(input).expect("parse error");
    let mut ranges: Intervals = seeds
        .chunks_exact(2)
        .map(|chunk| match chunk {
            [a, length] => (*a, a + length),
            _ => unreachable!(),
        })
        .collect();
    let mut map_type = "seed";
    while let Some((new_map_type, map)) = maps.get(map_type) {
        let mut new_ranges = Intervals::default();
        for (a, b) in ranges.0 {
            let mut last_end = a;
            if let Some((start, Segment { target, len })) = map.0.range(..a).next_back() {
                if start + len > a {
                    // The range starts in an interval
                    last_end = start + len;
                    new_ranges.push(target + a - start, target + (b - start).min(*len));
                }
            }
            for (start, Segment { target, len }) in map.0.range(a..b) {
                new_ranges.push(last_end, *start);
                new_ranges.push(*target, target + (b - start).min(*len));
                last_end = start + len
            }
            if last_end < b {
                new_ranges.push(last_end, b)
            }
        }

        ranges = new_ranges;
        map_type = *new_map_type;
    }
    debug_assert_eq!(map_type, "location");

    ranges.0[0].0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day5.txt");
        assert_eq!(level1(test_input), 35)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day5.txt");
        assert_eq!(level2(test_input), 46)
    }
}
