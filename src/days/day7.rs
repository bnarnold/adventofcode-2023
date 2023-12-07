use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    character::complete::{anychar, newline, space1, u32},
    combinator::{eof, map_opt, success},
    sequence::separated_pair,
    Parser,
};
use nom_supreme::{
    final_parser::final_parser, multi::collect_separated_terminated, tag::complete::tag, ParserExt,
};

use crate::util::prelude::*;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Clone, Copy)]
enum Card {
    Joker,
    Number(u8),
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    card_counts: Vec<usize>,
    cards: [Card; 5],
    bid: u32,
}

impl Hand {
    fn new(cards: [Card; 5], bid: u32) -> Self {
        let mut card_counts = cards.iter().counts().into_values().collect_vec();
        card_counts.sort_by(|a, b| a.cmp(b).reverse());
        Self {
            cards,
            bid,
            card_counts,
        }
    }

    fn new_with_jokers(cards: [Card; 5], bid: u32) -> Self {
        let mut card_counts = cards.iter().counts();
        let joker_count = card_counts.remove(&Card::Joker);
        let mut card_counts = card_counts.into_values().collect_vec();
        card_counts.sort_by(|a, b| a.cmp(b).reverse());
        if let Some(joker_count) = joker_count {
            if let Some(max_count) = card_counts.first_mut() {
                *max_count += joker_count
            } else {
                card_counts.push(joker_count)
            }
        }
        Self {
            cards,
            bid,
            card_counts,
        }
    }
}

fn parse_card_with_jokers(input: &str) -> ParseResult<Card> {
    map_opt(anychar, |c| {
        c.to_digit(10)
            .filter(|x| (2..=9).contains(x))
            .map(|x| Card::Number(x as u8))
    })
    .context("Number card")
    .or(alt((
        tag("T").value(Card::Ten),
        tag("J").value(Card::Joker),
        tag("Q").value(Card::Queen),
        tag("K").value(Card::King),
        tag("A").value(Card::Ace),
    ))
    .context("Face card"))
    .parse(input)
}

fn parse_hand_with_jokers(input: &str) -> ParseResult<Hand> {
    separated_pair(
        parse_card_with_jokers.separated_array(success(())),
        space1,
        u32,
    )
    .map(|(cards, bid)| Hand::new_with_jokers(cards, bid))
    .terminated(newline)
    .context("Hand")
    .parse(input)
}

fn parse_input_with_jokers(input: &str) -> ParseFinalResult<Vec<Hand>> {
    final_parser(collect_separated_terminated(
        parse_hand_with_jokers,
        success(()),
        eof,
    ))(input)
}

fn parse_card(input: &str) -> ParseResult<Card> {
    map_opt(anychar, |c| {
        c.to_digit(10)
            .filter(|x| (2..=9).contains(x))
            .map(|x| Card::Number(x as u8))
    })
    .context("Number card")
    .or(alt((
        tag("T").value(Card::Ten),
        tag("J").value(Card::Jack),
        tag("Q").value(Card::Queen),
        tag("K").value(Card::King),
        tag("A").value(Card::Ace),
    ))
    .context("Face card"))
    .parse(input)
}

fn parse_hand(input: &str) -> ParseResult<Hand> {
    separated_pair(parse_card.separated_array(success(())), space1, u32)
        .map(|(cards, bid)| Hand::new(cards, bid))
        .terminated(newline)
        .context("Hand")
        .parse(input)
}

fn parse_input(input: &str) -> ParseFinalResult<Vec<Hand>> {
    final_parser(collect_separated_terminated(parse_hand, success(()), eof))(input)
}

pub fn level1(input: &str) -> u32 {
    let mut hands = parse_input(input).expect("parse error");
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum()
}

pub fn level2(input: &str) -> u32 {
    let mut hands = parse_input_with_jokers(input).expect("parse error");
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .map(|(i, hand)| (i as u32 + 1) * hand.bid)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn level1_given_example() {
        let test_input = include_str!("./test_input/day7.txt");
        assert_eq!(level1(test_input), 6440)
    }

    #[test]
    fn level2_given_example() {
        let test_input = include_str!("./test_input/day7.txt");
        assert_eq!(level2(test_input), 5905)
    }
}
