use std::{
    collections::{hash_map::Entry, BTreeSet, BinaryHeap, HashMap, VecDeque},
    iter::once,
};

use nom::{
    branch::alt,
    character::complete::{alphanumeric1, newline},
    combinator::{eof, success},
    sequence::separated_pair,
    Parser,
};
use nom_supreme::{
    final_parser::final_parser, multi::collect_separated_terminated, tag::complete::tag, ParserExt,
};

use crate::util::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node<'a> {
    /// where you end up when you do all directions once
    next: &'a str,
    /// indices of targets which you saw along the way
    targets: Vec<usize>,
}

#[derive(Debug)]
struct Graph<'a> {
    step_length: usize,
    nodes: HashMap<&'a str, Node<'a>>,
}

#[derive(Debug)]
struct Cycle {
    prefix_length: usize,
    prefix_hits: Vec<usize>,
    cycle_length: usize,
    cycle_hits: Vec<usize>,
}

impl Cycle {
    fn all_hits() -> Self {
        Self {
            prefix_length: 0,
            prefix_hits: Vec::new(),
            cycle_length: 1,
            cycle_hits: vec![0],
        }
    }
    fn rotate(&mut self, steps: usize) {
        self.prefix_length += steps;
        let full_rotations = steps / self.cycle_length;
        self.prefix_hits.extend((0..full_rotations).flat_map(|i| {
            self.cycle_hits.iter().map({
                let len = self.cycle_length;
                move |x| x + i * len
            })
        }));

        let partial_rotation_steps = steps % self.cycle_length;
        let split_point = self
            .cycle_hits
            .partition_point(|x| *x >= partial_rotation_steps);
        self.prefix_hits.extend(
            self.cycle_hits[..split_point]
                .iter()
                .map(|x| x + full_rotations * self.cycle_length),
        );

        self.cycle_hits.rotate_left(split_point);
        let start_of_rotated = self.cycle_hits.len() - split_point;
        for x in &mut self.cycle_hits[start_of_rotated..] {
            *x += self.cycle_length;
        }
    }

    fn intersect(mut self, mut other: Self) -> Self {
        // Ensure the prefixes have the same length
        if self.prefix_length > other.prefix_length {
            other.rotate(self.prefix_length - other.prefix_length)
        }
        if other.prefix_length > self.prefix_length {
            self.rotate(other.prefix_length - self.prefix_length)
        }
        // Prefixes of merged cycle are those that appear in both prefixes
        let self_prefix_hits: BTreeSet<_> = self.prefix_hits.into_iter().collect();
        let other_prefix_hits: BTreeSet<_> = other.prefix_hits.into_iter().collect();
        let prefix_hits = self_prefix_hits
            .intersection(&other_prefix_hits)
            .copied()
            .collect();

        fn euclidean_algorithm(a: usize, b: usize) -> (usize, isize, isize) {
            // Invariant: c = u * a + v * y, d = x * a + y * b
            #[derive(Debug)]
            struct IterationState {
                c: usize,
                d: usize,
                u: isize,
                v: isize,
                x: isize,
                y: isize,
            }
            let mut state = IterationState {
                c: a,
                d: b,
                u: 1,
                v: 0,
                x: 0,
                y: 1,
            };
            while state.c != 0 {
                let div = (state.d / state.c) as isize;
                let rem = state.d % state.c;
                state = IterationState {
                    c: rem,
                    d: state.c,
                    u: state.x - div * state.u,
                    v: state.y - div * state.v,
                    x: state.u,
                    y: state.v,
                }
            }
            (state.d, state.x, state.y)
        }
        let (d, x, y) = euclidean_algorithm(self.cycle_length, other.cycle_length);
        debug_assert_eq!(
            d as isize,
            (self.cycle_length as isize) * x + (other.cycle_length as isize) * y
        );
        debug_assert_eq!(self.cycle_length % d, 0);
        debug_assert_eq!(other.cycle_length % d, 0);
        // least common multiple
        let cycle_length = self.cycle_length * (other.cycle_length / d);
        // Aggregate elements of the two cyclic groups by the element of ℤ/d over which they lie
        let mut fiber_product: HashMap<_, (Vec<_>, Vec<_>)> = HashMap::new();
        for self_hit in self.cycle_hits {
            fiber_product
                .entry(self_hit % d)
                .or_default()
                .0
                .push(self_hit / d)
        }
        for other_hit in other.cycle_hits {
            if let Some((_, others)) = fiber_product.get_mut(&(other_hit % d)) {
                others.push(other_hit / d);
            }
        }

        let self_cycle_len = self.cycle_length / d;
        let other_cycle_len = other.cycle_length / d;
        let mut cycle_hits =
            Vec::with_capacity(fiber_product.values().map(|(v, w)| v.len() * w.len()).sum());
        for (rem, (self_hits, other_hits)) in fiber_product {
            for other_hit in other_hits {
                for &self_hit in &self_hits {
                    // mod other_cycle_len, x is the multiplicative inverse of self_cycle_len…
                    let offset = (other_hit as isize - self_hit as isize) * x;
                    let offset = offset % other_cycle_len as isize;
                    let offset = if offset >= 0 {
                        offset as usize
                    } else {
                        (other_cycle_len as isize + offset) as usize
                    };
                    // …so this is other_hit mod other_cycle_len…
                    let lift = self_hit + self_cycle_len * offset;
                    // and this is rem + d * {self,other}_hit mod {self, other}.cycle_length
                    let lift = rem + d * lift;
                    cycle_hits.push(lift);
                }
            }
        }
        cycle_hits.sort();
        Cycle {
            prefix_length: self.prefix_length,
            prefix_hits,
            cycle_length,
            cycle_hits,
        }
    }

