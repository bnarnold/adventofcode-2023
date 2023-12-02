use crate::util::prelude::*;
use nom::{
    branch::alt,
    character::complete::{newline, space1, u32},
    combinator::{eof, success},
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

#[derive(Debug, Clone, Copy)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl Extend<(u32, Color)> for CubeSet {
    fn extend<T: IntoIterator<Item = (u32, Color)>>(&mut self, iter: T) {
        for (count, color) in iter {
            match color {
                Color::Red => self.red = count,
                Color::Green => self.green = count,
                Color::Blue => self.blue = count,
            }
        }
    }
}

impl Extend<CubeSet> for CubeSet {
    fn extend<T: IntoIterator<Item = CubeSet>>(&mut self, iter: T) {
        for set in iter {
            self.red = self.red.max(set.red);
            self.green = self.green.max(set.green);
            self.blue = self.blue.max(set.blue);
        }
    }
}

#[derive(Debug)]
struct IsGameUnderMax<const RED: u32, const BLUE: u32, const GREEN: u32> {
    is_under_max: bool,
}

impl<const RED: u32, const BLUE: u32, const GREEN: u32> Extend<CubeSet>
    for IsGameUnderMax<RED, BLUE, GREEN>
{
    fn extend<T: IntoIterator<Item = CubeSet>>(&mut self, iter: T) {
        for set in iter {
            self.is_under_max &= set.red <= RED && set.blue <= BLUE && set.green <= GREEN
        }
    }
}

#[derive(Debug, Default)]
struct GamesUnderMaxCounter<const RED: u32, const BLUE: u32, const GREEN: u32> {
    sum: u32,
}

impl<const RED: u32, const BLUE: u32, const GREEN: u32>
    Extend<(u32, IsGameUnderMax<RED, GREEN, BLUE>)> for GamesUnderMaxCounter<RED, BLUE, GREEN>
{
    fn extend<T: IntoIterator<Item = (u32, IsGameUnderMax<RED, GREEN, BLUE>)>>(&mut self, iter: T) {
        self.sum += iter
            .into_iter()
            .filter_map(|(id, is_under)| is_under.is_under_max.then_some(id))
            .sum::<u32>()
    }
}

#[derive(Debug, Default)]
struct GamePowerCounter {
    sum: u32,
}

impl Extend<(u32, CubeSet)> for GamePowerCounter {
    fn extend<T: IntoIterator<Item = (u32, CubeSet)>>(&mut self, iter: T) {
        self.sum += iter
            .into_iter()
            .map(|(_, CubeSet { red, green, blue })| red * green * blue)
            .sum::<u32>()
    }
}

impl<const RED: u32, const BLUE: u32, const GREEN: u32> Default
    for IsGameUnderMax<RED, BLUE, GREEN>
{
    fn default() -> Self {
        Self { is_under_max: true }
    }
}

fn parse_cube_set(input: &str) -> IResult<&str, CubeSet, ErrorTree<&str>> {
    let parse_color = tuple((
        u32,
        space1,
        alt((
            tag("red").value(Color::Red),
            tag("green").value(Color::Green),
            tag("blue").value(Color::Blue),
        )),
    ))
    .map(|(count, _, color)| (count, color))
    .context("color count");
    collect_separated_terminated(
        parse_color,
        tag(", "),
        tag("; ").or(alt((tag("\n"), eof)).peek()),
    )
    .context("set of cubes")
    .parse(input)
}

fn parse_game<C>(input: &str) -> IResult<&str, (u32, C), ErrorTree<&str>>
where
    C: Default + Extend<CubeSet>,
{
    tuple((
        u32.preceded_by(tag("Game "))
            .terminated(tag(": "))
            .context("Game ID"),
        collect_separated_terminated(parse_cube_set, success(()), tag("\n").or(eof)),
    ))
    .context("Game")
    .parse(input)
}

fn parse_input<C1, C2>(input: &str) -> Result<C1, ErrorTree<Location>>
where
    C1: Default + Extend<(u32, C2)>,
    C2: Default + Extend<CubeSet>,
{
    final_parser(collect_separated_terminated(parse_game, success(()), eof))(input)
}

pub fn level1(input: &str) -> u32 {
    let result: GamesUnderMaxCounter<12, 13, 14> = parse_input(input).expect("parse error");
    result.sum
}

pub fn level2(input: &str) -> u32 {
    let result: GamePowerCounter = parse_input(input).expect("parse error");
    result.sum
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
