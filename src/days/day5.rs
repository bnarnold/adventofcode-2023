use std::collections::{BTreeMap, HashMap};

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

fn parse_input(input: &str) -> ParseFinalResult<(Vec<usize>, HashMap<&str, (&str, RangeMap)>)> {
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
    dbg!(&maps);
    while let Some((new_map_type, map)) = maps.get(map_type) {
        offsets = offsets
            .into_iter()
            .map(|x| map.get(x).unwrap_or(x))
            .collect();
        dbg!(&offsets);
        map_type = new_map_type
    }
    offsets.into_iter().min().expect("empty offsets")
}

pub fn level2(input: &str) -> usize {
    0
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
        assert_eq!(level2(test_input), 0)
    }
}