    fn first(&self) -> Option<usize> {
        if let Some(first) = self.prefix_hits.first() {
            Some(*first)
        } else {
            self.cycle_hits.first().map(|x| x + self.prefix_length)
        }
    }
}

impl<'a> Graph<'a> {
    fn new(
        directions: &[Direction],
        mappings: &HashMap<&'a str, (&'a str, &'a str)>,
        mut is_target: impl FnMut(&str) -> bool,
    ) -> Self {
        let step_length = directions.len();
        let mut nodes: HashMap<_, _> = mappings
            .iter()
            .map(|(&src, _)| {
                (
                    src,
                    Node {
                        next: src,
                        targets: Vec::new(),
                    },
                )
            })
            .collect();
        for (i, dir) in directions.iter().enumerate() {
            let new_nodes = nodes
                .into_iter()
                .map(|(label, Node { next, mut targets })| {
                    (
                        label,
                        Node {
                            next: {
                                let children = mappings[next];
                                match dir {
                                    Direction::Left => children.0,
                                    Direction::Right => children.1,
                                }
                            },
                            targets: {
                                if is_target(next) {
                                    targets.push(i);
                                }
                                targets
                            },
                        },
                    )
                })
                .collect();
            nodes = new_nodes
        }
        Graph { step_length, nodes }
    }
    fn cycle(&self, start: &'a str) -> Cycle {
        let mut label = start;
        let mut hits = Vec::new();
        let mut positions = HashMap::new();
        for i in 0.. {
            match positions.entry(label) {
                Entry::Occupied(e) => {
                    let &(cycle_entry, prefix_count) = e.get();
                    let cycle_pos = cycle_entry * self.step_length;
                    let (hits_in_prefix, hits_in_cycle) = hits.split_at(prefix_count);
                    let prefix_hits = hits_in_prefix.to_vec();
                    let cycle_hits = hits_in_cycle.iter().map(|x| x - cycle_pos).collect_vec();
                    return Cycle {
                        prefix_length: cycle_pos,
                        prefix_hits,
                        cycle_length: (i - cycle_entry) * self.step_length,
                        cycle_hits,
                    };
                }
                Entry::Vacant(e) => {
                    e.insert((i, hits.len()));
                    hits.extend(
                        self.nodes[label]
                            .targets
                            .iter()
                            .map(|x| x + self.step_length * i),
                    );
                    label = self.nodes[label].next
                }
            };
        }
        unreachable!()
    }
}

fn parse_directions(input: &str) -> ParseResult<Vec<Direction>> {
    collect_separated_terminated(
        alt((
            tag("L").value(Direction::Left),
            tag("R").value(Direction::Right),
        )),
        success(()),
        tag("\n"),
    )
    .parse(input)
}

fn parse_node(input: &str) -> ParseResult<(&str, (&str, &str))> {
    separated_pair(
        alphanumeric1.context("source"),
        tag(" = ("),
        separated_pair(
            alphanumeric1.context("left"),
            tag(", "),
            alphanumeric1.context("right"),
        )
        .terminated(tag(")\n")),
    )(input)
}

type Input<'a> = (Vec<Direction>, HashMap<&'a str, (&'a str, &'a str)>);

fn parse_input(input: &str) -> ParseFinalResult<Input> {
    final_parser(
        parse_directions
            .terminated(newline)
            .and(collect_separated_terminated(parse_node, success(()), eof)),
    )(input)
}

pub fn level1(input: &str) -> usize {
    let (directions, mappings) = parse_input(input).expect("parse error");
    let graph = Graph::new(&directions, &mappings, |s| s == "ZZZ");
    let cycle = graph.cycle("AAA");
    cycle.first().expect("target is unreachable")
}

pub fn level2(input: &str) -> usize {
    let (directions, mappings) = parse_input(input).expect("parse error");
    let graph = Graph::new(&directions, &mappings, |s| s.ends_with('Z'));
    let cycle = mappings
        .keys()
        .filter(|s| s.ends_with('A'))
        .map(|s| graph.cycle(s))
        .fold(Cycle::all_hits(), |c1, c2| c1.intersect(c2));
    cycle.first().expect("target state is unreachable")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day8.txt");
        assert_eq!(level1(test_input), 2)
    }

    #[test]
    fn level1_given_example_long() {
        let test_input = include_str!("./test_input/day8_long.txt");
        assert_eq!(level1(test_input), 6)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day8_ghost.txt");
        assert_eq!(level2(test_input), 6)
    }
}
