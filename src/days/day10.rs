use std::collections::{hash_map::Entry, HashMap, HashSet};

use nom::{
    branch::alt,
    character::complete::char,
    combinator::{eof, success},
    multi::many1,
    Parser,
};
use nom_supreme::{
    final_parser::final_parser,
    multi::{collect_separated_terminated, parse_separated_terminated},
    tag::complete::tag,
    ParserExt,
};

use crate::util::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    W,
    S,
    E,
}

impl Direction {
    fn opp(&self) -> Self {
        match self {
            Direction::N => Direction::S,
            Direction::W => Direction::E,
            Direction::S => Direction::N,
            Direction::E => Direction::W,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Piece {
    Nothing,
    Start,
    Pipe([Direction; 2]),
}

impl Piece {
    fn directions(self) -> Option<[Direction; 2]> {
        debug_assert!(!matches!(self, Piece::Start));
        match self {
            Self::Pipe(directions) => Some(directions),
            _ => None,
        }
    }
}

fn parse_piece(input: &str) -> ParseResult<Piece> {
    use Direction::*;
    use Piece::*;

    alt((
        char('.').value(Nothing),
        char('S').value(Start),
        char('-').value(Pipe([E, W])),
        char('|').value(Pipe([S, N])),
        char('J').value(Pipe([W, N])),
        char('L').value(Pipe([N, E])),
        char('F').value(Pipe([E, S])),
        char('7').value(Pipe([S, W])),
    ))
    .context("pipe piece")
    .parse(input)
}

#[derive(Debug)]
struct PipeGrid {
    pieces: Vec<Piece>,
    width: usize,
    height: usize,
}

impl PipeGrid {
    fn start_pos(&self) -> Option<(usize, usize)> {
        let pos = self
            .pieces
            .iter()
            .position(|piece| matches!(piece, Piece::Start))?;
        Some((pos % self.width, pos / self.width))
    }

    fn get(&self, pos: (usize, usize)) -> Option<Piece> {
        if (0..self.width).contains(&pos.0) && (0..self.height).contains(&pos.1) {
            self.pieces.get(pos.0 + self.width * pos.1).copied()
        } else {
            None
        }
    }

    fn next(
        &self,
        pos: (usize, usize),
        direction: Direction,
    ) -> Option<((usize, usize), Option<Direction>)> {
        let next_pos = match direction {
            Direction::N => (pos.0, pos.1.checked_sub(1)?),
            Direction::W => (pos.0.checked_sub(1)?, pos.1),
            Direction::S => (pos.0, pos.1 + 1),
            Direction::E => (pos.0 + 1, pos.1),
        };
        let next_direction = match self.get(next_pos)? {
            Piece::Nothing => return None,
            Piece::Start => None,
            Piece::Pipe(next_directions) => {
                let next_direction_pos =
                    1 - next_directions.iter().position(|d| d.opp() == direction)?;
                Some(next_directions[next_direction_pos])
            }
        };
        Some((next_pos, next_direction))
    }

    fn loop_length_and_area(&self) -> Option<(usize, usize)> {
        let start_pos = self.start_pos()?;

        'dir: for start_dir in [Direction::N, Direction::W, Direction::S, Direction::E] {
            let mut pos = start_pos;
            let mut direction = start_dir;
            let mut area = 0_isize;

            for length in 1.. {
                let Some((next_pos, next_direction)) = self.next(pos, direction) else {
                    continue 'dir;
                };
                let next_direction = next_direction.unwrap_or(start_dir);
                {
                    use Direction::*;
                    match (direction, next_direction) {
                        // Shift the loop to the left in the direction of travel so that it lies on the grid lines.
                        // If the direction is clockwise, this shift encloses exactly the squares on the inside of the loop.
                        // If the direction is counterclockwise, it also enclodes the squares of the loop.
                        // The enclosed area can, up to sign, be calculated by integrating y dx along the shifted loop.
                        (N, N) | (N, E) | (W, N) => {
                            area += next_pos.0 as isize;
                        }
                        (S, S) | (S, W) | (E, S) => {
                            area -= next_pos.0 as isize + 1;
                        }
                        _ => {}
                    }
                }
                if next_pos == start_pos {
                    if area >= 0 {
                        // counterclockwise
                        return Some((area as usize, length));
                    } else {
                        // clockwise: The shifted path also includes all squares in the path,
                        // so fix the orientation and subtract them
                        return Some(((-area as usize) - length, length));
                    }
                }
                pos = next_pos;
                direction = next_direction;
            }
        }
        None
    }
}

fn parse_grid(input: &str) -> ParseResult<PipeGrid> {
    let mut pieces = Vec::new();
    let mut height = 0;
    let parse_line = parse_separated_terminated(
        parse_piece,
        success(()),
        tag("\n"),
        || height += 1,
        |_, piece| {
            pieces.push(piece);
        },
    );
    let (rest, _): (&str, ()) =
        collect_separated_terminated(parse_line, success(()), eof).parse(input)?;
    let width = pieces.len() / height;
    Ok((
        rest,
        PipeGrid {
            pieces,
            width,
            height,
        },
    ))
}

fn parse_input(input: &str) -> ParseFinalResult<PipeGrid> {
    final_parser(parse_grid)(input)
}

pub fn level1(input: &str) -> usize {
    let grid = parse_input(input).expect("parse error");
    let (_, length) = grid.loop_length_and_area().expect("No closed loop");
    length / 2
}

pub fn level2(input: &str) -> usize {
    let grid = parse_input(input).expect("parse error");
    grid.loop_length_and_area().expect("No closed loop").0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day10.txt");
        assert_eq!(level1(test_input), 4)
    }

    #[test]
    fn level1_given_example_complex() {
        let test_input = include_str!("./test_input/day10_complex.txt");
        assert_eq!(level1(test_input), 8)
    }

    #[test]
    fn level2_given_example_simple() {
        let test_input = include_str!("./test_input/day10.txt");
        assert_eq!(level2(test_input), 1)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day10_loop.txt");
        assert_eq!(level2(test_input), 4)
    }

    #[test]
    fn level2_given_example_complex() {
        let test_input = include_str!("./test_input/day10_loop_complex.txt");
        assert_eq!(level2(test_input), 8)
    }
}
