use crate::util::prelude::*;
use nom::{
    branch::alt,
    character::complete::{newline, space1, u32},
    combinator::eof,
    multi::many0,
    sequence::tuple,
    IResult, Parser,
};
use nom_supreme::{
    error::ErrorTree,
    final_parser::{final_parser, Location},
    multi::{collect_separated_terminated, parse_separated_terminated},
    tag::complete::tag,
    ParserExt,
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn max(&self, other: &Self) -> Self {
        CubeSet {
            red: self.red.max(other.red),
            green: self.green.max(other.green),
            blue: self.blue.max(other.blue),
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>,
}

fn parse_cube_set(input: &str) -> IResult<&str, CubeSet, ErrorTree<&str>> {
    let parse_color =
        tuple((u32, space1, alt((tag("red"), tag("green"), tag("blue"))))).context("color count");
    parse_separated_terminated(
        parse_color,
        tag(", "),
        alt((tag("; "), tag("\n"), eof)),
        CubeSet::default,
        |mut set, (count, _, color)| {
            match color {
                "red" => set.red = count,
                "green" => set.green = count,
                "blue" => set.blue = count,
                _ => unreachable!("parse succeeded but got unexpected color"),
            }
            set
        },
    )
    .context("set of cubes")
    .parse(input)
}

fn parse_game(input: &str) -> IResult<&str, Game, ErrorTree<&str>> {
    tuple((
        u32.preceded_by(tag("Game "))
            .terminated(tag(": "))
            .context("Game ID"),
        many0(parse_cube_set).context("Cube sets"),
    ))
    .map(|(id, sets)| Game { id, sets })
    .context("Game")
    .parse(input)
}

fn parse_input(input: &str) -> Result<Vec<Game>, ErrorTree<Location>> {
    final_parser(many0(parse_game))(input)
}

pub fn level1(input: &str) -> u32 {
    let max_set = CubeSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    let games = parse_input(input).expect("parse error");
    games
        .into_iter()
        .filter(|game| game.sets.iter().all(|set| set.max(&max_set) == max_set))
        .map(|game| game.id)
        .sum()
}

pub fn level2(input: &str) -> u32 {
    parse_input(input)
        .expect("Parse error")
        .into_iter()
        .map(|Game { sets, .. }| {
            sets.iter()
                .fold(CubeSet::default(), |acc, set| acc.max(set))
        })
        .map(|CubeSet { red, green, blue }| red * green * blue)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day2.txt");
        assert_eq!(level1(test_input), 8)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day2.txt");
        assert_eq!(level2(test_input), 2286)
    }
}
